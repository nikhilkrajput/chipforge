# PWM Generator IP Core

High-resolution PWM generator with configurable parameters.

## Features

- Configurable counter width (resolution)
- Variable frequency control
- Variable duty cycle control
- Polarity control (active high/low)
- Enable control
- Clean PWM output

## Parameters

- `COUNTER_WIDTH`: Counter width in bits (default: 16)

## Interface

### Inputs
- `clk`: System clock
- `rst`: Reset
- `period[COUNTER_WIDTH-1:0]`: PWM period in clock cycles
- `duty_cycle[COUNTER_WIDTH-1:0]`: Duty cycle value (0 to period)
- `enable`: PWM enable
- `polarity`: Output polarity (0=active high, 1=active low)

### Outputs
- `pwm_out`: PWM output signal

## Usage

```systemverilog
pwm #(
    .COUNTER_WIDTH(16)
) pwm_inst (
    .clk(clk),
    .rst(rst),
    .period(16'd1000),      // 1000 clocks
    .duty_cycle(16'd500),   // 50% duty cycle
    .enable(1'b1),
    .polarity(1'b0),        // Active high
    .pwm_out(pwm_signal)
);
```

## Frequency Calculation

```
PWM_freq = clk_freq / (period + 1)

Example: 100 MHz clock, period = 999
PWM_freq = 100,000,000 / 1000 = 100 kHz
```

## Duty Cycle Calculation

```
Duty_cycle_% = (duty_cycle / period) × 100%

Examples:
- duty_cycle = 0: 0%
- duty_cycle = period/2: 50%
- duty_cycle = period: 100%
```

## Resolution vs Frequency

Higher resolution = lower max frequency

| Width | Levels | Max PWM Freq @ 100MHz |
|-------|--------|----------------------|
| 8-bit | 256    | 390 kHz             |
| 10-bit| 1024   | 97 kHz              |
| 12-bit| 4096   | 24 kHz              |
| 16-bit| 65536  | 1.5 kHz             |

## Polarity

- `polarity = 0`: PWM high during duty cycle
- `polarity = 1`: PWM low during duty cycle (inverted)

## Dynamic Updates

Period and duty_cycle can be changed on-the-fly:
- Changes take effect at start of next PWM cycle
- Glitch-free updates

## Use Cases

- Motor speed control (ESC)
- LED brightness control
- Servo control
- DAC implementation
- Power supply control
- Audio generation
- Heating element control

## Example: Servo Control

```systemverilog
// 50 Hz servo signal (20ms period)
// Pulse width: 1ms (0°) to 2ms (180°)

localparam CLK_FREQ = 50_000_000;  // 50 MHz
localparam PERIOD = CLK_FREQ / 50 - 1;  // 20ms

// Center position (1.5ms pulse)
localparam CENTER = (CLK_FREQ / 1000) * 15 / 10;

pwm #(.COUNTER_WIDTH(20)) servo_pwm (
    .clk(clk),
    .rst(rst),
    .period(PERIOD),
    .duty_cycle(CENTER),
    .enable(1'b1),
    .polarity(1'b0),
    .pwm_out(servo_signal)
);
```

## Resource Usage

- Logic: ~40 LUTs
- Registers: COUNTER_WIDTH + 3
- No memory
- Very efficient

For 16-bit: ~20 registers
