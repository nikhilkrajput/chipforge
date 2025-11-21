//! D Flip-Flop Simulation Example
//!
//! This example demonstrates:
//! - Sequential logic simulation
//! - Clock edge detection
//! - Process-based modeling
//! - VCD waveform generation
//!
//! The D flip-flop captures the D input on the rising edge of the clock
//! and outputs it on Q. This is the fundamental storage element in digital design.

use chipforge_simulation::{
    event::{Event, SimTime},
    kernel::SimulationKernel,
    process::{Edge, ProcessType, Sensitivity},
    value::{BitValue, Value},
    vcd::{ScopeType, VarType, VcdWriter},
};
use std::io;

fn main() -> io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("=== D Flip-Flop Simulation ===\n");

    // Create simulation kernel
    let mut kernel = SimulationKernel::new();
    let mut current_time: SimTime = 0;

    // Create signals
    let clk = kernel.create_net("clk".to_string(), 1, Value::Bit(BitValue::Zero));
    let d = kernel.create_net("d".to_string(), 1, Value::Bit(BitValue::Zero));
    let q = kernel.create_net("q".to_string(), 1, Value::Bit(BitValue::Zero));
    let q_n = kernel.create_net("q_n".to_string(), 1, Value::Bit(BitValue::One));

    // Create D flip-flop process (sensitive to rising edge of clock)
    let _dff_process = kernel.create_process(
        ProcessType::Sequential,
        Sensitivity::Edge(vec![(clk, Edge::Posedge)]),
    );

    println!("Created D flip-flop with signals:");
    println!("  clk - Clock input");
    println!("  d   - Data input");
    println!("  q   - Output");
    println!("  q_n - Inverted output\n");

    // Setup VCD writer
    let mut vcd = VcdWriter::new("dff.vcd")?;

    // Write VCD header
    vcd.set_timescale("1ps".to_string());
    vcd.write_header("D Flip-Flop Example", "ChipForge 0.1.0", "")?;
    vcd.scope_begin(ScopeType::Module, "dff")?;
    let clk_id = vcd.var(VarType::Wire, 1, "clk")?;
    let d_id = vcd.var(VarType::Wire, 1, "d")?;
    let q_id = vcd.var(VarType::Reg, 1, "q")?;
    let q_n_id = vcd.var(VarType::Reg, 1, "q_n")?;
    vcd.scope_end()?;
    vcd.enddefinitions()?;

    // Initial values
    vcd.timestamp(0)?;
    vcd.change_scalar(&clk_id, BitValue::Zero)?;
    vcd.change_scalar(&d_id, BitValue::Zero)?;
    vcd.change_scalar(&q_id, BitValue::Zero)?;
    vcd.change_scalar(&q_n_id, BitValue::One)?;

    println!("Simulation timeline:");
    println!("Time | CLK | D | Q | Q_N | Event");
    println!("-----|-----|---|---|-----|------------------------");
    println!("   0 |  0  | 0 | 0 |  1  | Initial state");

    // Simulate a test sequence
    let mut test_sequence = vec![
        // (time, clk, d, description)
        (10, BitValue::One, BitValue::Zero, "CLK rise, D=0"),
        (20, BitValue::Zero, BitValue::Zero, "CLK fall"),
        (30, BitValue::Zero, BitValue::One, "D changes to 1"),
        (40, BitValue::One, BitValue::One, "CLK rise, D=1 -> Q=1"),
        (50, BitValue::Zero, BitValue::One, "CLK fall"),
        (60, BitValue::Zero, BitValue::Zero, "D changes to 0"),
        (70, BitValue::One, BitValue::Zero, "CLK rise, D=0 -> Q=0"),
        (80, BitValue::Zero, BitValue::Zero, "CLK fall"),
        (90, BitValue::One, BitValue::One, "CLK rise, D=1 -> Q=1"),
        (100, BitValue::Zero, BitValue::One, "CLK fall"),
    ];

    for (time, clk_val, d_val, description) in test_sequence.drain(..) {
        current_time = time;

        // Schedule clock change
        kernel.schedule_event(time, Event::new(time, clk, Value::Bit(clk_val)));

        // Schedule data change
        kernel.schedule_event(time, Event::new(time, d, Value::Bit(d_val)));

        // Simulate the D flip-flop behavior manually
        // (In a full implementation, this would be done by the process evaluation)
        if clk_val == BitValue::One {
            // Rising edge - capture D
            let prev_clk = kernel.get_net_value(clk).unwrap();
            if let Value::Bit(prev) = prev_clk {
                if *prev == BitValue::Zero {
                    // Rising edge detected, capture D to Q
                    kernel.schedule_event(
                        time + 1,
                        Event::new(time + 1, q, Value::Bit(d_val)),
                    );
                    kernel.schedule_event(
                        time + 1,
                        Event::new(time + 1, q_n, Value::Bit(!d_val)),
                    );
                }
            }
        }

        // Process time step
        kernel.process_time_step(&mut current_time).ok();

        // Update VCD
        vcd.timestamp(time)?;
        if let Some(Value::Bit(v)) = kernel.get_net_value(clk) {
            vcd.change_scalar(&clk_id, *v)?;
        }
        if let Some(Value::Bit(v)) = kernel.get_net_value(d) {
            vcd.change_scalar(&d_id, *v)?;
        }
        if let Some(Value::Bit(v)) = kernel.get_net_value(q) {
            vcd.change_scalar(&q_id, *v)?;
        }
        if let Some(Value::Bit(v)) = kernel.get_net_value(q_n) {
            vcd.change_scalar(&q_n_id, *v)?;
        }

        // Print state
        let clk_ch = if clk_val == BitValue::One { "1" } else { "0" };
        let d_ch = if d_val == BitValue::One { "1" } else { "0" };
        let q_val = kernel.get_net_value(q).unwrap();
        let q_ch = if let Value::Bit(BitValue::One) = q_val {
            "1"
        } else {
            "0"
        };
        let q_n_val = kernel.get_net_value(q_n).unwrap();
        let q_n_ch = if let Value::Bit(BitValue::One) = q_n_val {
            "1"
        } else {
            "0"
        };

        println!(
            " {:3} |  {}  | {} | {} |  {}  | {}",
            time, clk_ch, d_ch, q_ch, q_n_ch, description
        );
    }

    vcd.flush()?;

    println!("\n=== Simulation Complete ===");
    kernel.print_stats();
    println!("\nWaveform saved to: dff.vcd");
    println!("View with: gtkwave dff.vcd");

    Ok(())
}
