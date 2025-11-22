# Edge Detector IP Core

Synchronous edge detection with configurable edge selection.

## Features

- Rising edge detection
- Falling edge detection
- Both edge detection
- Single cycle pulse output
- Configurable via parameters
- Low resource usage

## Parameters

- `DETECT_RISING`: Enable rising edge detection (default: 1)
- `DETECT_FALLING`: Enable falling edge detection (default: 1)

## Interface

### Inputs
- `clk`: Clock
- `rst`: Reset
- `signal_in`: Input signal to monitor

### Outputs
- `edge_detected`: Any edge detected (OR of rising and falling)
- `rising_edge`: Rising edge detected (single cycle pulse)
- `falling_edge`: Falling edge detected (single cycle pulse)

## Usage

### Detect Rising Edge Only
```systemverilog
edge_detector #(
    .DETECT_RISING(1),
    .DETECT_FALLING(0)
) edge_det (
    .clk(clk),
    .rst(rst),
    .signal_in(signal),
    .edge_detected(edge),
    .rising_edge(rising),
    .falling_edge(falling)
);
```

### Detect Both Edges
```systemverilog
edge_detector #(
    .DETECT_RISING(1),
    .DETECT_FALLING(1)
) edge_det (
    .clk(clk),
    .rst(rst),
    .signal_in(signal),
    .edge_detected(edge),
    .rising_edge(rising),
    .falling_edge(falling)
);
```

## Timing Diagram

```
signal_in:    __/‾‾‾‾‾‾‾‾\____
rising_edge:  __/‾\___________
falling_edge: ____________/‾\__
edge_detected:__/‾\_______/‾\__
```

## Output Pulse

- All outputs are single-cycle pulses
- Pulses occur one cycle after edge
- Multiple outputs can assert simultaneously if configured

## Use Cases

- Button press detection
- Protocol start/stop detection
- Event triggering
- Interrupt generation
- Clock domain crossing detection
- Handshake protocols

## Resource Usage

- Logic: ~10 LUTs
- Registers: 5
- No memory

Very small, often optimized to just a few gates.
