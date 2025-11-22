# SPI Slave IP Core

Full-featured SPI slave supporting all 4 SPI modes.

## Features

- All 4 SPI modes (configurable CPOL and CPHA)
- Configurable data width
- Full-duplex operation
- Input synchronization
- Ready/valid handshaking
- Production-ready design

## SPI Modes

| Mode | CPOL | CPHA | Description |
|------|------|------|-------------|
| 0    | 0    | 0    | Sample on rising, shift on falling |
| 1    | 0    | 1    | Shift on rising, sample on falling |
| 2    | 1    | 0    | Sample on falling, shift on rising |
| 3    | 1    | 1    | Shift on falling, sample on rising |

## Parameters

- `DATA_WIDTH`: Data width in bits (default: 8)

## Interface

### Inputs
- `clk`: System clock (must be faster than SPI clock)
- `rst`: Synchronous reset
- `tx_data[DATA_WIDTH-1:0]`: Data to transmit
- `tx_valid`: Load transmit data
- `cpol`: Clock polarity
- `cpha`: Clock phase
- `sclk`: SPI clock (from master)
- `mosi`: Master Out Slave In
- `cs_n`: Chip select (active low)

### Outputs
- `tx_ready`: Ready for new transmit data
- `rx_data[DATA_WIDTH-1:0]`: Received data
- `rx_valid`: Received data valid
- `miso`: Master In Slave Out

## Usage

```systemverilog
spi_slave #(
    .DATA_WIDTH(8)
) spi_inst (
    .clk(clk),
    .rst(rst),
    .tx_data(data_out),
    .tx_valid(load_tx),
    .tx_ready(ready),
    .rx_data(data_in),
    .rx_valid(rx_done),
    .cpol(1'b0),
    .cpha(1'b0),
    .sclk(spi_sclk),
    .mosi(spi_mosi),
    .miso(spi_miso),
    .cs_n(spi_cs)
);
```

## Operation

1. Load transmit data when `tx_ready` is high
2. When master asserts CS_N (low), transfer begins
3. Data shifted in/out according to mode
4. When transfer complete, `rx_valid` pulses
5. New transmit data can be loaded

## Clock Requirements

System clock must be at least 4× faster than SPI clock for reliable operation.

Recommended: 8× or more for margin.

## Synchronization

All SPI inputs (SCLK, MOSI, CS_N) pass through multi-stage synchronizers for metastability protection.

## Use Cases

- Peripheral devices
- Sensor interfaces
- Memory devices
- ADC/DAC interfaces
- Multi-master systems

## Resource Usage

- Logic: ~100 LUTs
- Registers: ~60
- Scales with DATA_WIDTH
