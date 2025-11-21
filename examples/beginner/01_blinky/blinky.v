// Simple LED Blinker
// Toggles an LED at approximately 1Hz (assuming 50MHz clock)

module blinky (
    input wire clk,      // 50MHz clock input
    input wire reset,    // Active-high reset
    output reg led       // LED output
);

    // Counter to divide clock
    // 50MHz / 25M = 2Hz (toggle at 1Hz)
    reg [24:0] counter;

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            counter <= 25'h0;
            led <= 1'b0;
        end else begin
            counter <= counter + 1'b1;
            // Toggle LED when counter reaches max
            if (counter == 25'h1FFFFFF) begin
                led <= ~led;
                counter <= 25'h0;
            end
        end
    end

endmodule
