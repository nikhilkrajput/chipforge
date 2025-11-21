//! Simulate command

use anyhow::Result;
use colored::*;
use std::path::PathBuf;

pub async fn execute(
    _testbench: PathBuf,
    files: Vec<PathBuf>,
    _time: Option<String>,
    _wave: Option<PathBuf>,
) -> Result<()> {
    println!(
        "{} Simulating {} file(s)",
        "→".bright_green(),
        files.len()
    );

    println!("{} Simulation not yet implemented", "⚠".bright_yellow());

    Ok(())
}
