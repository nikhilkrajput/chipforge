// PWM (Pulse Width Modulation) Generator
module pwm_generator #(
    parameter WIDTH = 8  // Counter width determines PWM resolution
)(
    input wire clk,
    input wire rst,
    input wire [WIDTH-1:0] duty_cycle,  // Duty cycle value (0 to 2^WIDTH-1)
    output reg pwm_out
);

    reg [WIDTH-1:0] counter;

    always @(posedge clk) begin
        if (rst) begin
            counter <= {WIDTH{1'b0}};
            pwm_out <= 1'b0;
        end else begin
            counter <= counter + 1'b1;

            // PWM output is high when counter is less than duty cycle
            if (counter < duty_cycle)
                pwm_out <= 1'b1;
            else
                pwm_out <= 1'b0;
        end
    end

endmodule
