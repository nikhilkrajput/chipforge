// Clock Divider IP Core
// Generates integer divided clock with 50% duty cycle
module clock_divider #(
    parameter DIVIDE_BY = 2  // Division factor (must be >= 2)
)(
    input  wire  clk_in,
    input  wire  rst,
    output reg   clk_out
);

    localparam COUNTER_WIDTH = $clog2(DIVIDE_BY);

    reg [COUNTER_WIDTH-1:0] counter;

    generate
        if (DIVIDE_BY == 2) begin : div_by_2
            // Optimized divide-by-2
            always @(posedge clk_in) begin
                if (rst) begin
                    clk_out <= 1'b0;
                end else begin
                    clk_out <= ~clk_out;
                end
            end
        end else begin : div_by_n
            // General divider
            always @(posedge clk_in) begin
                if (rst) begin
                    counter <= 0;
                    clk_out <= 1'b0;
                end else begin
                    if (counter == (DIVIDE_BY / 2) - 1) begin
                        clk_out <= ~clk_out;
                        counter <= 0;
                    end else begin
                        counter <= counter + 1;
                    end
                end
            end
        end
    endgenerate

endmodule
