//! Process representation and execution

use crate::net::NetId;
use crate::value::Value;

/// Unique identifier for a process
pub type ProcessId = usize;

/// Process type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessType {
    /// Combinational (always @*)
    Combinational,
    /// Sequential (always @(posedge clk))
    Sequential,
    /// Initial block
    Initial,
    /// Continuous assignment
    Assign,
}

/// Edge sensitivity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    /// Positive edge (rising)
    Posedge,
    /// Negative edge (falling)
    Negedge,
    /// Any change
    AnyEdge,
}

/// Sensitivity specification
#[derive(Debug, Clone)]
pub enum Sensitivity {
    /// Level-sensitive (any change)
    Level(Vec<NetId>),
    /// Edge-sensitive
    Edge(Vec<(NetId, Edge)>),
    /// All signals (*)
    All,
}

/// A simulation process (always block, initial block, etc.)
#[derive(Debug, Clone)]
pub struct Process {
    /// Unique ID
    pub id: ProcessId,
    /// Process type
    pub process_type: ProcessType,
    /// Sensitivity list
    pub sensitivity: Sensitivity,
    /// Is this process currently active?
    pub is_active: bool,
}

impl Process {
    /// Create a new process
    pub fn new(id: ProcessId, process_type: ProcessType, sensitivity: Sensitivity) -> Self {
        Self {
            id,
            process_type,
            sensitivity,
            is_active: false,
        }
    }

    /// Check if process is sensitive to a signal change
    pub fn is_sensitive_to(&self, net_id: NetId, old_value: &Value, new_value: &Value) -> bool {
        match &self.sensitivity {
            Sensitivity::All => true,
            Sensitivity::Level(nets) => nets.contains(&net_id),
            Sensitivity::Edge(edges) => {
                for (id, edge) in edges {
                    if *id == net_id {
                        return self.check_edge(edge, old_value, new_value);
                    }
                }
                false
            }
        }
    }

    /// Check if value change matches edge type
    fn check_edge(&self, edge: &Edge, old_value: &Value, new_value: &Value) -> bool {
        let old_bool = old_value.to_bool();
        let new_bool = new_value.to_bool();

        match edge {
            Edge::Posedge => !old_bool && new_bool, // 0 -> 1
            Edge::Negedge => old_bool && !new_bool, // 1 -> 0
            Edge::AnyEdge => old_bool != new_bool,  // any change
        }
    }
}

/// Process database
#[derive(Debug)]
pub struct ProcessDatabase {
    /// All processes
    processes: Vec<Process>,
    /// Next available ID
    next_id: ProcessId,
}

impl ProcessDatabase {
    /// Create a new process database
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            next_id: 0,
        }
    }

    /// Create a new process
    pub fn create_process(&mut self, process_type: ProcessType, sensitivity: Sensitivity) -> ProcessId {
        let id = self.next_id;
        self.next_id += 1;

        let process = Process::new(id, process_type, sensitivity);
        self.processes.push(process);

        id
    }

    /// Get process by ID
    pub fn get_process(&self, id: ProcessId) -> Option<&Process> {
        self.processes.get(id)
    }

    /// Get mutable process by ID
    pub fn get_process_mut(&mut self, id: ProcessId) -> Option<&mut Process> {
        self.processes.get_mut(id)
    }

    /// Get all processes
    pub fn processes(&self) -> &[Process] {
        &self.processes
    }

    /// Get all active processes
    pub fn active_processes(&self) -> Vec<ProcessId> {
        self.processes
            .iter()
            .filter(|p| p.is_active)
            .map(|p| p.id)
            .collect()
    }

    /// Set process active state
    pub fn set_active(&mut self, id: ProcessId, active: bool) {
        if let Some(process) = self.get_process_mut(id) {
            process.is_active = active;
        }
    }

    /// Get number of processes
    pub fn len(&self) -> usize {
        self.processes.len()
    }

    /// Check if database is empty
    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }
}

impl Default for ProcessDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::BitValue;

    #[test]
    fn test_process_creation() {
        let mut db = ProcessDatabase::new();

        let id = db.create_process(
            ProcessType::Combinational,
            Sensitivity::Level(vec![0, 1]),
        );

        assert_eq!(id, 0);
        assert_eq!(db.len(), 1);

        let process = db.get_process(id).unwrap();
        assert_eq!(process.process_type, ProcessType::Combinational);
        assert!(!process.is_active);
    }

    #[test]
    fn test_edge_detection() {
        let process = Process::new(
            0,
            ProcessType::Sequential,
            Sensitivity::Edge(vec![(0, Edge::Posedge)]),
        );

        let zero = Value::Bit(BitValue::Zero);
        let one = Value::Bit(BitValue::One);

        // Rising edge: 0 -> 1
        assert!(process.is_sensitive_to(0, &zero, &one));

        // Not a rising edge: 1 -> 1
        assert!(!process.is_sensitive_to(0, &one, &one));

        // Falling edge: 1 -> 0 (not sensitive to this)
        assert!(!process.is_sensitive_to(0, &one, &zero));
    }

    #[test]
    fn test_level_sensitivity() {
        let process = Process::new(
            0,
            ProcessType::Combinational,
            Sensitivity::Level(vec![0, 1, 2]),
        );

        let zero = Value::Bit(BitValue::Zero);
        let one = Value::Bit(BitValue::One);

        // Sensitive to signals 0, 1, 2
        assert!(process.is_sensitive_to(0, &zero, &one));
        assert!(process.is_sensitive_to(1, &zero, &one));
        assert!(process.is_sensitive_to(2, &zero, &one));

        // Not sensitive to signal 3
        assert!(!process.is_sensitive_to(3, &zero, &one));
    }

    #[test]
    fn test_process_activation() {
        let mut db = ProcessDatabase::new();

        let id = db.create_process(
            ProcessType::Combinational,
            Sensitivity::All,
        );

        assert!(!db.get_process(id).unwrap().is_active);

        db.set_active(id, true);
        assert!(db.get_process(id).unwrap().is_active);

        let active = db.active_processes();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0], id);
    }
}
