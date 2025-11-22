// Pipelined Multiplier IP Core
// Configurable pipeline stages for timing closure
module multiplier #(
    parameter WIDTH = 32,
    parameter PIPELINE_STAGES = 3
)(
    input  wire                 clk,
    input  wire                 rst,
    input  wire [WIDTH-1:0]     a,
    input  wire [WIDTH-1:0]     b,
    input  wire                 valid_in,
    output reg  [2*WIDTH-1:0]   product,
    output reg                  valid_out
);

    // Pipeline registers
    reg [2*WIDTH-1:0] pipe [0:PIPELINE_STAGES-1];
    reg [PIPELINE_STAGES-1:0] valid_pipe;

    integer i;

    always @(posedge clk) begin
        if (rst) begin
            for (i = 0; i < PIPELINE_STAGES; i = i + 1) begin
                pipe[i] <= 0;
            end
            valid_pipe <= 0;
            product <= 0;
            valid_out <= 1'b0;
        end else begin
            // First stage: multiply
            pipe[0] <= a * b;
            valid_pipe[0] <= valid_in;

            // Intermediate pipeline stages
            for (i = 1; i < PIPELINE_STAGES; i = i + 1) begin
                pipe[i] <= pipe[i-1];
                valid_pipe[i] <= valid_pipe[i-1];
            end

            // Output
            product <= pipe[PIPELINE_STAGES-1];
            valid_out <= valid_pipe[PIPELINE_STAGES-1];
        end
    end

endmodule
