# 8-Bit Counter Example

A simple parameterized up-counter with enable and synchronous reset.

## Features

- Parameterized width (default 8 bits)
- Synchronous reset
- Enable signal for count control
- Wraps around at maximum value

## Files

- `counter.v` - Counter module implementation
- `tb_counter.sv` - SystemVerilog testbench
- `README.md` - This file

## Learning Objectives

- Understanding sequential logic
- Using parameters for reusability
- Implementing synchronous reset
- Creating enable logic
- Writing testbenches with monitors

## Simulation

```bash
chipforge simulate tb_counter.sv counter.v
```

## Waveform Analysis

The testbench generates a VCD file (`counter.vcd`) that can be viewed with GTKWave:

```bash
gtkwave counter.vcd
```

## Exercises

1. Modify the counter to count down instead of up
2. Add a load signal to preset the counter value
3. Create an up/down counter with direction control
4. Implement a modulo-N counter (resets at a specific value)
5. Add overflow and underflow flags
