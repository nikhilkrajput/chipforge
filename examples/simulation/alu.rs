//! 8-bit ALU Simulation Example
//!
//! This example demonstrates:
//! - Multi-bit combinational logic
//! - Multiple operations (ADD, SUB, AND, OR, XOR, NOT)
//! - Operation selection via opcode
//! - Flag generation (zero, carry, negative)
//!
//! The ALU performs arithmetic and logical operations on two 8-bit inputs
//! and generates an 8-bit result with status flags.

use chipforge_simulation::{
    event::{Event, SimTime},
    kernel::SimulationKernel,
    process::{ProcessType, Sensitivity},
    value::{BitValue, LogicValue, Value},
    vcd::{ScopeType, VarType, VcdWriter},
};
use std::io;

// ALU operation codes
#[derive(Debug, Clone, Copy, PartialEq)]
enum AluOp {
    Add = 0,
    Sub = 1,
    And = 2,
    Or = 3,
    Xor = 4,
    Not = 5,
    Shl = 6, // Shift left
    Shr = 7, // Shift right
}

impl AluOp {
    fn from_u8(val: u8) -> Self {
        match val {
            0 => AluOp::Add,
            1 => AluOp::Sub,
            2 => AluOp::And,
            3 => AluOp::Or,
            4 => AluOp::Xor,
            5 => AluOp::Not,
            6 => AluOp::Shl,
            7 => AluOp::Shr,
            _ => AluOp::Add,
        }
    }

    fn name(&self) -> &str {
        match self {
            AluOp::Add => "ADD",
            AluOp::Sub => "SUB",
            AluOp::And => "AND",
            AluOp::Or => "OR",
            AluOp::Xor => "XOR",
            AluOp::Not => "NOT",
            AluOp::Shl => "SHL",
            AluOp::Shr => "SHR",
        }
    }
}

fn main() -> io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("=== 8-bit ALU Simulation ===\n");

    // Create simulation kernel
    let mut kernel = SimulationKernel::new();
    let mut current_time: SimTime = 0;

    // Create signals
    let a = kernel.create_net(
        "a".to_string(),
        8,
        Value::Vector(LogicValue::from_u64(0, 8)),
    );
    let b = kernel.create_net(
        "b".to_string(),
        8,
        Value::Vector(LogicValue::from_u64(0, 8)),
    );
    let opcode = kernel.create_net(
        "opcode".to_string(),
        3,
        Value::Vector(LogicValue::from_u64(0, 3)),
    );
    let result = kernel.create_net(
        "result".to_string(),
        8,
        Value::Vector(LogicValue::from_u64(0, 8)),
    );
    let zero_flag = kernel.create_net("zero".to_string(), 1, Value::Bit(BitValue::Zero));
    let carry_flag = kernel.create_net("carry".to_string(), 1, Value::Bit(BitValue::Zero));
    let neg_flag = kernel.create_net("negative".to_string(), 1, Value::Bit(BitValue::Zero));

    // Create ALU process (combinational, sensitive to all inputs)
    let _alu_process = kernel.create_process(
        ProcessType::Combinational,
        Sensitivity::Level(vec![a, b, opcode]),
    );

    println!("Created 8-bit ALU with signals:");
    println!("  a[7:0]     - Input A");
    println!("  b[7:0]     - Input B");
    println!("  opcode[2:0] - Operation code");
    println!("  result[7:0] - Output");
    println!("  zero       - Zero flag (result == 0)");
    println!("  carry      - Carry flag (overflow)");
    println!("  negative   - Negative flag (MSB of result)\n");

    println!("Operations:");
    println!("  0 = ADD  |  1 = SUB  |  2 = AND  |  3 = OR");
    println!("  4 = XOR  |  5 = NOT  |  6 = SHL  |  7 = SHR\n");

    // Setup VCD writer
    let mut vcd = VcdWriter::new("alu.vcd")?;

    // Write VCD header
    vcd.set_timescale("1ns".to_string());
    vcd.write_header("8-bit ALU Example", "ChipForge 0.1.0", "")?;
    vcd.scope_begin(ScopeType::Module, "alu")?;
    let a_id = vcd.var(VarType::Wire, 8, "a")?;
    let b_id = vcd.var(VarType::Wire, 8, "b")?;
    let opcode_id = vcd.var(VarType::Wire, 3, "opcode")?;
    let result_id = vcd.var(VarType::Wire, 8, "result")?;
    let zero_id = vcd.var(VarType::Wire, 1, "zero")?;
    let carry_id = vcd.var(VarType::Wire, 1, "carry")?;
    let negative_id = vcd.var(VarType::Wire, 1, "negative")?;
    vcd.scope_end()?;
    vcd.enddefinitions()?;

    // Initial values
    vcd.timestamp(0)?;
    vcd.change_vector(&a_id, &LogicValue::from_u64(0, 8))?;
    vcd.change_vector(&b_id, &LogicValue::from_u64(0, 8))?;
    vcd.change_vector(&opcode_id, &LogicValue::from_u64(0, 3))?;
    vcd.change_vector(&result_id, &LogicValue::from_u64(0, 8))?;
    vcd.change_scalar(&zero_id, BitValue::Zero)?;
    vcd.change_scalar(&carry_id, BitValue::Zero)?;
    vcd.change_scalar(&negative_id, BitValue::Zero)?;

    println!("Time | Op  |  A  |  B  | Result | Z | C | N | Description");
    println!("-----|-----|-----|-----|--------|---|---|---|------------------");

    // Test cases: (a_val, b_val, op_code)
    let test_cases = vec![
        (15, 10, 0, "15 + 10 = 25"),
        (100, 50, 0, "100 + 50 = 150"),
        (200, 100, 0, "200 + 100 = 44 (overflow)"),
        (50, 30, 1, "50 - 30 = 20"),
        (30, 50, 1, "30 - 50 = 236 (underflow)"),
        (0b11110000, 0b10101010, 2, "AND operation"),
        (0b11110000, 0b00001111, 3, "OR operation"),
        (0b11110000, 0b10101010, 4, "XOR operation"),
        (0b10101010, 0, 5, "NOT operation"),
        (0b00001111, 0, 6, "Shift left"),
        (0b11110000, 0, 7, "Shift right"),
        (0, 0, 0, "Zero result test"),
        (128, 0, 0, "Negative flag test"),
    ];

    for (i, (a_val, b_val, op_val, description)) in test_cases.iter().enumerate() {
        let time = (i as u64 + 1) * 10;

        // Schedule input changes
        kernel.schedule_event(
            time,
            Event::new(time, a, Value::Vector(LogicValue::from_u64(*a_val, 8))),
        );
        kernel.schedule_event(
            time,
            Event::new(time, b, Value::Vector(LogicValue::from_u64(*b_val, 8))),
        );
        kernel.schedule_event(
            time,
            Event::new(time, opcode, Value::Vector(LogicValue::from_u64(*op_val, 3))),
        );

        // Compute ALU result
        let op = AluOp::from_u8(*op_val as u8);
        let (result_val, carry, zero, negative) = match op {
            AluOp::Add => {
                let sum = (*a_val as u16) + (*b_val as u16);
                let r = (sum & 0xFF) as u8;
                (r, sum > 255, r == 0, (r & 0x80) != 0)
            }
            AluOp::Sub => {
                let diff = (*a_val as i16) - (*b_val as i16);
                let r = (diff & 0xFF) as u8;
                (r, diff < 0, r == 0, (r & 0x80) != 0)
            }
            AluOp::And => {
                let r = (*a_val & *b_val) as u8;
                (r, false, r == 0, (r & 0x80) != 0)
            }
            AluOp::Or => {
                let r = (*a_val | *b_val) as u8;
                (r, false, r == 0, (r & 0x80) != 0)
            }
            AluOp::Xor => {
                let r = (*a_val ^ *b_val) as u8;
                (r, false, r == 0, (r & 0x80) != 0)
            }
            AluOp::Not => {
                let r = (!*a_val) as u8;
                (r, false, r == 0, (r & 0x80) != 0)
            }
            AluOp::Shl => {
                let r = (*a_val << 1) as u8;
                let c = (*a_val & 0x80) != 0;
                (r, c, r == 0, (r & 0x80) != 0)
            }
            AluOp::Shr => {
                let r = (*a_val >> 1) as u8;
                let c = (*a_val & 0x01) != 0;
                (r, c, r == 0, (r & 0x80) != 0)
            }
        };

        // Schedule result updates (with 1 unit delay)
        kernel.schedule_event(
            time + 1,
            Event::new(
                time + 1,
                result,
                Value::Vector(LogicValue::from_u64(result_val as u64, 8)),
            ),
        );
        kernel.schedule_event(
            time + 1,
            Event::new(
                time + 1,
                zero_flag,
                Value::Bit(if zero {
                    BitValue::One
                } else {
                    BitValue::Zero
                }),
            ),
        );
        kernel.schedule_event(
            time + 1,
            Event::new(
                time + 1,
                carry_flag,
                Value::Bit(if carry {
                    BitValue::One
                } else {
                    BitValue::Zero
                }),
            ),
        );
        kernel.schedule_event(
            time + 1,
            Event::new(
                time + 1,
                neg_flag,
                Value::Bit(if negative {
                    BitValue::One
                } else {
                    BitValue::Zero
                }),
            ),
        );

        // Process time step
        current_time = time;
        kernel.process_time_step(&mut current_time).ok();
        current_time = time + 1;
        kernel.process_time_step(&mut current_time).ok();

        // Update VCD
        vcd.timestamp(time)?;
        vcd.change_vector(&a_id, &LogicValue::from_u64(*a_val, 8))?;
        vcd.change_vector(&b_id, &LogicValue::from_u64(*b_val, 8))?;
        vcd.change_vector(&opcode_id, &LogicValue::from_u64(*op_val, 3))?;

        vcd.timestamp(time + 1)?;
        vcd.change_vector(&result_id, &LogicValue::from_u64(result_val as u64, 8))?;
        vcd.change_scalar(
            &zero_id,
            if zero {
                BitValue::One
            } else {
                BitValue::Zero
            },
        )?;
        vcd.change_scalar(
            &carry_id,
            if carry {
                BitValue::One
            } else {
                BitValue::Zero
            },
        )?;
        vcd.change_scalar(
            &negative_id,
            if negative {
                BitValue::One
            } else {
                BitValue::Zero
            },
        )?;

        // Print state
        println!(
            " {:3} | {} | {:3} | {:3} |  {:3}   | {} | {} | {} | {}",
            time,
            op.name(),
            a_val,
            b_val,
            result_val,
            if zero { "1" } else { "0" },
            if carry { "1" } else { "0" },
            if negative { "1" } else { "0" },
            description
        );
    }

    vcd.flush()?;

    println!("\n=== Simulation Complete ===");
    kernel.print_stats();
    println!("\nWaveform saved to: alu.vcd");
    println!("View with: gtkwave alu.vcd");
    println!("\nKey observations:");
    println!("  - Combinational logic (no clock)");
    println!("  - Multiple operations via opcode");
    println!("  - Flags indicate result properties");
    println!("  - Overflow/underflow in ADD/SUB");

    Ok(())
}
