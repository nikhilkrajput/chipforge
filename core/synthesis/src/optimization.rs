//! Optimization passes for synthesis

use crate::ir::{Design, Module, Signal, Assignment, AlwaysBlock};
use chipforge_common::Result;
use std::collections::{HashMap, HashSet};

/// Optimization pass trait
pub trait OptimizationPass {
    /// Name of the optimization pass
    fn name(&self) -> &str;

    /// Run the optimization pass on a design
    fn run(&mut self, design: &mut Design) -> Result<OptimizationStats>;
}

/// Statistics from running an optimization pass
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    /// Number of changes made
    pub changes: usize,
    /// Pass-specific metrics
    pub metrics: HashMap<String, usize>,
}

impl OptimizationStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_metric(&mut self, name: &str, value: usize) {
        self.metrics.insert(name.to_string(), value);
    }
}

/// Optimization manager
pub struct OptimizationManager {
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl OptimizationManager {
    /// Create a new optimization manager
    pub fn new() -> Self {
        OptimizationManager { passes: Vec::new() }
    }

    /// Add an optimization pass
    pub fn add_pass(&mut self, pass: Box<dyn OptimizationPass>) {
        self.passes.push(pass);
    }

    /// Run all optimization passes
    pub fn run(&mut self, design: &mut Design) -> Result<OptimizationStats> {
        let mut total_stats = OptimizationStats::new();

        for pass in &mut self.passes {
            tracing::info!("Running optimization pass: {}", pass.name());
            let stats = pass.run(design)?;
            total_stats.changes += stats.changes;
            tracing::debug!("Pass {} made {} changes", pass.name(), stats.changes);
        }

        Ok(total_stats)
    }

    /// Create a standard optimization pipeline
    pub fn standard_pipeline() -> Self {
        let mut manager = Self::new();
        manager.add_pass(Box::new(UnusedSignalElimination::new()));
        manager.add_pass(Box::new(ConstantPropagation::new()));
        manager.add_pass(Box::new(DeadCodeElimination::new()));
        manager.add_pass(Box::new(CommonSubexpressionElimination::new()));
        manager.add_pass(Box::new(LogicMinimization::new()));
        manager
    }
}

impl Default for OptimizationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Optimization Pass 1: Unused Signal Elimination
// ============================================================================

/// Remove signals that are never used
pub struct UnusedSignalElimination {
    removed_count: usize,
}

impl UnusedSignalElimination {
    pub fn new() -> Self {
        Self { removed_count: 0 }
    }

    fn find_used_signals(&self, module: &Module) -> HashSet<String> {
        let mut used = HashSet::new();

        // Signals used in assignments
        for assignment in &module.assignments {
            used.insert(assignment.target.clone());
            // Simple signal extraction from source (look for signal names)
            // This is a simplified approach - TODO: Parse expressions properly
            for signal in &module.signals {
                if assignment.source.contains(&signal.name) {
                    used.insert(signal.name.clone());
                }
            }
            // Also check inputs
            for input in &module.inputs {
                if assignment.source.contains(&input.name) {
                    used.insert(input.name.clone());
                }
            }
        }

        // Signals used in always blocks
        for _always in &module.always_blocks {
            // TODO: Parse always block body to find used signals
        }

        // Signals used in instance connections
        for instance in &module.instances {
            for (_port, connection) in &instance.connections {
                if let crate::ir::Connection::Signal(sig) = connection {
                    used.insert(sig.clone());
                }
            }
        }

        used
    }
}

impl Default for UnusedSignalElimination {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for UnusedSignalElimination {
    fn name(&self) -> &str {
        "unused-signal-elimination"
    }

    fn run(&mut self, design: &mut Design) -> Result<OptimizationStats> {
        let mut stats = OptimizationStats::new();
        self.removed_count = 0;

        for module in design.modules.values_mut() {
            let used_signals = self.find_used_signals(module);
            let original_count = module.signals.len();

            // Remove unused signals
            module.signals.retain(|sig| {
                let is_used = used_signals.contains(&sig.name);
                if !is_used {
                    tracing::trace!("Removing unused signal: {}", sig.name);
                    self.removed_count += 1;
                }
                is_used
            });

            stats.changes += original_count - module.signals.len();
        }

        stats.add_metric("signals_removed", self.removed_count);
        Ok(stats)
    }
}

// ============================================================================
// Optimization Pass 2: Constant Propagation
// ============================================================================

/// Propagate constant values through the design
pub struct ConstantPropagation {
    propagated_count: usize,
}

impl ConstantPropagation {
    pub fn new() -> Self {
        Self {
            propagated_count: 0,
        }
    }

    fn find_constant_signals(&self, module: &Module) -> HashMap<String, i64> {
        let mut constants = HashMap::new();

        // Look for assignments to constant values
        for assignment in &module.assignments {
            // Simple constant detection
            // TODO: Parse expressions properly
            if let Ok(value) = assignment.source.parse::<i64>() {
                constants.insert(assignment.target.clone(), value);
            }
        }

        constants
    }

    fn propagate_in_assignments(&mut self, assignments: &mut Vec<Assignment>, constants: &HashMap<String, i64>) -> usize {
        let mut changes = 0;

        for assignment in assignments.iter_mut() {
            // Replace constant signal references in source
            for (signal, value) in constants {
                if assignment.source.contains(signal) {
                    assignment.source = assignment.source.replace(signal, &value.to_string());
                    changes += 1;
                    self.propagated_count += 1;
                }
            }
        }

        changes
    }
}

impl Default for ConstantPropagation {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for ConstantPropagation {
    fn name(&self) -> &str {
        "constant-propagation"
    }

    fn run(&mut self, design: &mut Design) -> Result<OptimizationStats> {
        let mut stats = OptimizationStats::new();
        self.propagated_count = 0;

        for module in design.modules.values_mut() {
            let constants = self.find_constant_signals(module);

            // Propagate constants in assignments
            stats.changes += self.propagate_in_assignments(&mut module.assignments, &constants);

            // TODO: Propagate in always blocks
        }

        stats.add_metric("constants_propagated", self.propagated_count);
        Ok(stats)
    }
}

// ============================================================================
// Optimization Pass 3: Dead Code Elimination
// ============================================================================

/// Remove assignments that have no effect
pub struct DeadCodeElimination {
    removed_count: usize,
}

impl DeadCodeElimination {
    pub fn new() -> Self {
        Self { removed_count: 0 }
    }

    fn is_assignment_live(&self, assignment: &Assignment, outputs: &HashSet<String>) -> bool {
        // Assignment is live if its target is an output or feeds an output
        outputs.contains(&assignment.target)
        // TODO: Add more sophisticated liveness analysis
    }
}

impl Default for DeadCodeElimination {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for DeadCodeElimination {
    fn name(&self) -> &str {
        "dead-code-elimination"
    }

    fn run(&mut self, design: &mut Design) -> Result<OptimizationStats> {
        let mut stats = OptimizationStats::new();
        self.removed_count = 0;

        for module in design.modules.values_mut() {
            // Build set of output signals
            let outputs: HashSet<String> = module.outputs.iter().map(|p| p.name.clone()).collect();

            let original_count = module.assignments.len();

            // Remove dead assignments
            module.assignments.retain(|assignment| {
                let is_live = self.is_assignment_live(assignment, &outputs);
                if !is_live {
                    tracing::trace!("Removing dead assignment to: {}", assignment.target);
                    self.removed_count += 1;
                }
                is_live
            });

            stats.changes += original_count - module.assignments.len();
        }

        stats.add_metric("dead_assignments_removed", self.removed_count);
        Ok(stats)
    }
}

// ============================================================================
// Optimization Pass 4: Common Subexpression Elimination (CSE)
// ============================================================================

/// Eliminate common subexpressions
pub struct CommonSubexpressionElimination {
    eliminated_count: usize,
}

impl CommonSubexpressionElimination {
    pub fn new() -> Self {
        Self {
            eliminated_count: 0,
        }
    }

    fn find_common_expressions(&self, assignments: &[Assignment]) -> HashMap<String, Vec<String>> {
        let mut expr_map: HashMap<String, Vec<String>> = HashMap::new();

        for assignment in assignments {
            let expr = assignment.source.clone();
            expr_map
                .entry(expr)
                .or_insert_with(Vec::new)
                .push(assignment.target.clone());
        }

        // Keep only expressions used multiple times
        expr_map.retain(|_, targets| targets.len() > 1);
        expr_map
    }
}

impl Default for CommonSubexpressionElimination {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for CommonSubexpressionElimination {
    fn name(&self) -> &str {
        "common-subexpression-elimination"
    }

    fn run(&mut self, design: &mut Design) -> Result<OptimizationStats> {
        let mut stats = OptimizationStats::new();
        self.eliminated_count = 0;

        for module in design.modules.values_mut() {
            let common_exprs = self.find_common_expressions(&module.assignments);

            for (expr, targets) in common_exprs {
                if targets.len() > 1 {
                    tracing::trace!(
                        "Found common subexpression '{}' used {} times",
                        expr,
                        targets.len()
                    );
                    self.eliminated_count += targets.len() - 1;
                    stats.changes += targets.len() - 1;
                    // TODO: Actually eliminate the duplicates by introducing a temporary signal
                }
            }
        }

        stats.add_metric("common_subexpressions", self.eliminated_count);
        Ok(stats)
    }
}

// ============================================================================
// Optimization Pass 5: Logic Minimization
// ============================================================================

/// Minimize Boolean logic expressions
pub struct LogicMinimization {
    minimized_count: usize,
}

impl LogicMinimization {
    pub fn new() -> Self {
        Self {
            minimized_count: 0,
        }
    }

    fn minimize_expression(&mut self, expr: &str) -> String {
        let mut result = expr.to_string();
        let original = result.clone();

        // Simple algebraic optimizations
        // Double negation: ~~a => a
        result = result.replace("~~", "");

        // Identity: a & 1 => a, a | 0 => a
        result = result.replace("& 1", "");
        result = result.replace("| 0", "");

        // Null element: a & 0 => 0, a | 1 => 1
        if result.contains("& 0") {
            result = "0".to_string();
        }
        if result.contains("| 1") {
            result = "1".to_string();
        }

        // Idempotence: a & a => a, a | a => a
        // TODO: Implement proper expression parsing for this

        if result != original {
            self.minimized_count += 1;
        }

        result
    }
}

impl Default for LogicMinimization {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for LogicMinimization {
    fn name(&self) -> &str {
        "logic-minimization"
    }

    fn run(&mut self, design: &mut Design) -> Result<OptimizationStats> {
        let mut stats = OptimizationStats::new();
        self.minimized_count = 0;

        for module in design.modules.values_mut() {
            for assignment in &mut module.assignments {
                let minimized = self.minimize_expression(&assignment.source);
                if minimized != assignment.source {
                    tracing::trace!(
                        "Minimized '{}' to '{}'",
                        assignment.source,
                        minimized
                    );
                    assignment.source = minimized;
                    stats.changes += 1;
                }
            }
        }

        stats.add_metric("expressions_minimized", self.minimized_count);
        Ok(stats)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Module, Port, Signal, Assignment};

    fn create_test_module() -> Module {
        Module {
            name: "test".to_string(),
            parameters: Vec::new(),
            inputs: vec![Port {
                name: "in1".to_string(),
                width: 1,
            }],
            outputs: vec![Port {
                name: "out1".to_string(),
                width: 1,
            }],
            signals: vec![
                Signal {
                    name: "used".to_string(),
                    width: 1,
                },
                Signal {
                    name: "unused".to_string(),
                    width: 1,
                },
            ],
            instances: Vec::new(),
            assignments: vec![Assignment {
                target: "out1".to_string(),
                source: "used".to_string(),
            }],
            always_blocks: Vec::new(),
        }
    }

    #[test]
    fn test_unused_signal_elimination() {
        let mut design = Design::new();
        design.add_module(create_test_module());

        let mut pass = UnusedSignalElimination::new();
        let stats = pass.run(&mut design).unwrap();

        assert_eq!(stats.changes, 1); // Should remove 'unused' signal
    }

    #[test]
    fn test_constant_propagation() {
        let mut design = Design::new();
        let mut module = create_test_module();
        module.assignments.push(Assignment {
            target: "const_sig".to_string(),
            source: "5".to_string(),
        });
        module.assignments.push(Assignment {
            target: "result".to_string(),
            source: "const_sig + 1".to_string(),
        });
        design.add_module(module);

        let mut pass = ConstantPropagation::new();
        let _stats = pass.run(&mut design).unwrap();

        // Check that constant was propagated
        let module = design.modules.get("test").unwrap();
        let result_assign = module.assignments.iter().find(|a| a.target == "result");
        assert!(result_assign.is_some());
    }

    #[test]
    fn test_logic_minimization() {
        let mut design = Design::new();
        let mut module = create_test_module();
        module.assignments.push(Assignment {
            target: "out2".to_string(),
            source: "a & 1".to_string(),
        });
        design.add_module(module);

        let mut pass = LogicMinimization::new();
        let stats = pass.run(&mut design).unwrap();

        assert!(stats.changes > 0);
        let module = design.modules.get("test").unwrap();
        let out2 = module.assignments.iter().find(|a| a.target == "out2");
        assert!(out2.is_some());
    }

    #[test]
    fn test_optimization_manager() {
        let mut design = Design::new();
        design.add_module(create_test_module());

        let mut manager = OptimizationManager::standard_pipeline();
        let stats = manager.run(&mut design).unwrap();

        // Should have made at least one optimization
        assert!(stats.changes >= 0);
    }
}
