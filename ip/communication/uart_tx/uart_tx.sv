// UART Transmitter IP Core
// Configurable baud rate, 8N1 format
module uart_tx #(
    parameter CLK_FREQ = 100_000_000,  // Clock frequency in Hz
    parameter BAUD_RATE = 115200        // Baud rate
)(
    input  wire       clk,
    input  wire       rst,

    // Data interface
    input  wire [7:0] tx_data,
    input  wire       tx_valid,
    output reg        tx_ready,

    // UART output
    output reg        uart_tx
);

    localparam CLKS_PER_BIT = CLK_FREQ / BAUD_RATE;
    localparam COUNTER_WIDTH = $clog2(CLKS_PER_BIT);

    typedef enum logic [2:0] {
        IDLE,
        START,
        DATA,
        STOP
    } state_t;

    state_t state;
    reg [COUNTER_WIDTH-1:0] clk_count;
    reg [2:0] bit_index;
    reg [7:0] tx_shift_reg;

    always @(posedge clk) begin
        if (rst) begin
            state <= IDLE;
            uart_tx <= 1'b1;
            tx_ready <= 1'b1;
            clk_count <= 0;
            bit_index <= 0;
            tx_shift_reg <= 8'h00;
        end else begin
            case (state)
                IDLE: begin
                    uart_tx <= 1'b1;
                    tx_ready <= 1'b1;
                    clk_count <= 0;
                    bit_index <= 0;

                    if (tx_valid && tx_ready) begin
                        tx_shift_reg <= tx_data;
                        tx_ready <= 1'b0;
                        state <= START;
                    end
                end

                START: begin
                    uart_tx <= 1'b0;  // Start bit

                    if (clk_count < CLKS_PER_BIT - 1) begin
                        clk_count <= clk_count + 1;
                    end else begin
                        clk_count <= 0;
                        state <= DATA;
                    end
                end

                DATA: begin
                    uart_tx <= tx_shift_reg[bit_index];

                    if (clk_count < CLKS_PER_BIT - 1) begin
                        clk_count <= clk_count + 1;
                    end else begin
                        clk_count <= 0;

                        if (bit_index < 7) begin
                            bit_index <= bit_index + 1;
                        end else begin
                            bit_index <= 0;
                            state <= STOP;
                        end
                    end
                end

                STOP: begin
                    uart_tx <= 1'b1;  // Stop bit

                    if (clk_count < CLKS_PER_BIT - 1) begin
                        clk_count <= clk_count + 1;
                    end else begin
                        clk_count <= 0;
                        state <= IDLE;
                    end
                end

                default: state <= IDLE;
            endcase
        end
    end

endmodule
