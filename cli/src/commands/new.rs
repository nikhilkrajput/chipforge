//! Create a new project command

use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::PathBuf;

pub async fn execute(name: String, template: String, output: Option<PathBuf>) -> Result<()> {
    println!("{} Creating new project: {}", "→".bright_green(), name.bright_white().bold());

    let project_dir = output.unwrap_or_else(|| PathBuf::from(&name));

    // Create project directory
    fs::create_dir_all(&project_dir)
        .context("Failed to create project directory")?;

    // Create subdirectories
    let dirs = vec!["src", "sim", "constraints", "ip"];
    for dir in dirs {
        fs::create_dir_all(project_dir.join(dir))?;
    }

    // Create project configuration
    let config = format!(
        r#"[project]
name = "{}"
version = "0.1.0"
template = "{}"

[synthesis]
target = "generic"
top_module = "top"

[simulation]
simulator = "chipforge"
"#,
        name, template
    );

    fs::write(project_dir.join("chipforge.toml"), config)?;

    // Create a simple example design
    let example_rtl = match template.as_str() {
        "blinky" => create_blinky_example(),
        _ => create_empty_example(),
    };

    fs::write(project_dir.join("src/top.sv"), example_rtl)?;

    println!("{} Project created successfully!", "✓".bright_green());
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  chipforge synthesize src/top.sv");

    Ok(())
}

fn create_blinky_example() -> String {
    r#"// Simple LED blinker example
module top (
    input wire clk,
    input wire reset,
    output reg led
);

    reg [23:0] counter;

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            counter <= 24'h0;
            led <= 1'b0;
        end else begin
            counter <= counter + 1;
            if (counter == 24'hFFFFFF) begin
                led <= ~led;
            end
        end
    end

endmodule
"#
    .to_string()
}

fn create_empty_example() -> String {
    r#"// Top module
module top (
    input wire clk,
    input wire reset
);

    // Your design here

endmodule
"#
    .to_string()
}
