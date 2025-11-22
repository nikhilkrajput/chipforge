// Integer Divider IP Core
// Non-restoring division algorithm
module divider #(
    parameter WIDTH = 32
)(
    input  wire                 clk,
    input  wire                 rst,
    input  wire [WIDTH-1:0]     dividend,
    input  wire [WIDTH-1:0]     divisor,
    input  wire                 start,
    output reg  [WIDTH-1:0]     quotient,
    output reg  [WIDTH-1:0]     remainder,
    output reg                  valid,
    output reg                  div_by_zero
);

    typedef enum logic [1:0] {
        IDLE,
        COMPUTE,
        DONE
    } state_t;

    state_t state;

    reg [WIDTH-1:0] divisor_reg;
    reg [2*WIDTH-1:0] acc;  // Accumulator
    reg [$clog2(WIDTH)-1:0] bit_count;

    always @(posedge clk) begin
        if (rst) begin
            state <= IDLE;
            quotient <= 0;
            remainder <= 0;
            valid <= 1'b0;
            div_by_zero <= 1'b0;
            divisor_reg <= 0;
            acc <= 0;
            bit_count <= 0;
        end else begin
            valid <= 1'b0;  // Default

            case (state)
                IDLE: begin
                    div_by_zero <= 1'b0;

                    if (start) begin
                        if (divisor == 0) begin
                            // Division by zero
                            quotient <= {WIDTH{1'b1}};
                            remainder <= dividend;
                            div_by_zero <= 1'b1;
                            valid <= 1'b1;
                            state <= IDLE;
                        end else begin
                            divisor_reg <= divisor;
                            acc <= {{WIDTH{1'b0}}, dividend};
                            bit_count <= WIDTH - 1;
                            state <= COMPUTE;
                        end
                    end
                end

                COMPUTE: begin
                    // Shift accumulator left
                    acc <= {acc[2*WIDTH-2:0], 1'b0};

                    // Subtract divisor from upper half
                    if (acc[2*WIDTH-1:WIDTH] >= divisor_reg) begin
                        acc[2*WIDTH-1:WIDTH] <= acc[2*WIDTH-1:WIDTH] - divisor_reg;
                        acc[0] <= 1'b1;  // Set quotient bit
                    end

                    if (bit_count == 0) begin
                        state <= DONE;
                    end else begin
                        bit_count <= bit_count - 1;
                    end
                end

                DONE: begin
                    quotient <= acc[WIDTH-1:0];
                    remainder <= acc[2*WIDTH-1:WIDTH];
                    valid <= 1'b1;
                    state <= IDLE;
                end

                default: state <= IDLE;
            endcase
        end
    end

endmodule
