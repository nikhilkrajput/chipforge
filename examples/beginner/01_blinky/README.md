# LED Blinker Example

A simple LED blinker that demonstrates basic digital design concepts.

## What This Does

This design toggles an LED on and off at approximately 1Hz using a 50MHz clock.

## Concepts Demonstrated

- Clock domain logic
- Counter design
- Reset handling
- Output control

## Files

- `blinky.v` - Main Verilog module
- `tb_blinky.sv` - SystemVerilog testbench
- `constraints.xdc` - Pin constraints for FPGA (Xilinx)

## Running with ChipForge

### Synthesis

```bash
chipforge synthesize blinky.v --target xilinx/artix7
```

### Simulation

```bash
chipforge simulate tb_blinky.sv blinky.v --wave blinky.vcd
```

### View Waveforms

```bash
chipforge wave blinky.vcd
```

## Understanding the Code

### Clock Divider

The counter divides the 50MHz clock:
```verilog
reg [24:0] counter;  // 25-bit counter
```

- Maximum count: 2^25 = 33,554,432
- At 50MHz: 33.5M / 50M ≈ 0.67 seconds
- LED toggles every 0.67s → 1.5Hz blink rate

### Reset Logic

```verilog
if (reset) begin
    counter <= 25'h0;
    led <= 1'b0;
end
```

Active-high reset clears the counter and turns off the LED.

## Exercises

1. **Modify Blink Rate**: Change the counter size to blink faster or slower
2. **Add Button**: Add a button input to pause/resume blinking
3. **Multiple LEDs**: Extend to control 4 LEDs with different patterns

## Next Steps

- [02_counter](../02_counter/) - Build a 4-bit counter with display
- [03_seven_segment](../03_seven_segment/) - Drive a 7-segment display
