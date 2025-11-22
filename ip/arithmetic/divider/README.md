# Integer Divider IP Core

Iterative integer division with quotient and remainder.

## Features

- Non-restoring division algorithm
- Configurable width
- Quotient and remainder outputs
- Division by zero detection
- Multi-cycle operation
- Resource efficient

## Parameters

- `WIDTH`: Operand width in bits (default: 32)

## Interface

### Inputs
- `clk`: Clock
- `rst`: Reset
- `dividend[WIDTH-1:0]`: Dividend (numerator)
- `divisor[WIDTH-1:0]`: Divisor (denominator)
- `start`: Start division

### Outputs
- `quotient[WIDTH-1:0]`: Quotient (dividend / divisor)
- `remainder[WIDTH-1:0]`: Remainder (dividend % divisor)
- `valid`: Result valid
- `div_by_zero`: Division by zero error

## Usage

```systemverilog
divider #(
    .WIDTH(32)
) div_inst (
    .clk(clk),
    .rst(rst),
    .dividend(num),
    .divisor(denom),
    .start(start),
    .quotient(quot),
    .remainder(rem),
    .valid(done),
    .div_by_zero(error)
);
```

## Operation

1. Assert `start` with operands
2. Wait WIDTH clock cycles
3. `valid` asserts with result
4. If `div_by_zero` asserts, quotient = all 1's, remainder = dividend

## Timing

- Latency: WIDTH + 2 clock cycles
- Throughput: 1 operation per (WIDTH + 2) cycles
- Not pipelined

## Division by Zero

When divisor = 0:
- `div_by_zero` flag asserts
- `quotient` = 0xFFFF...
- `remainder` = dividend
- Result available immediately

## Resource Usage

- Logic: ~200 LUTs (32-bit)
- Registers: ~3×WIDTH
- No DSP blocks
- No memory
