// Reset Controller IP Core
// Power-on reset and synchronous reset generation
module reset_controller #(
    parameter RESET_CYCLES = 16  // Number of cycles to hold reset
)(
    input  wire  clk,
    input  wire  async_rst_n,    // Async reset input (active low)
    output reg   sync_rst,       // Sync reset output (active high)
    output reg   rst_done        // Reset sequence complete
);

    // Synchronize async reset
    reg async_rst_n_sync1, async_rst_n_sync2;

    // Reset counter
    reg [$clog2(RESET_CYCLES+1)-1:0] rst_counter;

    always @(posedge clk or negedge async_rst_n) begin
        if (!async_rst_n) begin
            async_rst_n_sync1 <= 1'b0;
            async_rst_n_sync2 <= 1'b0;
        end else begin
            async_rst_n_sync1 <= 1'b1;
            async_rst_n_sync2 <= async_rst_n_sync1;
        end
    end

    always @(posedge clk or negedge async_rst_n) begin
        if (!async_rst_n) begin
            rst_counter <= 0;
            sync_rst <= 1'b1;
            rst_done <= 1'b0;
        end else if (!async_rst_n_sync2) begin
            rst_counter <= 0;
            sync_rst <= 1'b1;
            rst_done <= 1'b0;
        end else begin
            if (rst_counter < RESET_CYCLES) begin
                rst_counter <= rst_counter + 1;
                sync_rst <= 1'b1;
                rst_done <= 1'b0;
            end else begin
                sync_rst <= 1'b0;
                rst_done <= 1'b1;
            end
        end
    end

endmodule
