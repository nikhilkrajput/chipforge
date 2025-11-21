//! Common utilities and data structures for ChipForge
//!
//! This crate provides shared functionality used across all ChipForge components.

pub mod error;
pub mod graph;
pub mod identifier;
pub mod location;
pub mod types;

pub use error::{Error, Result};

/// ChipForge version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the common library (logging, etc.)
pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
