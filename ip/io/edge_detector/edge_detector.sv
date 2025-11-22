// Edge Detector IP Core
// Detects rising, falling, or both edges
module edge_detector #(
    parameter DETECT_RISING = 1,
    parameter DETECT_FALLING = 1
)(
    input  wire  clk,
    input  wire  rst,
    input  wire  signal_in,
    output reg   edge_detected,
    output reg   rising_edge,
    output reg   falling_edge
);

    reg signal_delayed;

    always @(posedge clk) begin
        if (rst) begin
            signal_delayed <= 1'b0;
            edge_detected <= 1'b0;
            rising_edge <= 1'b0;
            falling_edge <= 1'b0;
        end else begin
            signal_delayed <= signal_in;

            // Detect rising edge
            if (DETECT_RISING && signal_in && !signal_delayed) begin
                rising_edge <= 1'b1;
            end else begin
                rising_edge <= 1'b0;
            end

            // Detect falling edge
            if (DETECT_FALLING && !signal_in && signal_delayed) begin
                falling_edge <= 1'b1;
            end else begin
                falling_edge <= 1'b0;
            end

            // Either edge detected
            edge_detected <= rising_edge || falling_edge;
        end
    end

endmodule
