//! Synthesize command

use anyhow::{Context, Result};
use chipforge_synthesis::{SynthesisConfig, SynthesisEngine};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Instant;

pub async fn execute(
    files: Vec<PathBuf>,
    top: Option<String>,
    target: String,
    optimization: u8,
    _output: Option<PathBuf>,
) -> Result<()> {
    println!(
        "{} Synthesizing {} file(s) for target: {}",
        "→".bright_green(),
        files.len(),
        target.bright_cyan()
    );

    // Verify all files exist
    for file in &files {
        if !file.exists() {
            anyhow::bail!("File not found: {}", file.display());
        }
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    pb.set_message("Parsing files...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let config = SynthesisConfig {
        target,
        optimization_level: optimization,
        top_module: top,
    };

    let engine = SynthesisEngine::new(config);

    let start = Instant::now();
    let _design = engine.synthesize(&files)
        .context("Synthesis failed")?;
    let duration = start.elapsed();

    pb.finish_and_clear();

    println!("{} Synthesis completed in {:.2}s", "✓".bright_green(), duration.as_secs_f64());
    println!("\nResults:");
    println!("  Files processed: {}", files.len());
    println!("  Optimization level: {}", optimization);

    Ok(())
}
