# Pipelined Multiplier IP Core

High-performance multiplier with configurable pipeline depth.

## Features

- Configurable operand width
- Configurable pipeline stages
- Full-width result (2×WIDTH)
- Valid signal propagation
- Uses DSP blocks on FPGA
- High Fmax capability

## Parameters

- `WIDTH`: Operand width in bits (default: 32)
- `PIPELINE_STAGES`: Number of pipeline stages (default: 3)

## Interface

### Inputs
- `clk`: Clock
- `rst`: Reset
- `a[WIDTH-1:0]`: Operand A
- `b[WIDTH-1:0]`: Operand B
- `valid_in`: Input data valid

### Outputs
- `product[2*WIDTH-1:0]`: Result (A × B)
- `valid_out`: Output data valid

## Usage

```systemverilog
multiplier #(
    .WIDTH(32),
    .PIPELINE_STAGES(3)
) mult_inst (
    .clk(clk),
    .rst(rst),
    .a(operand_a),
    .b(operand_b),
    .valid_in(start),
    .product(result),
    .valid_out(done)
);
```

## Latency

- Fixed latency: PIPELINE_STAGES clock cycles
- Throughput: 1 operation per clock cycle
- Valid signal tracks data through pipeline

## Performance vs Resources

| Stages | Fmax | Resources |
|--------|------|-----------|
| 1      | Low  | Minimal   |
| 2-3    | Med  | Moderate  |
| 4+     | High | More regs |

## Resource Usage

- DSP blocks: Depends on WIDTH and FPGA architecture
- Registers: PIPELINE_STAGES × 2×WIDTH
- Logic: Minimal

Typical for 32-bit:
- Xilinx: 4 DSP48 blocks
- Intel: 2-4 DSP blocks
