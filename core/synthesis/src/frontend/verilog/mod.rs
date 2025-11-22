//! Verilog frontend for ChipForge
//!
//! This module provides complete Verilog-2005 parsing with SystemVerilog extensions.

pub mod lexer;
pub mod ast;
pub mod parser;

pub use lexer::{Lexer, Token, NumberLiteral, NumberBase};
pub use ast::*;
pub use parser::Parser;

use chipforge_common::Result;

/// Parse a Verilog source file
pub fn parse_file(content: &str, filename: &str) -> Result<SourceFile> {
    let lexer = Lexer::new(content, &filename);
    let mut parser = Parser::new(lexer)?;
    parser.parse_source_file()
}

/// Parse a Verilog source string
pub fn parse(content: &str) -> Result<SourceFile> {
    parse_file(content, "<input>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_module() {
        let source = r#"
module simple_and(
    input wire a,
    input wire b,
    output wire y
);
    assign y = a & b;
endmodule
"#;

        let result = parse(source);
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast.items.len(), 1);

        if let Item::Module(module) = &ast.items[0] {
            assert_eq!(module.name, "simple_and");
            assert_eq!(module.ports.len(), 3);
        } else {
            panic!("Expected module");
        }
    }

    #[test]
    fn test_parse_counter() {
        let source = r#"
module counter #(
    parameter WIDTH = 8
)(
    input wire clk,
    input wire rst,
    output reg [WIDTH-1:0] count
);
    always @(posedge clk) begin
        if (rst)
            count <= 0;
        else
            count <= count + 1;
    end
endmodule
"#;

        let result = parse(source);
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast.items.len(), 1);

        if let Item::Module(module) = &ast.items[0] {
            assert_eq!(module.name, "counter");
            assert_eq!(module.params.len(), 1);
            assert_eq!(module.ports.len(), 3);
        } else {
            panic!("Expected module");
        }
    }

    #[test]
    fn test_parse_with_always_block() {
        let source = r#"
module dff(
    input clk,
    input d,
    output reg q
);
    always @(posedge clk)
        q <= d;
endmodule
"#;

        let result = parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_expressions() {
        let source = r#"
module expr_test;
    wire [7:0] a, b, c;
    assign c = (a + b) * 2;
    assign c = a[3:0] + b[7:4];
    assign c = {a[3:0], b[3:0]};
    assign c = a ? b : 8'hFF;
endmodule
"#;

        let result = parse(source);
        assert!(result.is_ok());
    }
}
