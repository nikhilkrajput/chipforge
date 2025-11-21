# Quick Start Guide

Get started with ChipForge in 5 minutes!

## Prerequisites

- Rust 1.75+ ([Install Rust](https://rustup.rs))
- 4GB RAM minimum
- Linux, macOS, or Windows

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/nikhilkrajput/chipforge.git
cd chipforge

# Build and install
cargo install --path cli

# Verify installation
chipforge --version
```

### Using Cargo

```bash
cargo install chipforge
```

## Your First Project

### 1. Create a New Project

```bash
chipforge new my_first_design --template blinky
cd my_first_design
```

This creates a project with:
- `chipforge.toml` - Project configuration
- `src/top.sv` - Main design file
- `sim/` - Testbenches
- `constraints/` - Timing and pin constraints

### 2. Examine the Design

```bash
cat src/top.sv
```

You'll see a simple LED blinker:

```systemverilog
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
```

### 3. Synthesize the Design

```bash
chipforge synthesize src/top.sv --target generic
```

### 4. Run Simulation (Coming Soon)

```bash
chipforge simulate sim/tb_top.sv src/top.sv --wave output.vcd
```

### 5. View Waveforms (Coming Soon)

```bash
chipforge wave output.vcd
```

## Next Steps

- [Tutorial Series](tutorial-series/) - Learn step-by-step
- [User Guide](../user-guide/) - Deep dive into features
- [Examples](../../examples/) - Sample projects

## Getting Help

- [Documentation](https://docs.chipforge.org)
- [Discord Community](https://discord.gg/chipforge)
- [GitHub Issues](https://github.com/nikhilkrajput/chipforge/issues)
