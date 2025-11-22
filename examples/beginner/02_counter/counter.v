// 8-bit up counter with synchronous reset
module counter #(
    parameter WIDTH = 8
)(
    input wire clk,
    input wire rst,
    input wire enable,
    output reg [WIDTH-1:0] count
);

    always @(posedge clk) begin
        if (rst)
            count <= {WIDTH{1'b0}};
        else if (enable)
            count <= count + 1'b1;
    end

endmodule
