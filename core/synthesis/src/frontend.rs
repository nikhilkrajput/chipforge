//! Frontend parsing for different HDL languages

pub mod verilog;

// TODO: Add more frontends
// pub mod systemverilog;
// pub mod vhdl;
// pub mod chisel;
// pub mod spinalhdl;
// pub mod systemc;

use chipforge_common::Result;
use crate::ir::Design;

/// Supported HDL languages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Verilog,
    SystemVerilog,
    VHDL,
}

impl Language {
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "v" => Some(Language::Verilog),
            "sv" | "svh" => Some(Language::SystemVerilog),
            "vhd" | "vhdl" => Some(Language::VHDL),
            _ => None,
        }
    }
}

/// Frontend for parsing HDL source files
pub trait Frontend {
    /// Parse source files and add to design
    fn parse(&self, files: &[std::path::PathBuf], design: &mut Design) -> Result<()>;
}

/// Verilog frontend
pub struct VerilogFrontend;

impl Frontend for VerilogFrontend {
    fn parse(&self, files: &[std::path::PathBuf], _design: &mut Design) -> Result<()> {
        for file in files {
            let content = std::fs::read_to_string(file)?;
            let filename = file.to_string_lossy().to_string();

            let ast = verilog::parse_file(&content, &filename)?;

            // Convert AST to IR (to be implemented)
            // For now, just log that we parsed it successfully
            tracing::info!("Successfully parsed {} with {} items", filename, ast.items.len());

            // TODO: Convert AST to IR and add to design
        }

        Ok(())
    }
}

/// Create a frontend for the given file extension
pub fn create_frontend(extension: &str) -> Result<Box<dyn Frontend>> {
    match extension {
        "v" | "vh" => Ok(Box::new(VerilogFrontend)),
        "sv" | "svh" => Ok(Box::new(VerilogFrontend)), // Use Verilog frontend for now
        _ => Err(chipforge_common::Error::unsupported(
            format!("Unsupported file extension: {}", extension)
        )),
    }
}
