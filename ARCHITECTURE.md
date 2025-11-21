# ChipForge Architecture

This document describes the high-level architecture of ChipForge.

## Overview

ChipForge is built as a modular, composable system with clear separation of concerns. The architecture follows these principles:

1. **Modularity**: Each major component is an independent crate
2. **Layering**: Higher-level components depend on lower-level ones
3. **Extensibility**: Plugin system for custom functionality
4. **Performance**: Rust for speed and safety, parallel processing where possible
5. **Interoperability**: Standard file formats and APIs

## System Layers

```
┌─────────────────────────────────────────────────────────┐
│                     User Interfaces                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │   CLI    │  │   GUI    │  │  Cloud   │              │
│  │  (Rust)  │  │ (Tauri+  │  │   API    │              │
│  │          │  │  React)  │  │  (REST)  │              │
│  └──────────┘  └──────────┘  └──────────┘              │
└─────────────────────────────────────────────────────────┘
                         │
┌─────────────────────────────────────────────────────────┐
│                    Core EDA Engines                      │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐        │
│  │ Synthesis  │  │ Simulation │  │   Formal   │        │
│  │            │  │            │  │Verification│        │
│  └────────────┘  └────────────┘  └────────────┘        │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐        │
│  │Place&Route │  │ Bitstream  │  │    DFT     │        │
│  │            │  │            │  │            │        │
│  └────────────┘  └────────────┘  └────────────┘        │
└─────────────────────────────────────────────────────────┘
                         │
┌─────────────────────────────────────────────────────────┐
│                  Common Infrastructure                   │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐        │
│  │   Error    │  │   Graph    │  │    Type    │        │
│  │  Handling  │  │ Algorithms │  │   System   │        │
│  └────────────┘  └────────────┘  └────────────┘        │
└─────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Common Library (`core/common`)
**Purpose**: Shared utilities used across all components

**Key Modules**:
- Error handling and Result types
- Source location tracking (file, line, column)
- Identifier interning (string deduplication)
- Type system (logic, bit vectors, arrays, structs)
- Graph algorithms (for netlist and dataflow analysis)

**Dependencies**: Minimal (serde, petgraph, thiserror)

### 2. Synthesis Engine (`core/synthesis`)
**Purpose**: Convert HDL to optimized netlist

**Pipeline**:
```
HDL Source → Parser → AST → Elaboration → IR → Optimization → Netlist
```

**Key Components**:
- **Frontend**: Language-specific parsers (Verilog, SystemVerilog, VHDL)
- **AST**: Abstract Syntax Tree representation
- **Elaboration**: Resolve hierarchy, parameters, generate blocks
- **IR**: Intermediate Representation (technology-independent)
- **Optimization**: Dead code elimination, constant folding, etc.
- **Technology Mapping**: Map to target FPGA/ASIC primitives

**Current Status**: Basic Verilog lexer implemented

### 3. Simulation Engine (`core/simulation`)
**Purpose**: Simulate HDL designs

**Simulation Types**:
- Event-driven (delta cycles, 4-state logic)
- Cycle-accurate (fast, 2-state)
- Transaction-level (TLM for system-level)

**Key Components**:
- Event queue and scheduler
- Process management
- Waveform generation (VCD, FST)
- Coverage collection
- DPI/VPI interfaces

**Current Status**: Stub only

### 4. Formal Verification (`core/formal`)
**Purpose**: Mathematically prove properties

**Techniques**:
- Bounded Model Checking (BMC)
- K-induction
- IC3/PDR
- Equivalence checking
- SAT/SMT solver integration

**Current Status**: Stub only

### 5. Place and Route (`core/place_and_route`)
**Purpose**: Physical implementation

**Components**:
- Packing (primitives → logic blocks)
- Placement (assign locations)
- Routing (connect with wires)
- Timing analysis (static timing analysis)

**Current Status**: Stub only

### 6. RISC-V Ecosystem (`riscv/`)
**Purpose**: Easy SoC creation

**Components**:
- Core library (PicoRV32, VexRiscv, etc.)
- SoC builder (GUI for assembly)
- Bus generators (AXI, Wishbone, etc.)
- Peripheral library (UART, SPI, GPIO, etc.)
- Debug infrastructure (JTAG, GDB)

**Current Status**: Stub only

### 7. CLI (`cli/`)
**Purpose**: Command-line interface

**Commands**:
- `new`: Create project
- `synthesize`: Run synthesis
- `simulate`: Run simulation
- `analyze`: Lint, timing, power analysis
- `program`: Upload bitstream to FPGA

**Current Status**: Basic commands implemented

## Data Flow

### Synthesis Flow
```
Verilog Files
     │
     ├─> Parser ──> AST
     │
     ├─> Elaboration ──> Hierarchical IR
     │
     ├─> Optimization ──> Optimized IR
     │
     ├─> Tech Mapping ──> Netlist (LUTs, FFs, etc.)
     │
     └─> Bitstream Generator ──> FPGA Configuration
```

### Simulation Flow
```
Netlist + Testbench
     │
     ├─> Elaboration ──> Flat Design
     │
     ├─> Event Queue ──> Simulation Engine
     │
     ├─> Waveform Writer ──> VCD/FST File
     │
     └─> Coverage Tracker ──> Coverage Report
```

## Design Patterns

### 1. Builder Pattern
Used for complex configuration:
```rust
let config = SynthesisConfig::builder()
    .target("xilinx/artix7")
    .optimization_level(3)
    .top_module("main")
    .build()?;
```

### 2. Visitor Pattern
Used for traversing AST and IR:
```rust
struct OptimizationVisitor;
impl Visitor for OptimizationVisitor {
    fn visit_module(&mut self, module: &mut Module) { ... }
    fn visit_statement(&mut self, stmt: &mut Statement) { ... }
}
```

### 3. Pass Manager
Optimization passes run in sequence:
```rust
let mut pass_manager = PassManager::new();
pass_manager.add(ConstantPropagation);
pass_manager.add(DeadCodeElimination);
pass_manager.run(&mut design)?;
```

### 4. Plugin System
Extend functionality via dynamic loading:
```rust
let plugin = PluginManager::load("custom_synthesis_pass.so")?;
plugin.register_pass(&mut pass_manager);
```

## Performance Considerations

### Parallelism
- Use Rayon for data parallelism (e.g., parallel module processing)
- Async/await (Tokio) for I/O-bound operations (file reading, network)
- Multi-threaded compilation units

### Memory Management
- Identifier interning reduces string duplication
- Arena allocation for AST nodes (bump allocator)
- Streaming processing for large files
- Incremental compilation (reuse previous results)

### Benchmarking
- Criterion for microbenchmarks
- Realistic designs for system benchmarks
- Comparison with Yosys, Verilator, etc.

## Testing Strategy

### Unit Tests
- Each module has comprehensive unit tests
- Property-based testing with proptest
- Edge cases and error conditions

### Integration Tests
- Full synthesis flow on small designs
- Simulation correctness (golden reference)
- End-to-end CLI tests

### Golden Tests
- Known-good designs with expected outputs
- Regression detection
- Performance tracking over time

## Future Architecture

### Distributed System
```
Load Balancer
     │
     ├─> Synthesis Worker 1
     ├─> Synthesis Worker 2
     ├─> Synthesis Worker N
     │
     └─> Result Aggregator
```

### AI Integration
```
User Code
     │
     ├─> AI Copilot ──> Suggestions
     │
     ├─> Bug Detector ──> Warnings
     │
     └─> Optimizer ──> Better Code
```

## Contributing

When adding new features:
1. Place in appropriate layer (common vs. core vs. UI)
2. Minimize dependencies
3. Write comprehensive tests
4. Document public APIs
5. Update this architecture document

## References
- [Yosys Manual](http://yosyshq.net/yosys/documentation.html)
- [Verilator Documentation](https://verilator.org/guide/latest/)
- [LLVM Architecture](https://llvm.org/docs/GettingStarted.html)
