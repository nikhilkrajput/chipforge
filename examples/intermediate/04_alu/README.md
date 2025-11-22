# Arithmetic Logic Unit (ALU)

A complete ALU supporting arithmetic, logic, and shift operations.

## Features

- Parameterized data width (default 32-bit)
- 12 operations: ADD, SUB, AND, OR, XOR, NOR, NAND, SLL, SRL, SRA, SLT, SLTU
- Status flags: zero, overflow, carry
- Signed and unsigned comparisons
- Arithmetic and logical shifts

## Operations

- **Arithmetic**: ADD, SUB with overflow detection
- **Logic**: AND, OR, XOR, NOR, NAND
- **Shifts**: Left logical, right logical, right arithmetic
- **Compare**: Signed and unsigned less-than

## Use Cases

- CPU datapath
- Integer arithmetic operations
- Bitwise manipulation
- Comparison operations

## Learning Objectives

- ALU design fundamentals
- Operation encoding
- Flag generation (zero, carry, overflow)
- Signed vs unsigned operations
- Arithmetic vs logical shifts
- RISC-V/MIPS-style ALU architecture
