# GPIO IP Core

General Purpose Input/Output controller with configurable direction.

## Features

- Configurable width (number of pins)
- Per-pin direction control
- Per-pin output enable
- Input synchronization (2-stage)
- Tristate output control
- Registered outputs

## Parameters

- `WIDTH`: Number of GPIO pins (default: 32)

## Interface

### Inputs
- `clk`: Clock
- `rst`: Reset
- `data_out[WIDTH-1:0]`: Data to drive on output pins
- `direction[WIDTH-1:0]`: Direction control (1=output, 0=input)
- `output_enable[WIDTH-1:0]`: Output enable per pin

### Outputs
- `data_in[WIDTH-1:0]`: Synchronized data from pins

### Inouts
- `gpio_pins[WIDTH-1:0]`: Bidirectional GPIO pins

## Usage

```systemverilog
gpio #(
    .WIDTH(16)
) gpio_inst (
    .clk(clk),
    .rst(rst),
    .data_out(gpio_out),
    .data_in(gpio_in),
    .direction(gpio_dir),
    .output_enable(gpio_oe),
    .gpio_pins(gpio)
);
```

## Operation

### Output Mode
```systemverilog
gpio_dir[0] = 1'b1;        // Configure as output
gpio_oe[0] = 1'b1;         // Enable output driver
gpio_out[0] = 1'b1;        // Drive high
```

### Input Mode
```systemverilog
gpio_dir[0] = 1'b0;        // Configure as input
// or
gpio_oe[0] = 1'b0;         // Disable output driver
value = gpio_in[0];        // Read pin (synchronized)
```

## Input Synchronization

All inputs pass through 2-stage synchronizer:
- Eliminates metastability
- Adds 2 clock cycles latency
- Safe for async input signals

## Timing

- Output: Data appears on pin after 1 clock cycle
- Input: Data available after 2 clock cycles (synchronization)

## Use Cases

- LED control
- Button/switch inputs
- General purpose I/O expansion
- Bit-banging protocols
- Debug signals

## Resource Usage

- Logic: ~3 LUTs per pin
- Registers: ~4 registers per pin
- No memory
