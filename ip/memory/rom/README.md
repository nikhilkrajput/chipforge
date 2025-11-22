# ROM IP Core

Read-only memory with initialization file support.

## Features

- Configurable data width and depth
- Initialization from hex file
- Synchronous read
- Infers block RAM on FPGA
- Default incremental initialization

## Parameters

- `DATA_WIDTH`: Data width in bits (default: 32)
- `ADDR_WIDTH`: Address width in bits (default: 10)
- `INIT_FILE`: Path to initialization file (optional)

## Interface

### Inputs
- `clk`: Clock
- `addr[ADDR_WIDTH-1:0]`: Address
- `rd_en`: Read enable

### Outputs
- `rd_data[DATA_WIDTH-1:0]`: Read data

## Usage

```systemverilog
rom #(
    .DATA_WIDTH(32),
    .ADDR_WIDTH(8),
    .INIT_FILE("bootrom.hex")
) rom_inst (
    .clk(clk),
    .addr(addr),
    .rd_en(rd_en),
    .rd_data(rd_data)
);
```

## Initialization File Format

Use hex format compatible with `$readmemh`:

```
// bootrom.hex
00000093  // addi x1, x0, 0
00100113  // addi x2, x0, 1
002081B3  // add x3, x1, x2
```

## Default Initialization

If no `INIT_FILE` is provided, ROM is initialized with incremental values:
- Address 0: 0
- Address 1: 1
- Address 2: 2
- ...

## Use Cases

- Boot ROM
- Lookup tables
- Constant coefficient storage
- Microcode storage
- Font tables

## Timing

- Data available one clock cycle after address
- Registered output

## Resource Usage

- Block RAM: DATA_WIDTH × 2^ADDR_WIDTH bits
- Logic: Minimal
