# Finite State Machine - Traffic Light Controller

A traffic light controller demonstrating proper FSM design patterns.

## Features

- 4-state FSM (main green, main yellow, side green, side yellow)
- Sensor-based state transitions
- Configurable timing parameters
- Separate state and output logic
- Timer-based state transitions

## FSM Design Pattern

This example demonstrates best practices for FSM design:
- **State register**: Sequential logic for state storage
- **Next state logic**: Combinational logic for transitions
- **Output logic**: Combinational or registered outputs
- **Timer integration**: State duration control

## States

1. **MAIN_GREEN**: Main street has green light
2. **MAIN_YELLOW**: Main street transitioning to red
3. **SIDE_GREEN**: Side street has green light
4. **SIDE_YELLOW**: Side street transitioning to red

## Use Cases

- Traffic control systems
- General FSM learning
- State machine design patterns
- Timer-based state transitions
- Sensor-driven control logic

## Learning Objectives

- FSM design methodology
- State encoding strategies
- Separation of state and output logic
- Timer integration in FSMs
- Mealy vs Moore machine patterns
- Real-world control applications
