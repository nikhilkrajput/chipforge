// Button Debouncer IP Core
// Eliminates mechanical switch bounce
module debouncer #(
    parameter CLK_FREQ = 100_000_000,   // Clock frequency in Hz
    parameter DEBOUNCE_TIME_MS = 20     // Debounce time in milliseconds
)(
    input  wire  clk,
    input  wire  rst,
    input  wire  button_in,
    output reg   button_out
);

    localparam COUNTER_MAX = (CLK_FREQ / 1000) * DEBOUNCE_TIME_MS;
    localparam COUNTER_WIDTH = $clog2(COUNTER_MAX);

    // Synchronize input
    reg button_sync1, button_sync2;

    // Debounce logic
    reg [COUNTER_WIDTH-1:0] counter;
    reg button_state;

    always @(posedge clk) begin
        if (rst) begin
            button_sync1 <= 1'b0;
            button_sync2 <= 1'b0;
            counter <= 0;
            button_state <= 1'b0;
            button_out <= 1'b0;
        end else begin
            // Two-stage synchronizer
            button_sync1 <= button_in;
            button_sync2 <= button_sync1;

            // Debounce logic
            if (button_sync2 != button_state) begin
                // Input differs from current state
                if (counter < COUNTER_MAX - 1) begin
                    counter <= counter + 1;
                end else begin
                    // Stable for debounce time
                    button_state <= button_sync2;
                    button_out <= button_sync2;
                    counter <= 0;
                end
            end else begin
                // Input matches current state
                counter <= 0;
            end
        end
    end

endmodule
