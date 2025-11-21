# ChipForge IP Library

A collection of reusable, verified IP cores for hardware design.

## Categories

### Interfaces
- **AXI** - AXI4, AXI4-Lite, AXI4-Stream interfaces
- **AHB** - AMBA AHB-Lite interfaces
- **APB** - AMBA APB interfaces
- **Wishbone** - Wishbone bus interfaces

### Memory
- **FIFO** - Synchronous and asynchronous FIFOs
- **RAM** - Single-port, dual-port, true dual-port RAM
- **Cache** - Direct-mapped, set-associative caches

### Communication
- **UART** - Universal Asynchronous Receiver-Transmitter
- **SPI** - Serial Peripheral Interface (master/slave)
- **I2C** - Inter-Integrated Circuit (master/slave)
- **Ethernet** - 10/100/1000 MAC
- **PCIe** - PCI Express controller
- **USB** - USB 2.0/3.0 controller

### DSP
- **FIR Filter** - Finite Impulse Response filters
- **FFT** - Fast Fourier Transform
- **CORDIC** - Coordinate Rotation Digital Computer

### ML
- **Systolic Array** - Matrix multiplication engines
- **Activation Functions** - ReLU, Sigmoid, Tanh, Softmax

### Security
- **AES** - Advanced Encryption Standard
- **SHA** - Secure Hash Algorithm
- **RSA** - RSA encryption/decryption
- **TRNG** - True Random Number Generator

## Using IP Cores

### In ChipForge Projects

```bash
chipforge add-ip uart/uart_16550
```

### In Verilog/SystemVerilog

```systemverilog
`include "chipforge_ip/uart/uart_16550.sv"

module my_design (
    input wire clk,
    input wire reset,
    input wire rx,
    output wire tx
);

    uart_16550 uart_inst (
        .clk(clk),
        .reset(reset),
        .rx(rx),
        .tx(tx),
        // ... other ports
    );

endmodule
```

## Quality Standards

All IP cores in this library:
- ✓ Are fully verified with testbenches
- ✓ Include comprehensive documentation
- ✓ Support multiple FPGA vendors
- ✓ Follow consistent coding style
- ✓ Are MIT/Apache-2.0 licensed

## Contributing IP

See [Contributing Guide](../CONTRIBUTING.md#ip-contributions) for guidelines on submitting new IP cores.

## Testing

Each IP core includes:
- Unit tests
- Integration tests
- Timing analysis results
- Resource utilization data
