# I2C Master IP Core

Full-featured I2C master controller supporting standard and fast modes.

## Features

- Standard mode (100 kHz) and fast mode (400 kHz) support
- Configurable clock frequency
- 7-bit addressing
- Read and write operations
- ACK/NAK detection
- Proper start and stop conditions
- Tristate I/O handling
- Error reporting

## Parameters

- `CLK_FREQ`: System clock frequency in Hz (default: 100 MHz)
- `I2C_FREQ`: I2C clock frequency in Hz (default: 100 kHz)

## Interface

### Inputs
- `clk`: System clock
- `rst`: Synchronous reset
- `slave_addr[6:0]`: 7-bit slave address
- `rw`: Read/write bit (0=write, 1=read)
- `tx_data[7:0]`: Data to transmit
- `tx_valid`: Start transaction

### Outputs
- `tx_ready`: Ready for new transaction
- `rx_data[7:0]`: Received data
- `rx_valid`: Received data valid
- `busy`: Transaction in progress
- `ack_error`: NAK received from slave

### Inouts
- `sda`: I2C data line (bidirectional)
- `scl`: I2C clock line (bidirectional)

## Usage

```systemverilog
i2c_master #(
    .CLK_FREQ(50_000_000),
    .I2C_FREQ(100_000)
) i2c_inst (
    .clk(clk),
    .rst(rst),
    .slave_addr(7'h50),
    .rw(1'b0),
    .tx_data(data),
    .tx_valid(start),
    .tx_ready(ready),
    .rx_data(rx_data),
    .rx_valid(rx_done),
    .busy(busy),
    .ack_error(error),
    .sda(i2c_sda),
    .scl(i2c_scl)
);
```

## Protocol

### Write Transaction
1. START condition
2. Send slave address + W bit
3. Wait for ACK
4. Send data byte
5. Wait for ACK
6. STOP condition

### Read Transaction
1. START condition
2. Send slave address + R bit
3. Wait for ACK
4. Read data byte
5. Send ACK/NAK
6. STOP condition

## External Requirements

- External pull-up resistors required on SDA and SCL (typically 4.7kΩ)
- For fast mode (400 kHz), use smaller resistors (e.g., 2.2kΩ)

## Resource Usage

- ~120 LUTs
- ~60 registers
- No block RAM
