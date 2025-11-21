//! Frontend parsing for different HDL languages

pub mod verilog;

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
