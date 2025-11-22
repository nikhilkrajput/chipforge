# PWM Generator

A parameterized Pulse Width Modulation generator.

## Features

- Configurable resolution (8-bit default)
- Simple counter-based design
- Duty cycle range: 0% to 100%
- Synchronous operation
- Low resource utilization

## Operation

The PWM generator uses a free-running counter:
- Counter increments every clock cycle
- Output is HIGH when counter < duty_cycle
- Output is LOW when counter >= duty_cycle
- Resolution: 2^WIDTH discrete duty cycle levels

## Resolution vs Frequency

- 8-bit: 256 levels, PWM freq = clk/256
- 10-bit: 1024 levels, PWM freq = clk/1024
- 12-bit: 4096 levels, PWM freq = clk/4096

## Use Cases

- Motor speed control
- LED brightness control
- DAC implementation
- Power regulation
- Servo control

## Learning Objectives

- PWM generation fundamentals
- Duty cycle control
- Counter-based timing
- Resolution vs frequency tradeoffs
- Analog control with digital signals
