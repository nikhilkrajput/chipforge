# Synchronous FIFO IP Core

Production-ready synchronous FIFO with comprehensive status flags.

## Features

- Single clock domain
- Configurable data width and depth
- Full/empty flags
- Almost full/almost empty thresholds
- Word count output
- Simultaneous read/write support
- Low latency

## Parameters

- `DATA_WIDTH`: Data width in bits (default: 8)
- `DEPTH`: FIFO depth in words (must be power of 2)
- `ALMOST_FULL_THRESHOLD`: Almost full level (default: DEPTH-2)
- `ALMOST_EMPTY_THRESHOLD`: Almost empty level (default: 2)

## Interface

### Inputs
- `clk`: Clock
- `rst`: Synchronous reset
- `wr_data[DATA_WIDTH-1:0]`: Write data
- `wr_en`: Write enable
- `rd_en`: Read enable

### Outputs
- `rd_data[DATA_WIDTH-1:0]`: Read data
- `full`: FIFO is full
- `almost_full`: FIFO level >= threshold
- `empty`: FIFO is empty
- `almost_empty`: FIFO level <= threshold
- `count`: Number of words in FIFO

## Usage

```systemverilog
sync_fifo #(
    .DATA_WIDTH(32),
    .DEPTH(16),
    .ALMOST_FULL_THRESHOLD(14),
    .ALMOST_EMPTY_THRESHOLD(2)
) fifo_inst (
    .clk(clk),
    .rst(rst),
    .wr_data(wr_data),
    .wr_en(wr_en),
    .full(full),
    .almost_full(almost_full),
    .rd_data(rd_data),
    .rd_en(rd_en),
    .empty(empty),
    .almost_empty(almost_empty),
    .count(word_count)
);
```

## Write Operation

```systemverilog
if (!full) begin
    wr_data <= data;
    wr_en <= 1'b1;
end
```

## Read Operation

```systemverilog
if (!empty) begin
    rd_en <= 1'b1;
    // data available next cycle in rd_data
end
```

## Simultaneous Read/Write

The FIFO supports simultaneous read and write operations when:
- Not full (for write)
- Not empty (for read)

## Timing

- Write: Data stored on rising edge
- Read: Data available next clock cycle (registered output)
- Latency: 1 clock cycle

## Almost Full/Empty

Use almost_full/almost_empty for flow control:

### Producer Side
```systemverilog
if (almost_full) begin
    // Slow down or stop writing
end
```

### Consumer Side
```systemverilog
if (almost_empty) begin
    // Request more data
end
```

## Use Cases

- Data buffering
- Rate matching
- Pipeline decoupling
- Burst handling
- Packet buffering

## Resource Usage

- Logic: ~50 LUTs
- Registers: ~(ADDR_WIDTH + 1) × 2
- Memory: DATA_WIDTH × DEPTH bits
- Infers block RAM on FPGA

## Design Notes

- Pointers use extra bit for full/empty distinction
- Memory uses power-of-2 depth for efficiency
- Gray code not needed (single clock domain)
