# ChipForge Simulation Examples

This directory contains working examples demonstrating the ChipForge simulation engine capabilities.

## Examples

### 1. D Flip-Flop (`dff.rs`)

A sequential logic example demonstrating:
- Rising edge clock detection
- State storage
- Process-based modeling
- VCD waveform generation

**Run:**
```bash
cargo run --bin dff
gtkwave dff.vcd
```

### 2. 4-bit Counter (`counter.rs`)

A more complex sequential circuit showing:
- Multi-bit signals (4-bit bus)
- Arithmetic operations
- Synchronous reset
- Enable control
- Wrap-around behavior

**Run:**
```bash
cargo run --bin counter
gtkwave counter.vcd
```

### 3. 8-bit ALU (`alu.rs`)

A combinational logic example featuring:
- 8-bit arithmetic and logic operations (ADD, SUB, AND, OR, XOR, NOT, SHL, SHR)
- Operation selection via 3-bit opcode
- Status flags (zero, carry, negative)
- No clock required (pure combinational)

**Run:**
```bash
cargo run --bin alu
gtkwave alu.vcd
```

## Building All Examples

```bash
cargo build --release -p chipforge-simulation-examples
```

## Output Files

Each example generates a VCD (Value Change Dump) file that can be viewed with waveform viewers like:
- GTKWave
- Surfer
- WaveTrace

## Key Concepts Demonstrated

1. **Event-Driven Simulation**: Discrete event scheduling and processing
2. **Delta Cycles**: Zero-delay signal propagation within a time step
3. **4-State Logic**: Support for 0, 1, X (unknown), and Z (high-impedance)
4. **Process Sensitivity**: Level-sensitive and edge-sensitive processes
5. **VCD Generation**: Industry-standard waveform format output

## Next Steps

These examples demonstrate the core simulation engine. Future examples will include:
- Verilog module instantiation
- Memory models (RAM/ROM)
- FIFO buffers
- UART communication
- SPI/I2C protocols
