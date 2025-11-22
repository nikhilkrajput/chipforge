# SPI Master IP Core

Full-featured SPI master supporting all 4 SPI modes.

## Features

- All 4 SPI modes (configurable CPOL and CPHA)
- Configurable clock divisor
- Configurable data width
- Full-duplex operation
- Ready/valid handshaking
- Automatic chip select control
- Production-ready design

## SPI Modes

| Mode | CPOL | CPHA | Description |
|------|------|------|-------------|
| 0    | 0    | 0    | Sample on rising, shift on falling |
| 1    | 0    | 1    | Shift on rising, sample on falling |
| 2    | 1    | 0    | Sample on falling, shift on rising |
| 3    | 1    | 1    | Shift on falling, sample on rising |

## Parameters

- `CLK_DIV`: Clock divisor (SPI clock = system clock / CLK_DIV)
- `DATA_WIDTH`: Data width in bits (default: 8)

## Interface

### Inputs
- `clk`: System clock
- `rst`: Synchronous reset
- `tx_data[DATA_WIDTH-1:0]`: Data to transmit
- `tx_valid`: Start transmission
- `cpol`: Clock polarity
- `cpha`: Clock phase
- `miso`: Master In Slave Out

### Outputs
- `tx_ready`: Ready for new transaction
- `rx_data[DATA_WIDTH-1:0]`: Received data
- `rx_valid`: Received data valid
- `sclk`: SPI clock
- `mosi`: Master Out Slave In
- `cs_n`: Chip select (active low)

## Usage

```systemverilog
spi_master #(
    .CLK_DIV(8),
    .DATA_WIDTH(8)
) spi_inst (
    .clk(clk),
    .rst(rst),
    .tx_data(data_out),
    .tx_valid(start),
    .tx_ready(ready),
    .rx_data(data_in),
    .rx_valid(done),
    .cpol(1'b0),
    .cpha(1'b0),
    .sclk(spi_clk),
    .mosi(spi_mosi),
    .miso(spi_miso),
    .cs_n(spi_cs)
);
```

## Timing

- SPI clock frequency = System clock / CLK_DIV
- Transaction takes DATA_WIDTH SPI clock cycles
- CS_N asserted one cycle before transfer, deasserted one cycle after

## Resource Usage

- ~80 LUTs
- ~50 registers
- Scales with DATA_WIDTH
