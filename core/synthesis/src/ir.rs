//! Intermediate representation for hardware designs

use std::collections::HashMap;

/// Hardware design representation
#[derive(Debug, Clone)]
pub struct Design {
    /// Modules in the design
    pub modules: HashMap<String, Module>,
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
        self.modules.insert(module.name.clone(), module);
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
    pub name: String,
    /// Module parameters
    pub parameters: Vec<Parameter>,
    /// Input ports
    pub inputs: Vec<Port>,
    /// Output ports
    pub outputs: Vec<Port>,
    /// Internal signals
    pub signals: Vec<Signal>,
    /// Module instances
    pub instances: Vec<Instance>,
    /// Continuous assignments
    pub assignments: Vec<Assignment>,
    /// Always blocks
    pub always_blocks: Vec<AlwaysBlock>,
}

/// Module parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Default value (if any)
    pub default_value: Option<i64>,
}

/// Port declaration
#[derive(Debug, Clone)]
pub struct Port {
    /// Port name
    pub name: String,
    /// Port width
    pub width: usize,
}

/// Internal signal
#[derive(Debug, Clone)]
pub struct Signal {
    /// Signal name
    pub name: String,
    /// Signal width
    pub width: usize,
}

/// Module instance
#[derive(Debug, Clone)]
pub struct Instance {
    /// Instance name
    pub name: String,
    /// Module being instantiated
    pub module_name: String,
    /// Parameter overrides
    pub parameters: HashMap<String, i64>,
    /// Port connections
    pub connections: HashMap<String, Connection>,
}

/// Port connection
#[derive(Debug, Clone)]
pub enum Connection {
    /// Direct signal connection
    Signal(String),
    /// Constant value
    Constant(i64),
    /// Expression (for more complex connections)
    Expression(String),
}

/// Continuous assignment (assign statement)
#[derive(Debug, Clone)]
pub struct Assignment {
    /// Target signal
    pub target: String,
    /// Source expression
    pub source: String,
}

/// Always block
#[derive(Debug, Clone)]
pub struct AlwaysBlock {
    /// Sensitivity list type
    pub sensitivity: SensitivityType,
    /// Block statements (simplified as string for now)
    pub body: String,
}

/// Sensitivity list type
#[derive(Debug, Clone)]
pub enum SensitivityType {
    /// Combinational (@*)
    Combinational,
    /// Posedge clock
    Posedge(String),
    /// Negedge clock
    Negedge(String),
    /// Both edges
    BothEdges(String),
    /// Custom sensitivity list
    Custom(Vec<String>),
}
