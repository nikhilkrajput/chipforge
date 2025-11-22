# Timer/Counter IP Core

Versatile timer with multiple operating modes.

## Features

- Configurable width
- Count up or count down modes
- One-shot or continuous mode
- Configurable period
- Timeout pulse output
- Current count readback
- Running status

## Parameters

- `WIDTH`: Counter width in bits (default: 32)

## Interface

### Inputs
- `clk`: Clock
- `rst`: Reset
- `period[WIDTH-1:0]`: Timer period (counts per timeout)
- `enable`: Timer enable
- `oneshot`: 1=one-shot mode, 0=continuous mode
- `count_up`: 1=count up, 0=count down

### Outputs
- `count[WIDTH-1:0]`: Current counter value
- `timeout`: Timeout pulse (single cycle)
- `running`: Timer is actively counting

## Usage

### Continuous Timer (Periodic)
```systemverilog
timer #(
    .WIDTH(32)
) timer_inst (
    .clk(clk),
    .rst(rst),
    .period(32'd1000),     // 1000 clocks
    .enable(1'b1),
    .oneshot(1'b0),        // Continuous
    .count_up(1'b1),
    .count(count),
    .timeout(tick),
    .running(active)
);
```

### One-Shot Timer
```systemverilog
timer #(
    .WIDTH(16)
) timer_inst (
    .clk(clk),
    .rst(rst),
    .period(16'd5000),
    .enable(start),
    .oneshot(1'b1),        // One-shot
    .count_up(1'b1),
    .count(count),
    .timeout(done),
    .running(busy)
);
```

## Operating Modes

### Count Up + Continuous
- Counter: 0 → period → 0 → period → ...
- Timeout pulses when counter == period

### Count Down + Continuous
- Counter: period → 0 → period → 0 → ...
- Timeout pulses when counter == 0

### Count Up + One-Shot
- Counter: 0 → period → stop
- Single timeout pulse, then stops

### Count Down + One-Shot
- Counter: period → 0 → stop
- Single timeout pulse, then stops

## Restart

To restart a one-shot timer:
1. Deassert `enable`
2. Wait one cycle
3. Assert `enable`

## Use Cases

- Periodic tick generation
- Timeout detection
- Delay generation
- Event scheduling
- Watchdog timers
- Baud rate generation

## Resource Usage

- Logic: ~50 LUTs (32-bit)
- Registers: ~WIDTH + 5
- No memory
- Scales linearly with WIDTH
