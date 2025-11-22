# UART (Universal Asynchronous Receiver/Transmitter)

Complete UART implementation with configurable baud rate.

## Features

- Configurable clock frequency and baud rate
- 8-bit data, no parity, 1 stop bit (8N1)
- Automatic baud rate generation
- Input synchronization for metastability protection
- Ready/valid handshaking
- Separate TX and RX state machines

## Parameters

- `CLK_FREQ`: System clock frequency in Hz (default: 50 MHz)
- `BAUD_RATE`: Serial baud rate (default: 115200)

## Interface

### TX (Transmit)
- `tx_data[7:0]`: Data to transmit
- `tx_valid`: Assert to start transmission
- `tx_ready`: High when ready to accept new data
- `tx_out`: Serial output

### RX (Receive)
- `rx_in`: Serial input
- `rx_data[7:0]`: Received data
- `rx_valid`: Pulses high when data is valid

## Timing

- Start bit: LOW for one bit period
- Data bits: 8 bits, LSB first
- Stop bit: HIGH for one bit period

## Learning Objectives

- State machine design
- Baud rate generation
- Input synchronization
- Bit-level serial communication
- Handshaking protocols
- Clock domain considerations

## Applications

- Serial console communication
- Sensor interfaces
- Debug interfaces
- Inter-chip communication
