// Watchdog Timer IP Core
// System watchdog with timeout reset capability
module watchdog #(
    parameter WIDTH = 32,
    parameter DEFAULT_TIMEOUT = 1000000
)(
    input  wire              clk,
    input  wire              rst,

    // Configuration
    input  wire [WIDTH-1:0]  timeout_value,
    input  wire              enable,

    // Control
    input  wire              kick,          // Kick watchdog (reset counter)

    // Status/Output
    output reg               timeout,       // Watchdog timeout occurred
    output reg               timeout_rst    // Reset output
);

    reg [WIDTH-1:0] counter;
    reg [WIDTH-1:0] timeout_reg;

    always @(posedge clk) begin
        if (rst) begin
            counter <= 0;
            timeout <= 1'b0;
            timeout_rst <= 1'b0;
            timeout_reg <= DEFAULT_TIMEOUT;
        end else begin
            // Update timeout value
            timeout_reg <= timeout_value;

            if (enable) begin
                if (kick) begin
                    // Kick watchdog - reset counter
                    counter <= 0;
                    timeout <= 1'b0;
                    timeout_rst <= 1'b0;
                end else if (counter >= timeout_reg) begin
                    // Watchdog timeout!
                    timeout <= 1'b1;
                    timeout_rst <= 1'b1;
                    // Counter stops at timeout
                end else begin
                    // Normal counting
                    counter <= counter + 1;
                    timeout_rst <= 1'b0;
                end
            end else begin
                // Disabled - reset
                counter <= 0;
                timeout <= 1'b0;
                timeout_rst <= 1'b0;
            end
        end
    end

endmodule
