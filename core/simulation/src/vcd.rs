//! VCD (Value Change Dump) waveform writer
//!
//! Generates VCD files for viewing in GTKWave, Surfer, or other waveform viewers

use crate::event::SimTime;
use crate::value::{BitValue, LogicValue, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// VCD variable identifier (unique per signal)
type VcdId = String;

/// VCD scope type
#[derive(Debug, Clone, Copy)]
pub enum ScopeType {
    Module,
    Task,
    Function,
    Begin,
    Fork,
}

impl ScopeType {
    fn as_str(&self) -> &str {
        match self {
            ScopeType::Module => "module",
            ScopeType::Task => "task",
            ScopeType::Function => "function",
            ScopeType::Begin => "begin",
            ScopeType::Fork => "fork",
        }
    }
}

/// VCD variable type
#[derive(Debug, Clone, Copy)]
pub enum VarType {
    Wire,
    Reg,
    Integer,
    Real,
    Parameter,
    Supply0,
    Supply1,
}

impl VarType {
    fn as_str(&self) -> &str {
        match self {
            VarType::Wire => "wire",
            VarType::Reg => "reg",
            VarType::Integer => "integer",
            VarType::Real => "real",
            VarType::Parameter => "parameter",
            VarType::Supply0 => "supply0",
            VarType::Supply1 => "supply1",
        }
    }
}

/// VCD writer
pub struct VcdWriter {
    file: File,
    /// Variable name to VCD ID mapping
    var_ids: HashMap<String, VcdId>,
    /// Next variable ID
    next_id: usize,
    /// Current time
    current_time: Option<SimTime>,
    /// Time scale (e.g., "1ps", "1ns")
    timescale: String,
    /// Has header been written?
    header_written: bool,
}

impl VcdWriter {
    /// Create a new VCD writer
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            file,
            var_ids: HashMap::new(),
            next_id: 0,
            current_time: None,
            timescale: "1ps".to_string(),
            header_written: false,
        })
    }

    /// Set timescale (e.g., "1ps", "1ns", "1us")
    pub fn set_timescale(&mut self, timescale: String) {
        self.timescale = timescale;
    }

    /// Write VCD header
    pub fn write_header(&mut self, date: &str, version: &str, comment: &str) -> io::Result<()> {
        writeln!(self.file, "$date")?;
        writeln!(self.file, "    {}", date)?;
        writeln!(self.file, "$end")?;

        writeln!(self.file, "$version")?;
        writeln!(self.file, "    {}", version)?;
        writeln!(self.file, "$end")?;

        if !comment.is_empty() {
            writeln!(self.file, "$comment")?;
            writeln!(self.file, "    {}", comment)?;
            writeln!(self.file, "$end")?;
        }

        writeln!(self.file, "$timescale {} $end", self.timescale)?;

        self.header_written = true;
        Ok(())
    }

    /// Begin a scope
    pub fn scope_begin(&mut self, scope_type: ScopeType, name: &str) -> io::Result<()> {
        writeln!(
            self.file,
            "$scope {} {} $end",
            scope_type.as_str(),
            name
        )?;
        Ok(())
    }

    /// End current scope
    pub fn scope_end(&mut self) -> io::Result<()> {
        writeln!(self.file, "$upscope $end")?;
        Ok(())
    }

    /// Declare a variable
    pub fn var(
        &mut self,
        var_type: VarType,
        width: usize,
        name: &str,
    ) -> io::Result<VcdId> {
        let id = self.generate_id();
        writeln!(
            self.file,
            "$var {} {} {} {} $end",
            var_type.as_str(),
            width,
            id,
            name
        )?;

        self.var_ids.insert(name.to_string(), id.clone());
        Ok(id)
    }

    /// End variable declarations
    pub fn enddefinitions(&mut self) -> io::Result<()> {
        writeln!(self.file, "$enddefinitions $end")?;
        Ok(())
    }

    /// Write initial values
    pub fn dumpvars(&mut self) -> io::Result<()> {
        writeln!(self.file, "$dumpvars")?;
        Ok(())
    }

    /// End initial values
    pub fn end_dumpvars(&mut self) -> io::Result<()> {
        writeln!(self.file, "$end")?;
        Ok(())
    }

    /// Set current simulation time
    pub fn timestamp(&mut self, time: SimTime) -> io::Result<()> {
        if self.current_time != Some(time) {
            writeln!(self.file, "#{}", time)?;
            self.current_time = Some(time);
        }
        Ok(())
    }

    /// Write value change for a variable
    pub fn change_scalar(&mut self, id: &str, value: BitValue) -> io::Result<()> {
        let ch = match value {
            BitValue::Zero => '0',
            BitValue::One => '1',
            BitValue::X => 'x',
            BitValue::Z => 'z',
        };
        writeln!(self.file, "{}{}", ch, id)?;
        Ok(())
    }

    /// Write value change for a vector
    pub fn change_vector(&mut self, id: &str, value: &LogicValue) -> io::Result<()> {
        write!(self.file, "b")?;
        for i in (0..value.width()).rev() {
            let bit = value.get_bit(i).unwrap_or(BitValue::X);
            let ch = match bit {
                BitValue::Zero => '0',
                BitValue::One => '1',
                BitValue::X => 'x',
                BitValue::Z => 'z',
            };
            write!(self.file, "{}", ch)?;
        }
        writeln!(self.file, " {}", id)?;
        Ok(())
    }

    /// Write value change for a real
    pub fn change_real(&mut self, id: &str, value: f64) -> io::Result<()> {
        writeln!(self.file, "r{} {}", value, id)?;
        Ok(())
    }

    /// Write value change (auto-detect type)
    pub fn change(&mut self, id: &str, value: &Value) -> io::Result<()> {
        match value {
            Value::Bit(b) => self.change_scalar(id, *b),
            Value::Vector(v) => self.change_vector(id, v),
            Value::Real(r) => self.change_real(id, *r),
            Value::Integer(i) => {
                let vec = LogicValue::from_u64(*i as u64, 64);
                self.change_vector(id, &vec)
            }
            Value::String(_) => {
                // VCD doesn't support strings directly, skip
                Ok(())
            }
        }
    }

    /// Generate next variable ID
    fn generate_id(&mut self) -> VcdId {
        let id = self.id_to_string(self.next_id);
        self.next_id += 1;
        id
    }

    /// Convert number to VCD identifier (base-94 encoding)
    fn id_to_string(&self, mut num: usize) -> VcdId {
        const CHARS: &[u8] = b"!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

        if num == 0 {
            return String::from("!");
        }

        let mut result = Vec::new();
        while num > 0 {
            result.push(CHARS[num % CHARS.len()]);
            num /= CHARS.len();
        }
        result.reverse();

        String::from_utf8(result).unwrap()
    }

    /// Get VCD ID for a variable name
    pub fn get_id(&self, name: &str) -> Option<&VcdId> {
        self.var_ids.get(name)
    }

    /// Flush writer
    pub fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn test_vcd_creation() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let writer = VcdWriter::new(temp_file.path())?;
        assert!(!writer.header_written);
        Ok(())
    }

    #[test]
    fn test_vcd_header() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let mut writer = VcdWriter::new(temp_file.path())?;

        writer.write_header(
            "2024-11-21",
            "ChipForge 0.1.0",
            "Test simulation",
        )?;

        writer.flush()?;

        // Read file and check contents
        let mut file = File::open(temp_file.path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        assert!(contents.contains("$date"));
        assert!(contents.contains("$version"));
        assert!(contents.contains("ChipForge"));

        Ok(())
    }

    #[test]
    fn test_id_generation() {
        let mut writer = VcdWriter::new(NamedTempFile::new().unwrap().path()).unwrap();

        let id1 = writer.generate_id();
        let id2 = writer.generate_id();
        let id3 = writer.generate_id();

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
    }

    #[test]
    fn test_full_vcd_workflow() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let mut writer = VcdWriter::new(temp_file.path())?;

        // Write header
        writer.write_header("2024-11-21", "ChipForge", "Test")?;

        // Define structure
        writer.scope_begin(ScopeType::Module, "top")?;
        let clk_id = writer.var(VarType::Wire, 1, "clk")?;
        let data_id = writer.var(VarType::Reg, 8, "data")?;
        writer.scope_end()?;

        writer.enddefinitions()?;

        // Write initial values
        writer.dumpvars()?;
        writer.change_scalar(&clk_id, BitValue::Zero)?;
        writer.change_vector(&data_id, &LogicValue::from_u64(0, 8))?;
        writer.end_dumpvars()?;

        // Write value changes
        writer.timestamp(10)?;
        writer.change_scalar(&clk_id, BitValue::One)?;

        writer.timestamp(20)?;
        writer.change_scalar(&clk_id, BitValue::Zero)?;
        writer.change_vector(&data_id, &LogicValue::from_u64(0x42, 8))?;

        writer.flush()?;

        // Verify file was created and has content
        let mut file = File::open(temp_file.path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        assert!(contents.contains("$var wire 1"));
        assert!(contents.contains("$var reg 8"));
        assert!(contents.contains("#10"));
        assert!(contents.contains("#20"));

        Ok(())
    }
}
