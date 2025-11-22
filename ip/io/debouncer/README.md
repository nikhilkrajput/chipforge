# Debouncer IP Core

Button/switch debouncer with configurable timing.

## Features

- Configurable debounce time
- Input synchronization (2-stage)
- Handles both rising and falling edges
- Eliminates mechanical bounce
- Clean digital output

## Parameters

- `CLK_FREQ`: System clock frequency in Hz (default: 100 MHz)
- `DEBOUNCE_TIME_MS`: Debounce time in milliseconds (default: 20ms)

## Interface

### Inputs
- `clk`: Clock
- `rst`: Reset
- `button_in`: Raw button input (async)

### Outputs
- `button_out`: Debounced button output (sync)

## Usage

```systemverilog
debouncer #(
    .CLK_FREQ(50_000_000),
    .DEBOUNCE_TIME_MS(20)
) deb_inst (
    .clk(clk),
    .rst(rst),
    .button_in(button),
    .button_out(button_clean)
);
```

## How It Works

1. Input passes through 2-stage synchronizer
2. When input changes, counter starts
3. If input stable for DEBOUNCE_TIME_MS, output changes
4. If input bounces, counter resets
5. Only stable transitions propagate to output

## Timing Diagram

```
button_in:  ___/‾‾\_/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
                 ^bounce     ^stable
button_out: _________________/‾‾‾‾‾‾‾‾‾‾
                             ^debounced
                             |<-20ms->|
```

## Debounce Time Selection

| Application | Recommended Time |
|-------------|------------------|
| Mechanical buttons | 10-20 ms |
| Mechanical switches | 5-10 ms |
| Rotary encoders | 1-5 ms |
| Reed switches | 5-10 ms |

## Use Cases

- Push button inputs
- Toggle switches
- Rotary encoder inputs
- Mechanical sensor inputs

## Resource Usage

- Logic: ~30 LUTs
- Registers: ~COUNTER_WIDTH + 5
- No memory

For 20ms @ 100MHz:
- Counter: 21 bits
- Total: ~25 registers
