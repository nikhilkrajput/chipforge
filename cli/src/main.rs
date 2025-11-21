//! ChipForge CLI - The Ultimate Open-Source Hardware Design Suite

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(name = "chipforge")]
#[command(author = "ChipForge Contributors")]
#[command(version = chipforge_common::VERSION)]
#[command(about = "The Ultimate Open-Source Hardware Design Suite", long_about = None)]
struct Cli {
    /// Set logging level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New {
        /// Project name
        name: String,

        /// Project template
        #[arg(short, long, default_value = "empty")]
        template: String,

        /// Target directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Synthesize a design
    Synthesize {
        /// Input files
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Top module name
        #[arg(short, long)]
        top: Option<String>,

        /// Target FPGA/ASIC
        #[arg(short = 't', long, default_value = "generic")]
        target: String,

        /// Optimization level (0-3)
        #[arg(short = 'O', long, default_value = "2")]
        optimization: u8,

        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Run simulation
    Simulate {
        /// Testbench file
        testbench: PathBuf,

        /// Design files
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Simulation time limit
        #[arg(short, long)]
        time: Option<String>,

        /// Waveform output file
        #[arg(short, long)]
        wave: Option<PathBuf>,
    },

    /// Analyze a design
    Analyze {
        /// Input files
        files: Vec<PathBuf>,

        /// Analysis type (lint, timing, power)
        #[arg(short, long, default_value = "lint")]
        analysis_type: String,
    },

    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(&cli.log_level)?;

    // Print banner
    print_banner();

    // Execute command
    match cli.command {
        Commands::New {
            name,
            template,
            output,
        } => commands::new::execute(name, template, output).await?,

        Commands::Synthesize {
            files,
            top,
            target,
            optimization,
            output,
        } => commands::synthesize::execute(files, top, target, optimization, output).await?,

        Commands::Simulate {
            testbench,
            files,
            time,
            wave,
        } => commands::simulate::execute(testbench, files, time, wave).await?,

        Commands::Analyze {
            files,
            analysis_type,
        } => commands::analyze::execute(files, analysis_type).await?,

        Commands::Version => {
            println!("{}", chipforge_common::VERSION);
        }
    }

    Ok(())
}

fn init_logging(level: &str) -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false))
        .with(env_filter)
        .init();

    Ok(())
}

fn print_banner() {
    let banner = r#"
     ██████╗██╗  ██╗██╗██████╗ ███████╗ ██████╗ ██████╗  ██████╗ ███████╗
    ██╔════╝██║  ██║██║██╔══██╗██╔════╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝
    ██║     ███████║██║██████╔╝█████╗  ██║   ██║██████╔╝██║  ███╗█████╗
    ██║     ██╔══██║██║██╔═══╝ ██╔══╝  ██║   ██║██╔══██╗██║   ██║██╔══╝
    ╚██████╗██║  ██║██║██║     ██║     ╚██████╔╝██║  ██║╚██████╔╝███████╗
     ╚═════╝╚═╝  ╚═╝╚═╝╚═╝     ╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝
    "#;

    println!("{}", banner.bright_cyan());
    println!(
        "    {} v{}\n",
        "ChipForge".bright_white().bold(),
        chipforge_common::VERSION.bright_yellow()
    );
}
