# Single-Port RAM IP Core

Parameterized single-port RAM with optional byte-enable support.

## Features

- Configurable data width and depth
- Synchronous read and write
- Byte-enable support for partial writes
- Infers block RAM on FPGA
- Zero initialization

## Parameters

- `DATA_WIDTH`: Data width in bits (default: 32)
- `ADDR_WIDTH`: Address width in bits (default: 10, for 1024 words)
- `BYTE_ENABLE`: Enable byte-wise writes (default: 1)

## Interface

### Inputs
- `clk`: Clock
- `addr[ADDR_WIDTH-1:0]`: Address
- `wr_data[DATA_WIDTH-1:0]`: Write data
- `wr_en[(DATA_WIDTH/8)-1:0]`: Write enable (byte-wise if BYTE_ENABLE=1)
- `rd_en`: Read enable

### Outputs
- `rd_data[DATA_WIDTH-1:0]`: Read data

## Usage

```systemverilog
sp_ram #(
    .DATA_WIDTH(32),
    .ADDR_WIDTH(10),
    .BYTE_ENABLE(1)
) ram_inst (
    .clk(clk),
    .addr(addr),
    .wr_data(wr_data),
    .wr_en(wr_en),
    .rd_en(rd_en),
    .rd_data(rd_data)
);
```

## Timing

- Write: Data written on rising edge of clock
- Read: Data available on next rising edge (registered output)
- Read-during-write: New data forwarded to output

## FPGA Mapping

This design infers block RAM on most FPGAs:
- Xilinx: RAMB18, RAMB36
- Intel: M10K, M20K
- Lattice: EBR

## Resource Usage

- Block RAM: Depends on DATA_WIDTH × 2^ADDR_WIDTH
- Logic: Minimal (~10 LUTs for control)
