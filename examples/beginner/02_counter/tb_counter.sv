// Testbench for counter
module tb_counter;

    reg clk;
    reg rst;
    reg enable;
    wire [7:0] count;

    // Instantiate counter
    counter #(.WIDTH(8)) dut (
        .clk(clk),
        .rst(rst),
        .enable(enable),
        .count(count)
    );

    // Clock generation
    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end

    // Test sequence
    initial begin
        $dumpfile("counter.vcd");
        $dumpvars(0, tb_counter);

        // Reset
        rst = 1;
        enable = 0;
        #20;
        rst = 0;

        // Enable counting
        enable = 1;
        #200;

        // Disable
        enable = 0;
        #20;

        // Enable again
        enable = 1;
        #100;

        // Reset while counting
        rst = 1;
        #10;
        rst = 0;
        #100;

        $display("Test completed!");
        $finish;
    end

    // Monitor
    initial begin
        $monitor("Time=%0t rst=%b enable=%b count=%d",
                 $time, rst, enable, count);
    end

endmodule
