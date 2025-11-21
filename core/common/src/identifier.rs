//! Identifier management

use serde::{Deserialize, Serialize};

/// An interned string identifier
///
/// Identifiers are interned strings that can be compared by pointer equality.
/// This makes comparison and hashing very fast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier(u32);

impl Identifier {
    /// Create a new identifier from an index
    pub const fn new(index: u32) -> Self {
        Identifier(index)
    }

    /// Get the index of this identifier
    pub const fn index(self) -> u32 {
        self.0
    }
}

/// Identifier interner for string deduplication
pub struct IdentifierInterner {
    strings: Vec<String>,
    map: hashbrown::HashMap<String, u32>,
}

impl IdentifierInterner {
    /// Create a new interner
    pub fn new() -> Self {
        IdentifierInterner {
            strings: Vec::new(),
            map: hashbrown::HashMap::new(),
        }
    }

    /// Intern a string and return its identifier
    pub fn intern(&mut self, s: &str) -> Identifier {
        if let Some(&index) = self.map.get(s) {
            return Identifier(index);
        }

        let index = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), index);
        Identifier(index)
    }

    /// Get the string for an identifier
    pub fn resolve(&self, id: Identifier) -> &str {
        &self.strings[id.0 as usize]
    }

    /// Get the number of interned strings
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Check if the interner is empty
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}

impl Default for IdentifierInterner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interner() {
        let mut interner = IdentifierInterner::new();

        let foo1 = interner.intern("foo");
        let bar = interner.intern("bar");
        let foo2 = interner.intern("foo");

        assert_eq!(foo1, foo2);
        assert_ne!(foo1, bar);
        assert_eq!(interner.resolve(foo1), "foo");
        assert_eq!(interner.resolve(bar), "bar");
    }
}
