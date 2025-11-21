// UART Transmitter
// Configurable baud rate, 8 data bits, 1 stop bit, no parity

module uart_tx #(
    parameter CLK_FREQ = 50_000_000,  // Clock frequency in Hz
    parameter BAUD_RATE = 115200       // Baud rate
) (
    input wire clk,
    input wire reset,

    // Data interface
    input wire [7:0] data_in,
    input wire data_valid,
    output reg data_ready,

    // UART output
    output reg tx
);

    localparam CLKS_PER_BIT = CLK_FREQ / BAUD_RATE;

    // State machine
    localparam IDLE = 2'b00;
    localparam START = 2'b01;
    localparam DATA = 2'b10;
    localparam STOP = 2'b11;

    reg [1:0] state;
    reg [15:0] clk_count;
    reg [2:0] bit_index;
    reg [7:0] tx_data;

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            state <= IDLE;
            tx <= 1'b1;  // Idle high
            data_ready <= 1'b1;
            clk_count <= 16'h0;
            bit_index <= 3'h0;
            tx_data <= 8'h0;
        end else begin
            case (state)
                IDLE: begin
                    tx <= 1'b1;
                    data_ready <= 1'b1;
                    clk_count <= 16'h0;
                    bit_index <= 3'h0;

                    if (data_valid) begin
                        tx_data <= data_in;
                        data_ready <= 1'b0;
                        state <= START;
                    end
                end

                START: begin
                    tx <= 1'b0;  // Start bit

                    if (clk_count < CLKS_PER_BIT - 1) begin
                        clk_count <= clk_count + 1'b1;
                    end else begin
                        clk_count <= 16'h0;
                        state <= DATA;
                    end
                end

                DATA: begin
                    tx <= tx_data[bit_index];

                    if (clk_count < CLKS_PER_BIT - 1) begin
                        clk_count <= clk_count + 1'b1;
                    end else begin
                        clk_count <= 16'h0;

                        if (bit_index < 7) begin
                            bit_index <= bit_index + 1'b1;
                        end else begin
                            bit_index <= 3'h0;
                            state <= STOP;
                        end
                    end
                end

                STOP: begin
                    tx <= 1'b1;  // Stop bit

                    if (clk_count < CLKS_PER_BIT - 1) begin
                        clk_count <= clk_count + 1'b1;
                    end else begin
                        clk_count <= 16'h0;
                        state <= IDLE;
                    end
                end
            endcase
        end
    end

endmodule
