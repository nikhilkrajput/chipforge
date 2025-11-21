//! Event-driven HDL simulation engine for ChipForge
//!
//! This crate implements a fast, event-driven simulation engine for digital designs.
//! It supports:
//! - Event-driven simulation with delta cycles
//! - 4-state logic (0, 1, X, Z)
//! - VCD waveform generation
//! - Hierarchical designs
//! - Combinational and sequential logic
//!
//! # Example
//!
//! ```rust
//! use chipforge_simulation::Simulator;
//!
//! let mut sim = Simulator::new();
//! // Add design, run simulation
//! sim.run_until(1000); // Run for 1000 time units
//! ```

pub mod event;
pub mod kernel;
pub mod value;
pub mod vcd;
pub mod net;
pub mod process;

use chipforge_common::Result;

pub use event::{Event, EventQueue, SimTime};
pub use kernel::SimulationKernel;
pub use value::{BitValue, LogicValue, Value};
pub use vcd::VcdWriter;

/// Main simulator interface
pub struct Simulator {
    kernel: SimulationKernel,
    current_time: SimTime,
    config: SimulatorConfig,
}

/// Simulation configuration
#[derive(Debug, Clone)]
pub struct SimulatorConfig {
    /// Maximum simulation time
    pub max_time: Option<SimTime>,
    /// Enable VCD waveform dumping
    pub dump_vcd: bool,
    /// VCD output file path
    pub vcd_file: Option<String>,
    /// Time resolution (in picoseconds)
    pub time_resolution: u64,
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self {
            max_time: None,
            dump_vcd: false,
            vcd_file: None,
            time_resolution: 1, // 1ps
        }
    }
}

impl Simulator {
    /// Create a new simulator with default configuration
    pub fn new() -> Self {
        Self::with_config(SimulatorConfig::default())
    }

    /// Create a new simulator with custom configuration
    pub fn with_config(config: SimulatorConfig) -> Self {
        Self {
            kernel: SimulationKernel::new(),
            current_time: 0,
            config,
        }
    }

    /// Get current simulation time
    pub fn current_time(&self) -> SimTime {
        self.current_time
    }

    /// Run simulation until specified time
    pub fn run_until(&mut self, time: SimTime) -> Result<()> {
        tracing::info!("Starting simulation until time {}", time);

        while self.current_time < time {
            if let Some(max_time) = self.config.max_time {
                if self.current_time >= max_time {
                    tracing::info!("Reached maximum simulation time");
                    break;
                }
            }

            // Process all events at current time
            if !self.kernel.process_time_step(&mut self.current_time)? {
                tracing::info!("No more events, simulation complete at time {}", self.current_time);
                break;
            }
        }

        tracing::info!("Simulation completed at time {}", self.current_time);
        Ok(())
    }

    /// Run one time step
    pub fn step(&mut self) -> Result<bool> {
        self.kernel.process_time_step(&mut self.current_time)
    }

    /// Add an event to the simulation queue
    pub fn schedule_event(&mut self, time: SimTime, event: Event) {
        self.kernel.schedule_event(time, event);
    }

    /// Get reference to simulation kernel (for advanced use)
    pub fn kernel(&self) -> &SimulationKernel {
        &self.kernel
    }

    /// Get mutable reference to simulation kernel (for advanced use)
    pub fn kernel_mut(&mut self) -> &mut SimulationKernel {
        &mut self.kernel
    }

    /// Reset simulation to time 0
    pub fn reset(&mut self) {
        self.current_time = 0;
        self.kernel = SimulationKernel::new();
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulator_creation() {
        let sim = Simulator::new();
        assert_eq!(sim.current_time(), 0);
    }

    #[test]
    fn test_simulator_with_config() {
        let config = SimulatorConfig {
            max_time: Some(1000),
            dump_vcd: true,
            vcd_file: Some("test.vcd".to_string()),
            time_resolution: 1000, // 1ns
        };
        let sim = Simulator::with_config(config);
        assert_eq!(sim.current_time(), 0);
    }

    #[test]
    fn test_simulator_reset() {
        let mut sim = Simulator::new();
        sim.current_time = 100;
        sim.reset();
        assert_eq!(sim.current_time(), 0);
    }
}
