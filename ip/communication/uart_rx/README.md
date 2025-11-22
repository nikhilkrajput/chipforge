# UART Receiver IP Core

Production-ready UART receiver with input synchronization and error detection.

## Features

- Configurable clock frequency and baud rate
- 8N1 format (8 data bits, no parity, 1 stop bit)
- Input synchronization (2-stage)
- Start bit validation
- Framing error detection
- Mid-bit sampling for reliability
- Low resource utilization

## Parameters

- `CLK_FREQ`: System clock frequency in Hz (default: 100 MHz)
- `BAUD_RATE`: UART baud rate (default: 115200)

## Interface

### Inputs
- `clk`: System clock
- `rst`: Synchronous reset (active high)
- `uart_rx`: UART serial input
- `rx_ready`: Receiver ready for new data

### Outputs
- `rx_data[7:0]`: Received data byte
- `rx_valid`: Data valid signal (pulsed)

## Usage

```systemverilog
uart_rx #(
    .CLK_FREQ(50_000_000),
    .BAUD_RATE(9600)
) uart_rx_inst (
    .clk(clk),
    .rst(rst),
    .uart_rx(rx),
    .rx_data(data),
    .rx_valid(valid),
    .rx_ready(ready)
);
```

## Error Handling

- **False start bit**: Returns to IDLE if start bit not stable
- **Framing error**: Discards data if stop bit not detected
- **Input glitches**: Eliminated by 2-stage synchronizer

## Timing

- Samples at middle of bit period for maximum noise immunity
- Automatically calculates clock divisor based on CLK_FREQ and BAUD_RATE
- Supports any standard baud rate
- Minimum clock frequency: 16 × BAUD_RATE (recommended)

## Resource Usage

- ~60 LUTs
- ~35 registers
- No block RAM
