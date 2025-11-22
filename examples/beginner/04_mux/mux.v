// Parameterized multiplexer
module mux #(
    parameter WIDTH = 8,
    parameter INPUTS = 4
)(
    input wire [$clog2(INPUTS)-1:0] sel,
    input wire [WIDTH-1:0] in [0:INPUTS-1],
    output reg [WIDTH-1:0] out
);

    integer i;

    always @(*) begin
        out = {WIDTH{1'b0}};
        for (i = 0; i < INPUTS; i = i + 1) begin
            if (sel == i)
                out = in[i];
        end
    end

endmodule
