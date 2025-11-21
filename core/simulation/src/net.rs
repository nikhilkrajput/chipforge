//! Network and signal representation

use crate::value::Value;
use std::collections::HashMap;

/// Unique identifier for a net/signal
pub type NetId = usize;

/// A signal/net in the design
#[derive(Debug, Clone)]
pub struct Net {
    /// Unique ID
    pub id: NetId,
    /// Hierarchical name (e.g., "top.cpu.alu.result")
    pub name: String,
    /// Current value
    pub value: Value,
    /// Width in bits
    pub width: usize,
    /// Sensitivity list - processes that depend on this net
    pub sensitive_processes: Vec<usize>,
}

impl Net {
    /// Create a new net
    pub fn new(id: NetId, name: String, width: usize, initial_value: Value) -> Self {
        Self {
            id,
            name,
            value: initial_value,
            width,
            sensitive_processes: Vec::new(),
        }
    }

    /// Add a process to sensitivity list
    pub fn add_sensitive_process(&mut self, process_id: usize) {
        if !self.sensitive_processes.contains(&process_id) {
            self.sensitive_processes.push(process_id);
        }
    }
}

/// Network database - stores all signals
#[derive(Debug)]
pub struct NetDatabase {
    /// All nets indexed by ID
    nets: Vec<Net>,
    /// Name to ID mapping
    name_map: HashMap<String, NetId>,
    /// Next available ID
    next_id: NetId,
}

impl NetDatabase {
    /// Create a new network database
    pub fn new() -> Self {
        Self {
            nets: Vec::new(),
            name_map: HashMap::new(),
            next_id: 0,
        }
    }

    /// Create a new net
    pub fn create_net(&mut self, name: String, width: usize, initial_value: Value) -> NetId {
        let id = self.next_id;
        self.next_id += 1;

        let net = Net::new(id, name.clone(), width, initial_value);
        self.nets.push(net);
        self.name_map.insert(name, id);

        id
    }

    /// Get net by ID
    pub fn get_net(&self, id: NetId) -> Option<&Net> {
        self.nets.get(id)
    }

    /// Get mutable net by ID
    pub fn get_net_mut(&mut self, id: NetId) -> Option<&mut Net> {
        self.nets.get_mut(id)
    }

    /// Get net ID by name
    pub fn get_net_id(&self, name: &str) -> Option<NetId> {
        self.name_map.get(name).copied()
    }

    /// Get net value
    pub fn get_value(&self, id: NetId) -> Option<&Value> {
        self.get_net(id).map(|net| &net.value)
    }

    /// Set net value (returns true if value changed)
    pub fn set_value(&mut self, id: NetId, value: Value) -> bool {
        if let Some(net) = self.get_net_mut(id) {
            if net.value != value {
                net.value = value;
                return true;
            }
        }
        false
    }

    /// Get all nets
    pub fn nets(&self) -> &[Net] {
        &self.nets
    }

    /// Get number of nets
    pub fn len(&self) -> usize {
        self.nets.len()
    }

    /// Check if database is empty
    pub fn is_empty(&self) -> bool {
        self.nets.is_empty()
    }
}

impl Default for NetDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::BitValue;

    #[test]
    fn test_net_creation() {
        let mut db = NetDatabase::new();

        let id = db.create_net(
            "top.clk".to_string(),
            1,
            Value::Bit(BitValue::Zero),
        );

        assert_eq!(id, 0);
        assert_eq!(db.len(), 1);

        let net = db.get_net(id).unwrap();
        assert_eq!(net.name, "top.clk");
        assert_eq!(net.width, 1);
    }

    #[test]
    fn test_net_lookup() {
        let mut db = NetDatabase::new();

        db.create_net("top.clk".to_string(), 1, Value::Bit(BitValue::Zero));
        db.create_net("top.rst".to_string(), 1, Value::Bit(BitValue::One));

        let clk_id = db.get_net_id("top.clk").unwrap();
        let rst_id = db.get_net_id("top.rst").unwrap();

        assert_ne!(clk_id, rst_id);
        assert_eq!(db.get_net(clk_id).unwrap().name, "top.clk");
        assert_eq!(db.get_net(rst_id).unwrap().name, "top.rst");
    }

    #[test]
    fn test_net_value_update() {
        let mut db = NetDatabase::new();

        let id = db.create_net(
            "top.signal".to_string(),
            1,
            Value::Bit(BitValue::Zero),
        );

        // Value should change
        assert!(db.set_value(id, Value::Bit(BitValue::One)));

        // Same value, no change
        assert!(!db.set_value(id, Value::Bit(BitValue::One)));

        // Different value again
        assert!(db.set_value(id, Value::Bit(BitValue::Zero)));
    }

    #[test]
    fn test_sensitivity_list() {
        let mut db = NetDatabase::new();

        let id = db.create_net(
            "top.clk".to_string(),
            1,
            Value::Bit(BitValue::Zero),
        );

        let net = db.get_net_mut(id).unwrap();
        net.add_sensitive_process(0);
        net.add_sensitive_process(1);
        net.add_sensitive_process(0); // Duplicate, shouldn't be added

        assert_eq!(net.sensitive_processes.len(), 2);
        assert_eq!(net.sensitive_processes, vec![0, 1]);
    }
}
