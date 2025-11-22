# Clock Divider IP Core

Integer clock divider with 50% duty cycle output.

## Features

- Integer division ratios
- 50% duty cycle output
- Optimized divide-by-2 implementation
- Synchronous reset
- Low resource usage

## Parameters

- `DIVIDE_BY`: Division factor (must be >= 2)

## Interface

### Inputs
- `clk_in`: Input clock
- `rst`: Reset

### Outputs
- `clk_out`: Divided output clock

## Usage

```systemverilog
// Divide 100MHz by 4 to get 25MHz
clock_divider #(
    .DIVIDE_BY(4)
) clk_div (
    .clk_in(clk_100mhz),
    .rst(rst),
    .clk_out(clk_25mhz)
);
```

## Frequency Calculation

```
f_out = f_in / DIVIDE_BY

Examples:
- 100 MHz ÷ 2 = 50 MHz
- 100 MHz ÷ 4 = 25 MHz
- 100 MHz ÷ 10 = 10 MHz
```

## Duty Cycle

Output has 50% duty cycle for even division ratios.

For odd division ratios:
- Slight duty cycle variation
- Still acceptable for most applications

## Timing Diagram

```
DIVIDE_BY=4 example:

clk_in:  _/‾\_/‾\_/‾\_/‾\_/‾\_/‾\_/‾\_/‾\_
clk_out: ___/‾‾‾‾‾\______/‾‾‾‾‾\______
```

## Important Notes

**WARNING**: This creates a divided clock, NOT a clock enable!

For most FPGA designs, prefer:
1. Clock enables instead of divided clocks
2. PLLs/MMCMs for precise clock generation
3. This module for simple integer division only

## Clock Enable Alternative

Instead of clock divider, consider:
```systemverilog
reg [3:0] counter;
reg clk_en;

always @(posedge clk) begin
    if (counter == 3) begin
        counter <= 0;
        clk_en <= 1'b1;
    end else begin
        counter <= counter + 1;
        clk_en <= 1'b0;
    end
end

always @(posedge clk) begin
    if (clk_en) begin
        // Logic runs at divided rate
    end
end
```

## Use Cases

- Simple clock generation
- Test benches
- Legacy design integration
- Low-speed peripherals

## Resource Usage

- Logic: ~10 LUTs
- Registers: $clog2(DIVIDE_BY) + 1
- No memory

Divide-by-2: 1 register only
