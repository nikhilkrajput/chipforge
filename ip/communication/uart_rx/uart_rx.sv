// UART Receiver IP Core
// Configurable baud rate, 8N1 format with input synchronization
module uart_rx #(
    parameter CLK_FREQ = 100_000_000,  // Clock frequency in Hz
    parameter BAUD_RATE = 115200        // Baud rate
)(
    input  wire       clk,
    input  wire       rst,

    // UART input
    input  wire       uart_rx,

    // Data interface
    output reg  [7:0] rx_data,
    output reg        rx_valid,
    input  wire       rx_ready
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

    // Input synchronization
    reg uart_rx_sync1, uart_rx_sync2;

    reg [COUNTER_WIDTH-1:0] clk_count;
    reg [2:0] bit_index;
    reg [7:0] rx_shift_reg;

    // Synchronize input
    always @(posedge clk) begin
        if (rst) begin
            uart_rx_sync1 <= 1'b1;
            uart_rx_sync2 <= 1'b1;
        end else begin
            uart_rx_sync1 <= uart_rx;
            uart_rx_sync2 <= uart_rx_sync1;
        end
    end

    always @(posedge clk) begin
        if (rst) begin
            state <= IDLE;
            rx_valid <= 1'b0;
            clk_count <= 0;
            bit_index <= 0;
            rx_shift_reg <= 8'h00;
            rx_data <= 8'h00;
        end else begin
            rx_valid <= 1'b0;  // Default

            case (state)
                IDLE: begin
                    clk_count <= 0;
                    bit_index <= 0;

                    // Detect start bit (falling edge)
                    if (uart_rx_sync2 == 1'b0) begin
                        state <= START;
                    end
                end

                START: begin
                    // Sample at middle of bit period
                    if (clk_count < (CLKS_PER_BIT / 2)) begin
                        clk_count <= clk_count + 1;
                    end else begin
                        if (uart_rx_sync2 == 1'b0) begin
                            // Valid start bit
                            clk_count <= 0;
                            state <= DATA;
                        end else begin
                            // False start bit
                            state <= IDLE;
                        end
                    end
                end

                DATA: begin
                    if (clk_count < CLKS_PER_BIT - 1) begin
                        clk_count <= clk_count + 1;
                    end else begin
                        clk_count <= 0;
                        rx_shift_reg[bit_index] <= uart_rx_sync2;

                        if (bit_index < 7) begin
                            bit_index <= bit_index + 1;
                        end else begin
                            bit_index <= 0;
                            state <= STOP;
                        end
                    end
                end

                STOP: begin
                    if (clk_count < CLKS_PER_BIT - 1) begin
                        clk_count <= clk_count + 1;
                    end else begin
                        clk_count <= 0;

                        if (uart_rx_sync2 == 1'b1) begin
                            // Valid stop bit
                            rx_data <= rx_shift_reg;
                            rx_valid <= 1'b1;
                        end
                        // Else: framing error, discard data

                        state <= IDLE;
                    end
                end

                default: state <= IDLE;
            endcase
        end
    end

endmodule
