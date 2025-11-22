# UART Transmitter IP Core

Production-ready UART transmitter with configurable baud rate.

## Features

- Configurable clock frequency and baud rate
- 8N1 format (8 data bits, no parity, 1 stop bit)
- Ready/valid handshaking
- Automatic baud rate calculation
- Low resource utilization
- Tested on FPGA

## Parameters

- `CLK_FREQ`: System clock frequency in Hz (default: 100 MHz)
- `BAUD_RATE`: UART baud rate (default: 115200)

## Interface

### Inputs
- `clk`: System clock
- `rst`: Synchronous reset (active high)
- `tx_data[7:0]`: Data byte to transmit
- `tx_valid`: Data valid signal

### Outputs
- `tx_ready`: Ready to accept new data
- `uart_tx`: UART serial output

## Usage

```systemverilog
uart_tx #(
    .CLK_FREQ(50_000_000),
    .BAUD_RATE(9600)
) uart_tx_inst (
    .clk(clk),
    .rst(rst),
    .tx_data(data),
    .tx_valid(valid),
    .tx_ready(ready),
    .uart_tx(tx)
);
```

## Timing

- Automatically calculates clock divisor based on CLK_FREQ and BAUD_RATE
- Supports any standard baud rate (9600, 19200, 38400, 57600, 115200, etc.)
- Minimum clock frequency: 16 × BAUD_RATE (recommended)

## Resource Usage

- ~50 LUTs
- ~30 registers
- No block RAM
