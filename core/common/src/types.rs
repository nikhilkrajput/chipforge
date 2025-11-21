//! Type system for hardware design

use serde::{Deserialize, Serialize};
use std::fmt;

/// Hardware data type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Logic type (1-bit)
    Logic,
    /// Bit vector with specified width
    BitVector(usize),
    /// Integer type
    Integer,
    /// Real/floating-point type
    Real,
    /// String type
    String,
    /// Array type
    Array {
        element_type: Box<Type>,
        size: usize,
    },
    /// Struct type
    Struct {
        name: Option<String>,
        fields: Vec<(String, Type)>,
    },
    /// Enum type
    Enum {
        name: Option<String>,
        variants: Vec<String>,
    },
    /// User-defined type
    UserDefined(String),
    /// Unknown/unresolved type
    Unknown,
}

impl Type {
    /// Check if type is a logic type (1-bit)
    pub fn is_logic(&self) -> bool {
        matches!(self, Type::Logic)
    }

    /// Check if type is a bit vector
    pub fn is_bit_vector(&self) -> bool {
        matches!(self, Type::BitVector(_))
    }

    /// Get the width in bits (if applicable)
    pub fn width(&self) -> Option<usize> {
        match self {
            Type::Logic => Some(1),
            Type::BitVector(w) => Some(*w),
            _ => None,
        }
    }

    /// Check if two types are compatible for assignment
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Logic, Type::Logic) => true,
            (Type::BitVector(w1), Type::BitVector(w2)) => w1 == w2,
            (Type::Integer, Type::Integer) => true,
            (Type::Real, Type::Real) => true,
            (Type::String, Type::String) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Logic => write!(f, "logic"),
            Type::BitVector(w) => write!(f, "logic[{}:0]", w - 1),
            Type::Integer => write!(f, "integer"),
            Type::Real => write!(f, "real"),
            Type::String => write!(f, "string"),
            Type::Array { element_type, size } => {
                write!(f, "{}[{}]", element_type, size)
            }
            Type::Struct { name, .. } => {
                write!(f, "struct {}", name.as_deref().unwrap_or("<anonymous>"))
            }
            Type::Enum { name, .. } => {
                write!(f, "enum {}", name.as_deref().unwrap_or("<anonymous>"))
            }
            Type::UserDefined(name) => write!(f, "{}", name),
            Type::Unknown => write!(f, "<unknown>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_width() {
        assert_eq!(Type::Logic.width(), Some(1));
        assert_eq!(Type::BitVector(32).width(), Some(32));
        assert_eq!(Type::Integer.width(), None);
    }

    #[test]
    fn test_compatibility() {
        assert!(Type::Logic.is_compatible_with(&Type::Logic));
        assert!(Type::BitVector(8).is_compatible_with(&Type::BitVector(8)));
        assert!(!Type::BitVector(8).is_compatible_with(&Type::BitVector(16)));
    }
}
