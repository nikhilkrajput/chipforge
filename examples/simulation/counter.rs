//! 4-bit Counter Simulation Example
//!
//! This example demonstrates:
//! - Multi-bit signals
//! - Arithmetic operations in simulation
//! - Synchronous reset
//! - Sequential state machines
//!
//! A 4-bit counter increments on each clock cycle. When it reaches 15 (0xF),
//! it wraps around to 0. The reset signal asynchronously resets the counter to 0.

use chipforge_simulation::{
    event::{Event, SimTime},
    kernel::SimulationKernel,
    process::{Edge, ProcessType, Sensitivity},
    value::{BitValue, LogicValue, Value},
    vcd::{ScopeType, VarType, VcdWriter},
};
use std::io;

fn main() -> io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("=== 4-bit Counter Simulation ===\n");

    // Create simulation kernel
    let mut kernel = SimulationKernel::new();
    let mut current_time: SimTime = 0;

    // Create signals
    let clk = kernel.create_net("clk".to_string(), 1, Value::Bit(BitValue::Zero));
    let rst = kernel.create_net("rst".to_string(), 1, Value::Bit(BitValue::Zero));
    let enable = kernel.create_net("enable".to_string(), 1, Value::Bit(BitValue::One));
    let count = kernel.create_net(
        "count".to_string(),
        4,
        Value::Vector(LogicValue::from_u64(0, 4)),
    );

    // Create counter process (sensitive to rising edge of clock)
    let _counter_process = kernel.create_process(
        ProcessType::Sequential,
        Sensitivity::Edge(vec![(clk, Edge::Posedge), (rst, Edge::Posedge)]),
    );

    println!("Created 4-bit counter with signals:");
    println!("  clk    - Clock input");
    println!("  rst    - Reset (active high)");
    println!("  enable - Count enable");
    println!("  count  - 4-bit counter output [3:0]\n");

    // Setup VCD writer
    let mut vcd = VcdWriter::new("counter.vcd")?;

    // Write VCD header
    vcd.set_timescale("1ns".to_string());
    vcd.write_header("4-bit Counter Example", "ChipForge 0.1.0", "")?;
    vcd.scope_begin(ScopeType::Module, "counter")?;
    let clk_id = vcd.var(VarType::Wire, 1, "clk")?;
    let rst_id = vcd.var(VarType::Wire, 1, "rst")?;
    let enable_id = vcd.var(VarType::Wire, 1, "enable")?;
    let count_id = vcd.var(VarType::Reg, 4, "count")?;
    vcd.scope_end()?;
    vcd.enddefinitions()?;

    // Initial values
    vcd.timestamp(0)?;
    vcd.change_scalar(&clk_id, BitValue::Zero)?;
    vcd.change_scalar(&rst_id, BitValue::Zero)?;
    vcd.change_scalar(&enable_id, BitValue::One)?;
    vcd.change_vector(&count_id, &LogicValue::from_u64(0, 4))?;

    println!("Simulation timeline:");
    println!("Time | CLK | RST | EN | COUNT | Event");
    println!("-----|-----|-----|----| ------|------------------");
    println!("   0 |  0  |  0  | 1  | 0000  | Initial state");

    // Counter state
    let mut counter_value: u8 = 0;

    // Simulate test sequence
    for cycle in 0..20 {
        let time = cycle * 10;

        // Apply reset for first cycle
        let rst_val = if cycle == 0 {
            BitValue::One
        } else {
            BitValue::Zero
        };

        // Disable counting for cycles 8-10
        let enable_val = if cycle >= 8 && cycle <= 10 {
            BitValue::Zero
        } else {
            BitValue::One
        };

        // Clock low phase
        kernel.schedule_event(time, Event::new(time, clk, Value::Bit(BitValue::Zero)));
        kernel.schedule_event(time, Event::new(time, rst, Value::Bit(rst_val)));
        kernel.schedule_event(time, Event::new(time, enable, Value::Bit(enable_val)));

        current_time = time;
        kernel.process_time_step(&mut current_time).ok();

        // Update VCD for low phase
        vcd.timestamp(time)?;
        vcd.change_scalar(&clk_id, BitValue::Zero)?;
        vcd.change_scalar(&rst_id, rst_val)?;
        vcd.change_scalar(&enable_id, enable_val)?;

        // Clock high phase (rising edge)
        let time_high = time + 5;
        kernel.schedule_event(
            time_high,
            Event::new(time_high, clk, Value::Bit(BitValue::One)),
        );

        // Counter logic: on rising edge
        if rst_val == BitValue::One {
            counter_value = 0;
        } else if enable_val == BitValue::One {
            counter_value = (counter_value + 1) % 16;
        }

        // Convert counter value to LogicValue
        let count_logic = LogicValue::from_u64(counter_value as u64, 4);

        kernel.schedule_event(
            time_high + 1,
            Event::new(time_high + 1, count, Value::Vector(count_logic.clone())),
        );

        current_time = time_high;
        kernel.process_time_step(&mut current_time).ok();

        // Update VCD for high phase
        vcd.timestamp(time_high)?;
        vcd.change_scalar(&clk_id, BitValue::One)?;

        // Update counter in VCD
        vcd.timestamp(time_high + 1)?;
        vcd.change_vector(&count_id, &count_logic)?;

        // Print state
        let rst_ch = if rst_val == BitValue::One { "1" } else { "0" };
        let en_ch = if enable_val == BitValue::One { "1" } else { "0" };

        let event_desc = if rst_val == BitValue::One {
            "Reset active"
        } else if enable_val == BitValue::Zero {
            "Count disabled"
        } else {
            "Count increment"
        };

        println!(
            " {:3} | ↑1  |  {}  | {}  | {:04b}  | {}",
            time_high, rst_ch, en_ch, counter_value, event_desc
        );
    }

    vcd.flush()?;

    println!("\n=== Simulation Complete ===");
    kernel.print_stats();
    println!("\nWaveform saved to: counter.vcd");
    println!("View with: gtkwave counter.vcd");
    println!("\nKey observations:");
    println!("  - Counter resets to 0 when RST=1");
    println!("  - Counter holds value when ENABLE=0 (cycles 8-10)");
    println!("  - Counter wraps from 15 to 0");

    Ok(())
}
