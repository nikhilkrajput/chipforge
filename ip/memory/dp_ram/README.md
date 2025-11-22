# Dual-Port RAM IP Core

True dual-port RAM with independent read/write on both ports.

## Features

- Two independent ports (A and B)
- Separate clocks for each port
- Simultaneous read/write on both ports
- Configurable data width and depth
- Infers block RAM on FPGA
- Zero initialization

## Parameters

- `DATA_WIDTH`: Data width in bits (default: 32)
- `ADDR_WIDTH`: Address width in bits (default: 10)

## Interface

### Port A
- `clk_a`: Port A clock
- `addr_a[ADDR_WIDTH-1:0]`: Port A address
- `wr_data_a[DATA_WIDTH-1:0]`: Port A write data
- `wr_en_a`: Port A write enable
- `rd_en_a`: Port A read enable
- `rd_data_a[DATA_WIDTH-1:0]`: Port A read data

### Port B
- `clk_b`: Port B clock
- `addr_b[ADDR_WIDTH-1:0]`: Port B address
- `wr_data_b[DATA_WIDTH-1:0]`: Port B write data
- `wr_en_b`: Port B write enable
- `rd_en_b`: Port B read enable
- `rd_data_b[DATA_WIDTH-1:0]`: Port B read data

## Usage

```systemverilog
dp_ram #(
    .DATA_WIDTH(32),
    .ADDR_WIDTH(12)
) ram_inst (
    .clk_a(clk_a),
    .addr_a(addr_a),
    .wr_data_a(wr_data_a),
    .wr_en_a(wr_en_a),
    .rd_en_a(rd_en_a),
    .rd_data_a(rd_data_a),

    .clk_b(clk_b),
    .addr_b(addr_b),
    .wr_data_b(wr_data_b),
    .wr_en_b(wr_en_b),
    .rd_en_b(rd_en_b),
    .rd_data_b(rd_data_b)
);
```

## Collision Handling

When both ports access the same address simultaneously:
- **Both writing**: Port B write takes precedence
- **One writing, one reading**: New data may or may not be read (undefined)
- **Both reading**: No collision

For deterministic behavior, avoid simultaneous access to same address.

## Use Cases

- Multi-processor systems
- Video frame buffers
- Network packet buffers
- Cache implementations

## Resource Usage

- Block RAM: DATA_WIDTH × 2^ADDR_WIDTH bits
- Logic: Minimal
