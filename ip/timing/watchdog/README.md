# Watchdog Timer IP Core

System watchdog for detecting software hangs and crashes.

## Features

- Configurable timeout value
- Enable/disable control
- Kick (pet) mechanism to reset
- Timeout flag and reset output
- Stops at timeout (doesn't wrap)
- Default timeout parameter

## Parameters

- `WIDTH`: Counter width in bits (default: 32)
- `DEFAULT_TIMEOUT`: Default timeout value (default: 1000000)

## Interface

### Inputs
- `clk`: Clock
- `rst`: System reset
- `timeout_value[WIDTH-1:0]`: Watchdog timeout period
- `enable`: Watchdog enable
- `kick`: Kick/pet watchdog (reset counter)

### Outputs
- `timeout`: Timeout occurred (sticky)
- `timeout_rst`: Reset output (can be connected to system reset)

## Usage

```systemverilog
watchdog #(
    .WIDTH(32),
    .DEFAULT_TIMEOUT(32'd10_000_000)  // 100ms @ 100MHz
) wdt_inst (
    .clk(clk),
    .rst(rst),
    .timeout_value(32'd10_000_000),
    .enable(1'b1),
    .kick(wdt_kick),
    .timeout(wdt_timeout),
    .timeout_rst(wdt_rst)
);

// In system reset logic
assign system_rst = rst | wdt_rst;
```

## Software Integration

### Kick/Pet the Watchdog
```c
#define WDT_KICK_REG 0x40000000

void watchdog_kick(void) {
    *((volatile uint32_t*)WDT_KICK_REG) = 1;
}

void main(void) {
    while(1) {
        // Normal processing
        do_work();

        // Kick watchdog periodically
        watchdog_kick();
    }
}
```

## Operation

1. Counter increments every clock cycle when enabled
2. Software must assert `kick` before counter reaches `timeout_value`
3. `kick` resets counter to 0
4. If counter reaches timeout:
   - `timeout` flag sets (sticky until reset)
   - `timeout_rst` pulses high (can trigger system reset)
   - Counter stops counting

## Timeout Calculation

```
Timeout (seconds) = timeout_value / clock_frequency

Example: 100ms timeout @ 100MHz
timeout_value = 0.1s × 100,000,000 Hz = 10,000,000
```

## Best Practices

- Set timeout 2-3× longer than max loop time
- Kick watchdog in main loop, not in interrupts
- Don't kick inside error handlers
- Test watchdog functionality during development
- Use timeout_rst for actual system reset

## Use Cases

- Embedded system reliability
- Crash detection and recovery
- Hang detection
- Safety-critical systems
- Autonomous operation

## Resource Usage

- Logic: ~40 LUTs (32-bit)
- Registers: ~WIDTH + 3
- No memory
