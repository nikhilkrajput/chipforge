//! Simulation kernel - the heart of the event-driven simulator

use crate::event::{Event, EventQueue, SimTime};
use crate::net::{NetDatabase, NetId};
use crate::process::{ProcessDatabase, ProcessId};
use crate::value::Value;
use chipforge_common::Result;

/// Simulation kernel
#[derive(Debug)]
pub struct SimulationKernel {
    /// Network database (all signals)
    pub nets: NetDatabase,
    /// Process database (all processes)
    pub processes: ProcessDatabase,
    /// Event queue
    event_queue: EventQueue,
    /// Current delta cycle
    delta_cycle: usize,
    /// Statistics
    stats: SimulationStats,
}

/// Simulation statistics
#[derive(Debug, Clone, Default)]
pub struct SimulationStats {
    /// Total events processed
    pub events_processed: usize,
    /// Total delta cycles
    pub delta_cycles: usize,
    /// Total time steps
    pub time_steps: usize,
    /// Maximum delta cycles in one time step
    pub max_delta_cycles: usize,
}

impl SimulationKernel {
    /// Create a new simulation kernel
    pub fn new() -> Self {
        Self {
            nets: NetDatabase::new(),
            processes: ProcessDatabase::new(),
            event_queue: EventQueue::new(),
            delta_cycle: 0,
            stats: SimulationStats::default(),
        }
    }

    /// Create a new net
    pub fn create_net(&mut self, name: String, width: usize, initial_value: Value) -> NetId {
        self.nets.create_net(name, width, initial_value)
    }

    /// Create a new process
    pub fn create_process(
        &mut self,
        process_type: crate::process::ProcessType,
        sensitivity: crate::process::Sensitivity,
    ) -> ProcessId {
        self.processes.create_process(process_type, sensitivity)
    }

    /// Schedule an event
    pub fn schedule_event(&mut self, time: SimTime, event: Event) {
        self.event_queue.schedule(event);
    }

    /// Get net value
    pub fn get_net_value(&self, id: NetId) -> Option<&Value> {
        self.nets.get_value(id)
    }

    /// Set net value (immediately, schedules events for sensitive processes)
    pub fn set_net_value(&mut self, id: NetId, value: Value, current_time: SimTime) -> Result<()> {
        if let Some(old_value) = self.nets.get_value(id).cloned() {
            // Only process if value changed
            if self.nets.set_value(id, value) {
                // Find all processes sensitive to this net
                let sensitive_processes = if let Some(net) = self.nets.get_net(id) {
                    net.sensitive_processes.clone()
                } else {
                    Vec::new()
                };

                // Trigger sensitive processes
                for process_id in sensitive_processes {
                    if let Some(process) = self.processes.get_process(process_id) {
                        let new_value = self.nets.get_value(id).unwrap();
                        if process.is_sensitive_to(id, &old_value, new_value) {
                            self.processes.set_active(process_id, true);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Process one time step (advances time to next event)
    pub fn process_time_step(&mut self, current_time: &mut SimTime) -> Result<bool> {
        // Get next event time
        if let Some(event) = self.event_queue.peek() {
            let next_time = event.time;

            // Advance time
            *current_time = next_time;
            self.stats.time_steps += 1;

            // Process all events at this time (delta cycle processing)
            self.process_delta_cycles(next_time)?;

            Ok(true)
        } else {
            // No more events
            Ok(false)
        }
    }

    /// Process delta cycles at current time
    fn process_delta_cycles(&mut self, time: SimTime) -> Result<()> {
        let mut delta_count = 0;
        const MAX_DELTA_CYCLES: usize = 10000; // Prevent infinite loops

        loop {
            delta_count += 1;
            self.delta_cycle = delta_count;

            if delta_count > MAX_DELTA_CYCLES {
                return Err(chipforge_common::Error::Other(format!(
                    "Maximum delta cycles ({}) exceeded at time {} - possible combinational loop",
                    MAX_DELTA_CYCLES, time
                )));
            }

            // Get all events at current time
            let events = self.event_queue.get_events_at_time(time);

            if events.is_empty() {
                break;
            }

            // Process events
            for event in events {
                self.process_event(event, time)?;
                self.stats.events_processed += 1;
            }

            // Check if any processes were activated
            let active_processes = self.processes.active_processes();
            if active_processes.is_empty() {
                break;
            }

            // Execute active processes (would call evaluation functions here)
            for process_id in active_processes {
                self.processes.set_active(process_id, false);
                // In a full implementation, we would execute the process body here
                // For now, we just clear the active flag
            }
        }

        self.stats.delta_cycles += delta_count;
        if delta_count > self.stats.max_delta_cycles {
            self.stats.max_delta_cycles = delta_count;
        }

        Ok(())
    }

    /// Process a single event
    fn process_event(&mut self, event: Event, current_time: SimTime) -> Result<()> {
        tracing::trace!(
            "Processing event at time {}: net {} <- {}",
            current_time,
            event.target,
            event.value
        );

        self.set_net_value(event.target, event.value, current_time)?;

        Ok(())
    }

    /// Get simulation statistics
    pub fn stats(&self) -> &SimulationStats {
        &self.stats
    }

    /// Print statistics summary
    pub fn print_stats(&self) {
        println!("Simulation Statistics:");
        println!("  Events processed: {}", self.stats.events_processed);
        println!("  Time steps: {}", self.stats.time_steps);
        println!("  Delta cycles: {}", self.stats.delta_cycles);
        println!("  Max delta/step: {}", self.stats.max_delta_cycles);
        println!("  Nets: {}", self.nets.len());
        println!("  Processes: {}", self.processes.len());
    }
}

impl Default for SimulationKernel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::{ProcessType, Sensitivity};
    use crate::value::BitValue;

    #[test]
    fn test_kernel_creation() {
        let kernel = SimulationKernel::new();
        assert_eq!(kernel.nets.len(), 0);
        assert_eq!(kernel.processes.len(), 0);
    }

    #[test]
    fn test_net_and_process_creation() {
        let mut kernel = SimulationKernel::new();

        let net_id = kernel.create_net(
            "test_signal".to_string(),
            1,
            Value::Bit(BitValue::Zero),
        );
        assert_eq!(net_id, 0);

        let process_id = kernel.create_process(
            ProcessType::Combinational,
            Sensitivity::Level(vec![net_id]),
        );
        assert_eq!(process_id, 0);

        assert_eq!(kernel.nets.len(), 1);
        assert_eq!(kernel.processes.len(), 1);
    }

    #[test]
    fn test_event_scheduling() {
        let mut kernel = SimulationKernel::new();

        let net_id = kernel.create_net(
            "clk".to_string(),
            1,
            Value::Bit(BitValue::Zero),
        );

        let event = Event::new(100, net_id, Value::Bit(BitValue::One));
        kernel.schedule_event(100, event);

        assert!(!kernel.event_queue.is_empty());
    }

    #[test]
    fn test_simple_simulation() -> Result<()> {
        let mut kernel = SimulationKernel::new();
        let mut time = 0;

        // Create a signal
        let net_id = kernel.create_net(
            "signal".to_string(),
            1,
            Value::Bit(BitValue::Zero),
        );

        // Schedule some events
        kernel.schedule_event(
            10,
            Event::new(10, net_id, Value::Bit(BitValue::One)),
        );
        kernel.schedule_event(
            20,
            Event::new(20, net_id, Value::Bit(BitValue::Zero)),
        );

        // Run simulation
        kernel.process_time_step(&mut time)?;
        assert_eq!(time, 10);
        assert_eq!(
            kernel.get_net_value(net_id),
            Some(&Value::Bit(BitValue::One))
        );

        kernel.process_time_step(&mut time)?;
        assert_eq!(time, 20);
        assert_eq!(
            kernel.get_net_value(net_id),
            Some(&Value::Bit(BitValue::Zero))
        );

        Ok(())
    }

    #[test]
    fn test_simulation_stats() -> Result<()> {
        let mut kernel = SimulationKernel::new();
        let mut time = 0;

        let net_id = kernel.create_net(
            "signal".to_string(),
            1,
            Value::Bit(BitValue::Zero),
        );

        // Schedule multiple events
        for i in 1..=10 {
            kernel.schedule_event(
                i * 10,
                Event::new(i * 10, net_id, Value::Bit(BitValue::One)),
            );
        }

        // Run simulation
        for _ in 0..10 {
            kernel.process_time_step(&mut time)?;
        }

        let stats = kernel.stats();
        assert_eq!(stats.time_steps, 10);
        assert_eq!(stats.events_processed, 10);

        Ok(())
    }
}
