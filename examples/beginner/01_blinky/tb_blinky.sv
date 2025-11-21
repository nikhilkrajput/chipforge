// Testbench for LED Blinker

`timescale 1ns / 1ps

module tb_blinky;

    // Testbench signals
    reg clk;
    reg reset;
    wire led;

    // Instantiate the blinky module
    blinky dut (
        .clk(clk),
        .reset(reset),
        .led(led)
    );

    // Clock generation (50MHz = 20ns period)
    initial begin
        clk = 0;
        forever #10 clk = ~clk;
    end

    // Test sequence
    initial begin
        $display("Starting blinky testbench");

        // Initialize
        reset = 1;
        #100;
        reset = 0;

        // Run for 1 second of simulated time
        #1_000_000_000;

        $display("Test completed");
        $finish;
    end

    // Monitor LED changes
    always @(led) begin
        $display("Time %0t: LED = %b", $time, led);
    end

    // Generate VCD waveform
    initial begin
        $dumpfile("blinky.vcd");
        $dumpvars(0, tb_blinky);
    end

endmodule
