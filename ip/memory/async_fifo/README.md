# Asynchronous FIFO IP Core

Clock domain crossing FIFO using Gray code pointers for safe asynchronous operation.

## Features

- Safe clock domain crossing
- Gray code pointer synchronization
- Configurable data width and depth
- Full/empty flags
- Almost full/almost empty thresholds
- Word count in each domain
- Production-ready metastability handling

## Gray Code Synchronization

Uses Gray code for pointer synchronization because:
- Only one bit changes at a time
- Eliminates multi-bit transition glitches
- Safe for asynchronous clock domain crossing
- Industry-standard CDC technique

## Parameters

- `DATA_WIDTH`: Data width in bits (default: 8)
- `DEPTH`: FIFO depth (must be power of 2)
- `ALMOST_FULL_THRESHOLD`: Almost full level
- `ALMOST_EMPTY_THRESHOLD`: Almost empty level

## Interface

### Write Clock Domain
- `wr_clk`: Write clock
- `wr_rst`: Write reset
- `wr_data[DATA_WIDTH-1:0]`: Write data
- `wr_en`: Write enable
- `full`: FIFO full flag
- `almost_full`: Almost full flag
- `wr_count`: Words in FIFO (write domain)

### Read Clock Domain
- `rd_clk`: Read clock
- `rd_rst`: Read reset
- `rd_data[DATA_WIDTH-1:0]`: Read data
- `rd_en`: Read enable
- `empty`: FIFO empty flag
- `almost_empty`: Almost empty flag
- `rd_count`: Words in FIFO (read domain)

## Usage

```systemverilog
async_fifo #(
    .DATA_WIDTH(32),
    .DEPTH(16),
    .ALMOST_FULL_THRESHOLD(14),
    .ALMOST_EMPTY_THRESHOLD(2)
) fifo_inst (
    .wr_clk(wr_clk),
    .wr_rst(wr_rst),
    .wr_data(wr_data),
    .wr_en(wr_en),
    .full(full),
    .almost_full(almost_full),
    .wr_count(wr_count),

    .rd_clk(rd_clk),
    .rd_rst(rd_rst),
    .rd_data(rd_data),
    .rd_en(rd_en),
    .empty(empty),
    .almost_empty(almost_empty),
    .rd_count(rd_count)
);
```

## Timing Constraints

**CRITICAL**: Add timing constraints to allow metastability:

```tcl
set_max_delay -from [get_cells -hier *wr_ptr_gray*] \
              -to [get_cells -hier *wr_ptr_gray_sync1*] \
              -datapath_only [get_property PERIOD [get_clocks rd_clk]]

set_max_delay -from [get_cells -hier *rd_ptr_gray*] \
              -to [get_cells -hier *rd_ptr_gray_sync1*] \
              -datapath_only [get_property PERIOD [get_clocks wr_clk]]
```

## Latency

- Write to read: 2-3 read clock cycles (due to synchronization)
- Read to write status update: 2-3 write clock cycles

## Use Cases

- Clock domain crossing
- Rate matching between different clock domains
- Asynchronous interfaces
- Network packet buffers

## Resource Usage

- Logic: ~150 LUTs
- Registers: ~50
- Memory: DATA_WIDTH × DEPTH bits
