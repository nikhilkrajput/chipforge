# ChipForge Roadmap

This document outlines the planned development roadmap for ChipForge.

## Version 0.1.0 (Current) ✅
- [x] Project structure and build system
- [x] Core common library
- [x] Basic CLI interface
- [x] Simple Verilog lexer
- [x] Documentation structure
- [x] CI/CD pipeline

## Version 0.2.0 (Q1 2025)
### Synthesis Engine
- [ ] Complete Verilog parser (IEEE 1364-2005)
- [ ] AST construction and elaboration
- [ ] Module hierarchy resolution
- [ ] Constant folding and dead code elimination
- [ ] Basic technology-independent optimization

### Simulation
- [ ] Event-driven simulation kernel
- [ ] 4-state logic simulation (0, 1, X, Z)
- [ ] VCD waveform generation
- [ ] Basic testbench support

### CLI Enhancements
- [ ] Project configuration file (chipforge.toml)
- [ ] Progress reporting for long operations
- [ ] Error message improvements
- [ ] Colored output and formatting

## Version 0.3.0 (Q2 2025)
### Advanced Synthesis
- [ ] SystemVerilog parser (subset)
- [ ] Parameter elaboration
- [ ] Generate blocks
- [ ] Advanced optimization passes
- [ ] Technology mapping for Xilinx and Intel FPGAs

### Formal Verification
- [ ] Bounded model checking (BMC)
- [ ] Assertion support (SVA subset)
- [ ] Counterexample generation
- [ ] Basic equivalence checking

### Place and Route
- [ ] Basic placement engine
- [ ] Simple routing algorithm
- [ ] Timing analysis foundation
- [ ] Bitstream generation for iCE40 FPGAs

## Version 0.4.0 (Q3 2025)
### RISC-V Integration
- [ ] PicoRV32 core integration
- [ ] Basic SoC builder GUI
- [ ] UART, GPIO, Timer peripherals
- [ ] Memory map generation
- [ ] Software toolchain integration (GCC)

### GUI Foundation
- [ ] Tauri-based desktop application
- [ ] Code editor with syntax highlighting
- [ ] Project file browser
- [ ] Build output console
- [ ] Basic waveform viewer

## Version 0.5.0 (Q4 2025)
### AI Features (Phase 1)
- [ ] Code completion (basic)
- [ ] Syntax error suggestions
- [ ] Common pattern detection
- [ ] Documentation generation

### Advanced Simulation
- [ ] UVM/OVM testbench support
- [ ] Coverage collection
- [ ] Assertion-based verification
- [ ] Mixed-language simulation (Verilog + VHDL)

## Version 1.0.0 (Q1 2026)
### Production Ready
- [ ] Comprehensive Verilog and SystemVerilog support
- [ ] Multi-vendor FPGA support (Xilinx, Intel, Lattice, Gowin)
- [ ] Advanced optimization algorithms
- [ ] AI-powered bug detection
- [ ] Cloud build service (beta)
- [ ] Professional documentation and tutorials
- [ ] Performance benchmarks vs. commercial tools

## Version 2.0.0 (Q3 2026)
### Advanced Features
- [ ] High-Level Synthesis (HLS) - C/C++ to RTL
- [ ] ML accelerator design automation
- [ ] Quantum circuit integration
- [ ] Advanced formal verification
- [ ] 3D visualization
- [ ] Real-time collaboration
- [ ] Plugin marketplace

## Long-term Vision (2027+)
### Revolutionary Features
- [ ] Natural language design interface
- [ ] Automated design space exploration
- [ ] Hardware-software co-design
- [ ] Neuromorphic computing support
- [ ] Photonic integration
- [ ] Advanced AI copilot with learning
- [ ] AR/VR design environment
- [ ] Cloud-native development platform

## Community Goals
- Reach 10,000 GitHub stars
- 100+ contributors
- Academic adoption in universities
- Industry partnerships
- Open-source silicon tapeouts using ChipForge

## Performance Targets
- Synthesis: Match or exceed Yosys performance
- Simulation: 10x faster than open-source alternatives
- P&R: Competitive with open-source tools
- Memory: Efficient for designs up to 1M gates
- Scalability: Support for multi-core parallel processing

## Notes
- Dates are approximate and may shift based on community feedback
- Features may be added or reprioritized based on user needs
- We welcome community input on roadmap priorities
- Enterprise features may be developed in parallel

Last updated: November 21, 2024
