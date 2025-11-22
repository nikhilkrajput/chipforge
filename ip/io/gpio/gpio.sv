// GPIO IP Core
// General Purpose Input/Output with direction control
module gpio #(
    parameter WIDTH = 32
)(
    input  wire              clk,
    input  wire              rst,

    // CPU interface
    input  wire [WIDTH-1:0]  data_out,      // Data to output
    output reg  [WIDTH-1:0]  data_in,       // Data read from pins
    input  wire [WIDTH-1:0]  direction,     // 1=output, 0=input
    input  wire [WIDTH-1:0]  output_enable, // Per-pin output enable

    // GPIO pins (bidirectional)
    inout  wire [WIDTH-1:0]  gpio_pins
);

    // Synchronized input
    reg [WIDTH-1:0] gpio_sync1, gpio_sync2;

    // Output data register
    reg [WIDTH-1:0] data_out_reg;

    genvar i;
    generate
        for (i = 0; i < WIDTH; i = i + 1) begin : gpio_pin
            // Tristate control
            assign gpio_pins[i] = (direction[i] && output_enable[i]) ? data_out_reg[i] : 1'bz;
        end
    endgenerate

    always @(posedge clk) begin
        if (rst) begin
            data_out_reg <= {WIDTH{1'b0}};
            gpio_sync1 <= {WIDTH{1'b0}};
            gpio_sync2 <= {WIDTH{1'b0}};
            data_in <= {WIDTH{1'b0}};
        end else begin
            // Register output data
            data_out_reg <= data_out;

            // Two-stage synchronizer for inputs
            gpio_sync1 <= gpio_pins;
            gpio_sync2 <= gpio_sync1;
            data_in <= gpio_sync2;
        end
    end

endmodule
