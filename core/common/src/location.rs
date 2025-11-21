//! Source location tracking

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    /// File ID (index into file table)
    pub file: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl Location {
    /// Create a new location
    pub fn new(file: usize, line: usize, column: usize) -> Self {
        Location { file, line, column }
    }

    /// Create a dummy location (for generated code)
    pub fn dummy() -> Self {
        Location {
            file: 0,
            line: 0,
            column: 0,
        }
    }

    /// Check if this is a dummy location
    pub fn is_dummy(&self) -> bool {
        self.line == 0 && self.column == 0
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dummy() {
            write!(f, "<generated>")
        } else {
            write!(f, "{}:{}:{}", self.file, self.line, self.column)
        }
    }
}

/// Represents a span of source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Start location
    pub start: Location,
    /// End location
    pub end: Location,
}

impl Span {
    /// Create a new span
    pub fn new(start: Location, end: Location) -> Self {
        Span { start, end }
    }

    /// Create a dummy span
    pub fn dummy() -> Self {
        Span {
            start: Location::dummy(),
            end: Location::dummy(),
        }
    }

    /// Check if this is a dummy span
    pub fn is_dummy(&self) -> bool {
        self.start.is_dummy() && self.end.is_dummy()
    }

    /// Combine two spans into one that covers both
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start,
            end: other.end,
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dummy() {
            write!(f, "<generated>")
        } else {
            write!(f, "{} to {}", self.start, self.end)
        }
    }
}
