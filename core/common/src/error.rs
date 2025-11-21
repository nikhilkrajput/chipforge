//! Error types for ChipForge

/// Result type alias for ChipForge operations
pub type Result<T> = std::result::Result<T, Error>;

/// ChipForge error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Parse error
    #[error("Parse error at {location}: {message}")]
    Parse {
        message: String,
        location: crate::location::Location,
    },

    /// Elaboration error
    #[error("Elaboration error: {0}")]
    Elaboration(String),

    /// Type error
    #[error("Type error: {0}")]
    Type(String),

    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Internal error (should not happen)
    #[error("Internal error: {0}")]
    Internal(String),

    /// Generic error with context
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a parse error
    pub fn parse(message: impl Into<String>, location: crate::location::Location) -> Self {
        Error::Parse {
            message: message.into(),
            location,
        }
    }

    /// Create an elaboration error
    pub fn elaboration(message: impl Into<String>) -> Self {
        Error::Elaboration(message.into())
    }

    /// Create a type error
    pub fn type_error(message: impl Into<String>) -> Self {
        Error::Type(message.into())
    }

    /// Create a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Error::NotFound(message.into())
    }

    /// Create an invalid operation error
    pub fn invalid_operation(message: impl Into<String>) -> Self {
        Error::InvalidOperation(message.into())
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Error::Internal(message.into())
    }
}
