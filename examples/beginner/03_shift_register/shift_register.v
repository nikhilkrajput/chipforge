// Parameterized shift register with parallel load
module shift_register #(
    parameter WIDTH = 8
)(
    input wire clk,
    input wire rst,
    input wire load,
    input wire shift_en,
    input wire serial_in,
    input wire [WIDTH-1:0] parallel_in,
    output wire serial_out,
    output wire [WIDTH-1:0] parallel_out
);

    reg [WIDTH-1:0] shift_reg;

    always @(posedge clk) begin
        if (rst)
            shift_reg <= {WIDTH{1'b0}};
        else if (load)
            shift_reg <= parallel_in;
        else if (shift_en)
            shift_reg <= {shift_reg[WIDTH-2:0], serial_in};
    end

    assign serial_out = shift_reg[WIDTH-1];
    assign parallel_out = shift_reg;

endmodule
