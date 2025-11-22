# Reset Controller IP Core

Generates clean synchronous reset from asynchronous reset input.

## Features

- Async reset input synchronization
- Configurable reset duration
- Synchronous reset output
- Reset done flag
- Metastability protection
- Power-on reset handling

## Parameters

- `RESET_CYCLES`: Number of clock cycles to hold reset (default: 16)

## Interface

### Inputs
- `clk`: System clock
- `async_rst_n`: Asynchronous reset input (active low)

### Outputs
- `sync_rst`: Synchronous reset output (active high)
- `rst_done`: Reset sequence complete flag

## Usage

```systemverilog
reset_controller #(
    .RESET_CYCLES(16)
) rst_ctrl (
    .clk(clk),
    .async_rst_n(rst_n),
    .sync_rst(rst),
    .rst_done(system_ready)
);

// Use sync_rst in your design
always @(posedge clk) begin
    if (rst) begin
        // Reset logic
    end else begin
        // Normal operation
    end
end
```

## How It Works

1. Asynchronous reset input passes through 2-stage synchronizer
2. On reset release, counter starts
3. Sync reset held for RESET_CYCLES clocks
4. After RESET_CYCLES, sync_rst deasserts
5. rst_done asserts when reset complete

## Timing Diagram

```
async_rst_n: \______/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
                    ^
sync_rst:    ‾‾‾‾‾‾‾‾‾‾‾‾\__________________
                          ^
                          |<-RESET_CYCLES->|
rst_done:    _________________/‾‾‾‾‾‾‾‾‾‾‾‾
```

## Why This Is Needed

### Problem: Async Reset Issues
- Metastability on reset release
- Timing violations
- Unpredictable behavior

### Solution: This Module
- Synchronizes reset to clock
- Guarantees setup/hold times
- Clean, glitch-free reset
- Meets timing in static timing analysis

## Reset Duration

Typical values:
- **8-16 cycles**: Most designs
- **32+ cycles**: PLL lock time
- **100+ cycles**: External device init

## Best Practices

1. Use synchronous reset in your design logic
2. Connect async_rst_n to board reset button/POR
3. Use rst_done to enable critical logic
4. Add RESET_CYCLES margin for safety

## Use Cases

- Power-on reset generation
- External reset button handling
- System reset coordination
- FPGA configuration reset

## Resource Usage

- Logic: ~15 LUTs
- Registers: ~$clog2(RESET_CYCLES) + 5
- No memory
