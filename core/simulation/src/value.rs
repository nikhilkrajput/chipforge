//! Logic values and operations for simulation
//!
//! Implements 4-state logic (0, 1, X, Z) as used in Verilog/SystemVerilog

use std::fmt;
use std::ops::Not;

/// Single bit value (4-state logic)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitValue {
    /// Logic 0
    Zero,
    /// Logic 1
    One,
    /// Unknown/uninitialized
    X,
    /// High impedance
    Z,
}

impl BitValue {
    /// Check if value is known (0 or 1)
    pub fn is_known(&self) -> bool {
        matches!(self, BitValue::Zero | BitValue::One)
    }

    /// Convert to boolean (X and Z become false)
    pub fn to_bool(&self) -> bool {
        matches!(self, BitValue::One)
    }

    /// Convert from boolean
    pub fn from_bool(b: bool) -> Self {
        if b {
            BitValue::One
        } else {
            BitValue::Zero
        }
    }

    /// Logical AND
    pub fn and(self, other: Self) -> Self {
        use BitValue::*;
        match (self, other) {
            (Zero, _) | (_, Zero) => Zero,
            (One, One) => One,
            (X, One) | (One, X) | (X, X) => X,
            _ => X, // Any Z becomes X
        }
    }

    /// Logical OR
    pub fn or(self, other: Self) -> Self {
        use BitValue::*;
        match (self, other) {
            (One, _) | (_, One) => One,
            (Zero, Zero) => Zero,
            (X, Zero) | (Zero, X) | (X, X) => X,
            _ => X, // Any Z becomes X
        }
    }

    /// Logical XOR
    pub fn xor(self, other: Self) -> Self {
        use BitValue::*;
        match (self, other) {
            (Zero, Zero) | (One, One) => Zero,
            (Zero, One) | (One, Zero) => One,
            _ => X,
        }
    }

}

/// Implement the Not trait for BitValue
impl Not for BitValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        use BitValue::*;
        match self {
            Zero => One,
            One => Zero,
            X => X,
            Z => X,
        }
    }
}

impl fmt::Display for BitValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitValue::Zero => write!(f, "0"),
            BitValue::One => write!(f, "1"),
            BitValue::X => write!(f, "x"),
            BitValue::Z => write!(f, "z"),
        }
    }
}

/// Multi-bit logic value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicValue {
    bits: Vec<BitValue>,
}

impl LogicValue {
    /// Create a new logic value from bits (LSB first)
    pub fn new(bits: Vec<BitValue>) -> Self {
        Self { bits }
    }

    /// Create from integer value
    pub fn from_u64(value: u64, width: usize) -> Self {
        let mut bits = Vec::with_capacity(width);
        for i in 0..width {
            let bit = (value >> i) & 1;
            bits.push(BitValue::from_bool(bit == 1));
        }
        Self { bits }
    }

    /// Create X value of given width
    pub fn x(width: usize) -> Self {
        Self {
            bits: vec![BitValue::X; width],
        }
    }

    /// Create Z value of given width
    pub fn z(width: usize) -> Self {
        Self {
            bits: vec![BitValue::Z; width],
        }
    }

    /// Get width (number of bits)
    pub fn width(&self) -> usize {
        self.bits.len()
    }

    /// Get bit at index (LSB = 0)
    pub fn get_bit(&self, index: usize) -> Option<BitValue> {
        self.bits.get(index).copied()
    }

    /// Set bit at index (LSB = 0)
    pub fn set_bit(&mut self, index: usize, value: BitValue) {
        if index < self.bits.len() {
            self.bits[index] = value;
        }
    }

    /// Convert to u64 (X and Z become 0)
    pub fn to_u64(&self) -> u64 {
        let mut result = 0u64;
        for (i, bit) in self.bits.iter().enumerate() {
            if *bit == BitValue::One {
                result |= 1 << i;
            }
        }
        result
    }

    /// Check if all bits are known (0 or 1)
    pub fn is_known(&self) -> bool {
        self.bits.iter().all(|b| b.is_known())
    }

    /// Bitwise AND
    pub fn and(&self, other: &Self) -> Self {
        let width = self.width().max(other.width());
        let mut bits = Vec::with_capacity(width);
        for i in 0..width {
            let a = self.get_bit(i).unwrap_or(BitValue::Zero);
            let b = other.get_bit(i).unwrap_or(BitValue::Zero);
            bits.push(a.and(b));
        }
        Self { bits }
    }

    /// Bitwise OR
    pub fn or(&self, other: &Self) -> Self {
        let width = self.width().max(other.width());
        let mut bits = Vec::with_capacity(width);
        for i in 0..width {
            let a = self.get_bit(i).unwrap_or(BitValue::Zero);
            let b = other.get_bit(i).unwrap_or(BitValue::Zero);
            bits.push(a.or(b));
        }
        Self { bits }
    }

    /// Bitwise XOR
    pub fn xor(&self, other: &Self) -> Self {
        let width = self.width().max(other.width());
        let mut bits = Vec::with_capacity(width);
        for i in 0..width {
            let a = self.get_bit(i).unwrap_or(BitValue::Zero);
            let b = other.get_bit(i).unwrap_or(BitValue::Zero);
            bits.push(a.xor(b));
        }
        Self { bits }
    }

    /// Bitwise NOT
    pub fn not(&self) -> Self {
        Self {
            bits: self.bits.iter().map(|b| !*b).collect(),
        }
    }

    /// Reduction AND (all bits ANDed together)
    pub fn reduce_and(&self) -> BitValue {
        self.bits
            .iter()
            .fold(BitValue::One, |acc, &b| acc.and(b))
    }

    /// Reduction OR (all bits ORed together)
    pub fn reduce_or(&self) -> BitValue {
        self.bits
            .iter()
            .fold(BitValue::Zero, |acc, &b| acc.or(b))
    }

    /// Reduction XOR (all bits XORed together)
    pub fn reduce_xor(&self) -> BitValue {
        self.bits
            .iter()
            .fold(BitValue::Zero, |acc, &b| acc.xor(b))
    }
}

impl fmt::Display for LogicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in self.bits.iter().rev() {
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

/// Generic value type for simulation
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Single bit
    Bit(BitValue),
    /// Multi-bit vector
    Vector(LogicValue),
    /// Integer (for parameters, variables)
    Integer(i64),
    /// Real number
    Real(f64),
    /// String
    String(String),
}

impl Value {
    /// Create from boolean
    pub fn from_bool(b: bool) -> Self {
        Value::Bit(BitValue::from_bool(b))
    }

    /// Create from integer
    pub fn from_int(value: i64, width: Option<usize>) -> Self {
        if let Some(w) = width {
            Value::Vector(LogicValue::from_u64(value as u64, w))
        } else {
            Value::Integer(value)
        }
    }

    /// Convert to boolean (for conditional evaluation)
    pub fn to_bool(&self) -> bool {
        match self {
            Value::Bit(b) => b.to_bool(),
            Value::Vector(v) => v.reduce_or().to_bool(),
            Value::Integer(i) => *i != 0,
            Value::Real(r) => *r != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }

    /// Get width in bits
    pub fn width(&self) -> usize {
        match self {
            Value::Bit(_) => 1,
            Value::Vector(v) => v.width(),
            Value::Integer(_) => 64,
            Value::Real(_) => 64,
            Value::String(s) => s.len() * 8,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bit(b) => write!(f, "{}", b),
            Value::Vector(v) => write!(f, "{}", v),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Real(r) => write!(f, "{}", r),
            Value::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_value_logic() {
        assert_eq!(BitValue::Zero.and(BitValue::One), BitValue::Zero);
        assert_eq!(BitValue::One.and(BitValue::One), BitValue::One);
        assert_eq!(BitValue::Zero.or(BitValue::One), BitValue::One);
        assert_eq!(BitValue::One.xor(BitValue::One), BitValue::Zero);
        assert_eq!(!BitValue::Zero, BitValue::One);
    }

    #[test]
    fn test_logic_value_from_u64() {
        let val = LogicValue::from_u64(5, 4); // Binary: 0101
        assert_eq!(val.width(), 4);
        assert_eq!(val.to_u64(), 5);
        assert_eq!(val.get_bit(0), Some(BitValue::One));
        assert_eq!(val.get_bit(1), Some(BitValue::Zero));
        assert_eq!(val.get_bit(2), Some(BitValue::One));
        assert_eq!(val.get_bit(3), Some(BitValue::Zero));
    }

    #[test]
    fn test_logic_value_operations() {
        let a = LogicValue::from_u64(0b1010, 4);
        let b = LogicValue::from_u64(0b1100, 4);

        let and_result = a.and(&b);
        assert_eq!(and_result.to_u64(), 0b1000);

        let or_result = a.or(&b);
        assert_eq!(or_result.to_u64(), 0b1110);

        let xor_result = a.xor(&b);
        assert_eq!(xor_result.to_u64(), 0b0110);
    }

    #[test]
    fn test_reduction_operations() {
        let val = LogicValue::from_u64(0b1111, 4);
        assert_eq!(val.reduce_and(), BitValue::One);
        assert_eq!(val.reduce_or(), BitValue::One);

        let val = LogicValue::from_u64(0b0000, 4);
        assert_eq!(val.reduce_and(), BitValue::Zero);
        assert_eq!(val.reduce_or(), BitValue::Zero);
    }

    #[test]
    fn test_value_to_bool() {
        assert!(!Value::from_bool(false).to_bool());
        assert!(Value::from_bool(true).to_bool());
        assert!(!Value::from_int(0, None).to_bool());
        assert!(Value::from_int(42, None).to_bool());
    }
}
