// Binary to one-hot decoder
module decoder #(
    parameter ADDR_WIDTH = 3
)(
    input wire [ADDR_WIDTH-1:0] addr,
    input wire enable,
    output reg [(1<<ADDR_WIDTH)-1:0] out
);

    localparam OUTPUTS = 1 << ADDR_WIDTH;

    integer i;

    always @(*) begin
        out = {OUTPUTS{1'b0}};
        if (enable) begin
            for (i = 0; i < OUTPUTS; i = i + 1) begin
                if (addr == i)
                    out[i] = 1'b1;
            end
        end
    end

endmodule
