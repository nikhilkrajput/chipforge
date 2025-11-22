// Elaboration engine - converts AST to IR
use chipforge_common::error::Result;
use crate::frontend::verilog::ast::{self, SourceFile};
use crate::ir::{Design, Module, Port, Signal};
use std::collections::HashMap;

/// Elaboration engine
pub struct Elaborator {
    /// Source AST
    ast_modules: HashMap<String, ast::Module>,
    /// Elaborated design
    design: Design,
}

impl Elaborator {
    /// Create a new elaborator
    pub fn new() -> Self {
        Elaborator {
            ast_modules: HashMap::new(),
            design: Design::new(),
        }
    }

    /// Elaborate a source file
    pub fn elaborate(&mut self, source: &SourceFile) -> Result<Design> {
        // Step 1: Collect all modules
        self.collect_modules(source)?;

        // Step 2: Elaborate each module
        for (_name, ast_module) in &self.ast_modules.clone() {
            let module = self.elaborate_module(ast_module)?;
            self.design.add_module(module);
        }

        Ok(self.design.clone())
    }

    /// Collect all modules from the source
    fn collect_modules(&mut self, source: &SourceFile) -> Result<()> {
        for item in &source.items {
            match item {
                ast::Item::Module(module) => {
                    self.ast_modules.insert(module.name.clone(), module.clone());
                }
            }
        }
        Ok(())
    }

    /// Elaborate a single module
    fn elaborate_module(&self, ast_module: &ast::Module) -> Result<Module> {
        let name = ast_module.name.clone();

        // Elaborate parameters
        let parameters = self.elaborate_parameters(&ast_module.params)?;

        // Elaborate ports
        let (inputs, outputs) = self.elaborate_ports(&ast_module.ports)?;

        // Elaborate internal signals
        let signals = self.elaborate_signals(&ast_module.items)?;

        // TODO: Elaborate instances, assignments, and always blocks
        let instances = Vec::new();
        let assignments = Vec::new();
        let always_blocks = Vec::new();

        Ok(Module {
            name,
            parameters,
            inputs,
            outputs,
            signals,
            instances,
            assignments,
            always_blocks,
        })
    }

    /// Elaborate module parameters
    fn elaborate_parameters(&self, params: &[ast::Parameter]) -> Result<Vec<crate::ir::Parameter>> {
        let mut parameters = Vec::new();

        for param in params {
            let param_name = param.name.clone();
            let default_value = if param.default_value.is_some() {
                // TODO: Evaluate expression to get value
                Some(0)
            } else {
                None
            };

            parameters.push(crate::ir::Parameter {
                name: param_name,
                default_value,
            });
        }

        Ok(parameters)
    }

    /// Elaborate module ports
    fn elaborate_ports(&self, ports: &[ast::Port]) -> Result<(Vec<Port>, Vec<Port>)> {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        for port in ports {
            let port_name = port.name.clone();
            let width = self.calculate_width_from_port(port)?;

            let ir_port = Port {
                name: port_name,
                width,
            };

            match port.direction {
                Some(ast::PortDirection::Input) => inputs.push(ir_port),
                Some(ast::PortDirection::Output) => outputs.push(ir_port),
                Some(ast::PortDirection::Inout) => {
                    // Bidirectional ports become both input and output
                    inputs.push(ir_port.clone());
                    outputs.push(ir_port);
                }
                None => {
                    // Default to input if not specified
                    inputs.push(ir_port);
                }
            }
        }

        Ok((inputs, outputs))
    }

    /// Calculate width from port
    fn calculate_width_from_port(&self, _port: &ast::Port) -> Result<usize> {
        // TODO: Extract width from port type properly
        // For now, return 1 (single bit)
        Ok(1)
    }

    /// Elaborate internal signals from module items
    fn elaborate_signals(&self, items: &[ast::ModuleItem]) -> Result<Vec<Signal>> {
        let mut signals = Vec::new();

        for item in items {
            match item {
                ast::ModuleItem::NetDeclaration(decl) => {
                    for name in &decl.names {
                        signals.push(Signal {
                            name: name.clone(),
                            width: self.calculate_width(&decl.data_type)?,
                        });
                    }
                }
                _ => {
                    // TODO: Handle other module items (variables, instances, always blocks, etc.)
                }
            }
        }

        Ok(signals)
    }

    /// Calculate bit width from data type
    fn calculate_width(&self, data_type: &ast::DataType) -> Result<usize> {
        match data_type {
            ast::DataType::Logic(range) | ast::DataType::Wire(range) | ast::DataType::Reg(range) => {
                if let Some(range) = range {
                    // Calculate width from range
                    Ok(self.calculate_range_width(range)?)
                } else {
                    // Single bit
                    Ok(1)
                }
            }
            ast::DataType::Integer => Ok(32),
            ast::DataType::Real => Ok(64),
            _ => Ok(1), // Default for other types
        }
    }

    /// Calculate width from a range
    fn calculate_range_width(&self, range: &ast::Range) -> Result<usize> {
        // For now, assume constant expressions
        // TODO: Evaluate expressions properly
        match (&range.msb, &range.lsb) {
            (ast::Expression::Number(_, msb_lit), ast::Expression::Number(_, lsb_lit)) => {
                let msb = self.evaluate_number_from_expr(&range.msb)?;
                let lsb = self.evaluate_number_from_expr(&range.lsb)?;
                let width = (msb as i64 - lsb as i64).abs() + 1;
                Ok(width as usize)
            }
            _ => {
                // Can't evaluate yet, default to 1
                Ok(1)
            }
        }
    }

    /// Evaluate a number from expression
    fn evaluate_number_from_expr(&self, expr: &ast::Expression) -> Result<usize> {
        match expr {
            ast::Expression::Number(_, _lit) => {
                // TODO: Parse number literal properly
                // For now, return a default value
                Ok(0)
            }
            _ => Ok(0),
        }
    }
}

impl Default for Elaborator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::verilog::parser::Parser;
    use crate::frontend::verilog::lexer::Lexer;

    #[test]
    fn test_elaborate_simple_module() {
        let source = r#"
module simple (
    input wire clk,
    input wire [7:0] data_in,
    output reg [7:0] data_out
);
    wire [3:0] internal;
    reg [15:0] counter;
endmodule
"#;

        let lexer = Lexer::new(source, "test.v");
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_source_file().unwrap();

        let mut elaborator = Elaborator::new();
        let design = elaborator.elaborate(&ast).unwrap();

        assert_eq!(design.modules.len(), 1);

        let module = design.modules.values().next().unwrap();
        assert_eq!(module.inputs.len(), 2);
        assert_eq!(module.outputs.len(), 1);

        // Check port widths
        assert_eq!(module.inputs[0].width, 1); // clk
        assert_eq!(module.inputs[1].width, 1); // data_in (TODO: should be 8)
        assert_eq!(module.outputs[0].width, 1); // data_out (TODO: should be 8)
    }

    #[test]
    fn test_elaborate_multiple_modules() {
        let source = r#"
module top (
    input wire clk,
    output wire [7:0] out
);
endmodule

module bottom (
    input wire rst
);
endmodule
"#;

        let lexer = Lexer::new(source, "test.v");
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_source_file().unwrap();

        let mut elaborator = Elaborator::new();
        let design = elaborator.elaborate(&ast).unwrap();

        assert_eq!(design.modules.len(), 2);
    }
}
