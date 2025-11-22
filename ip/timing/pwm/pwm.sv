// PWM Generator IP Core
// High-resolution PWM with configurable frequency and duty cycle
module pwm #(
    parameter COUNTER_WIDTH = 16
)(
    input  wire                        clk,
    input  wire                        rst,

    // Configuration
    input  wire [COUNTER_WIDTH-1:0]    period,       // PWM period
    input  wire [COUNTER_WIDTH-1:0]    duty_cycle,   // Duty cycle (0 to period)
    input  wire                        enable,       // PWM enable
    input  wire                        polarity,     // 0=active high, 1=active low

    // Output
    output reg                         pwm_out
);

    reg [COUNTER_WIDTH-1:0] counter;
    reg pwm_internal;

    always @(posedge clk) begin
        if (rst) begin
            counter <= 0;
            pwm_internal <= 1'b0;
        end else if (enable) begin
            // Increment counter
            if (counter >= period) begin
                counter <= 0;
            end else begin
                counter <= counter + 1;
            end

            // Generate PWM
            if (counter < duty_cycle) begin
                pwm_internal <= 1'b1;
            end else begin
                pwm_internal <= 1'b0;
            end
        end else begin
            counter <= 0;
            pwm_internal <= 1'b0;
        end
    end

    // Apply polarity
    always @(posedge clk) begin
        if (rst) begin
            pwm_out <= 1'b0;
        end else begin
            pwm_out <= polarity ? ~pwm_internal : pwm_internal;
        end
    end

endmodule
