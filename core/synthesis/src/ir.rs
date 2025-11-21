//! Intermediate representation for hardware designs

use chipforge_common::identifier::Identifier;
use std::collections::HashMap;

/// Hardware design representation
#[derive(Debug, Clone)]
pub struct Design {
    /// Modules in the design
    pub modules: HashMap<Identifier, Module>,
}

impl Design {
    /// Create a new empty design
    pub fn new() -> Self {
        Design {
            modules: HashMap::new(),
        }
    }

    /// Add a module to the design
    pub fn add_module(&mut self, module: Module) {
        self.modules.insert(module.name, module);
    }
}

impl Default for Design {
    fn default() -> Self {
        Self::new()
    }
}

/// Hardware module
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    pub name: Identifier,
    /// Input ports
    pub inputs: Vec<Port>,
    /// Output ports
    pub outputs: Vec<Port>,
    /// Internal signals
    pub signals: Vec<Signal>,
}

/// Port declaration
#[derive(Debug, Clone)]
pub struct Port {
    /// Port name
    pub name: Identifier,
    /// Port width
    pub width: usize,
}

/// Internal signal
#[derive(Debug, Clone)]
pub struct Signal {
    /// Signal name
    pub name: Identifier,
    /// Signal width
    pub width: usize,
}
