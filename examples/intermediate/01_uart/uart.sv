// UART transmitter and receiver
module uart #(
    parameter CLK_FREQ = 50000000,  // 50 MHz
    parameter BAUD_RATE = 115200
)(
    input wire clk,
    input wire rst,

    // TX interface
    input wire [7:0] tx_data,
    input wire tx_valid,
    output reg tx_ready,
    output reg tx_out,

    // RX interface
    input wire rx_in,
    output reg [7:0] rx_data,
    output reg rx_valid
);

    // Baud rate generator
    localparam CLKS_PER_BIT = CLK_FREQ / BAUD_RATE;
    localparam COUNTER_WIDTH = $clog2(CLKS_PER_BIT);

    reg [COUNTER_WIDTH-1:0] baud_counter;
    reg baud_tick;

    always @(posedge clk) begin
        if (rst) begin
            baud_counter <= 0;
            baud_tick <= 0;
        end else begin
            if (baud_counter == CLKS_PER_BIT - 1) begin
                baud_counter <= 0;
                baud_tick <= 1;
            end else begin
                baud_counter <= baud_counter + 1;
                baud_tick <= 0;
            end
        end
    end

    // TX state machine
    localparam TX_IDLE  = 3'd0;
    localparam TX_START = 3'd1;
    localparam TX_DATA  = 3'd2;
    localparam TX_STOP  = 3'd3;

    reg [2:0] tx_state;
    reg [2:0] tx_bit_index;
    reg [7:0] tx_shift_reg;

    always @(posedge clk) begin
        if (rst) begin
            tx_state <= TX_IDLE;
            tx_out <= 1'b1;
            tx_ready <= 1'b1;
            tx_bit_index <= 0;
        end else begin
            case (tx_state)
                TX_IDLE: begin
                    tx_out <= 1'b1;
                    tx_ready <= 1'b1;
                    if (tx_valid) begin
                        tx_shift_reg <= tx_data;
                        tx_ready <= 1'b0;
                        tx_state <= TX_START;
                    end
                end

                TX_START: begin
                    if (baud_tick) begin
                        tx_out <= 1'b0;  // Start bit
                        tx_state <= TX_DATA;
                        tx_bit_index <= 0;
                    end
                end

                TX_DATA: begin
                    if (baud_tick) begin
                        tx_out <= tx_shift_reg[tx_bit_index];
                        if (tx_bit_index == 7) begin
                            tx_state <= TX_STOP;
                        end else begin
                            tx_bit_index <= tx_bit_index + 1;
                        end
                    end
                end

                TX_STOP: begin
                    if (baud_tick) begin
                        tx_out <= 1'b1;  // Stop bit
                        tx_state <= TX_IDLE;
                    end
                end

                default: tx_state <= TX_IDLE;
            endcase
        end
    end

    // RX state machine
    localparam RX_IDLE  = 3'd0;
    localparam RX_START = 3'd1;
    localparam RX_DATA  = 3'd2;
    localparam RX_STOP  = 3'd3;

    reg [2:0] rx_state;
    reg [2:0] rx_bit_index;
    reg [7:0] rx_shift_reg;
    reg [COUNTER_WIDTH-1:0] rx_counter;
    reg rx_in_sync, rx_in_sync2;

    // Synchronize RX input
    always @(posedge clk) begin
        rx_in_sync2 <= rx_in;
        rx_in_sync <= rx_in_sync2;
    end

    always @(posedge clk) begin
        if (rst) begin
            rx_state <= RX_IDLE;
            rx_valid <= 1'b0;
            rx_counter <= 0;
        end else begin
            rx_valid <= 1'b0;  // Default

            case (rx_state)
                RX_IDLE: begin
                    if (rx_in_sync == 1'b0) begin  // Start bit detected
                        rx_counter <= 0;
                        rx_state <= RX_START;
                    end
                end

                RX_START: begin
                    if (rx_counter == CLKS_PER_BIT / 2) begin
                        if (rx_in_sync == 1'b0) begin  // Validate start bit
                            rx_counter <= 0;
                            rx_bit_index <= 0;
                            rx_state <= RX_DATA;
                        end else begin
                            rx_state <= RX_IDLE;  // False start
                        end
                    end else begin
                        rx_counter <= rx_counter + 1;
                    end
                end

                RX_DATA: begin
                    if (rx_counter == CLKS_PER_BIT - 1) begin
                        rx_shift_reg[rx_bit_index] <= rx_in_sync;
                        rx_counter <= 0;
                        if (rx_bit_index == 7) begin
                            rx_state <= RX_STOP;
                        end else begin
                            rx_bit_index <= rx_bit_index + 1;
                        end
                    end else begin
                        rx_counter <= rx_counter + 1;
                    end
                end

                RX_STOP: begin
                    if (rx_counter == CLKS_PER_BIT - 1) begin
                        if (rx_in_sync == 1'b1) begin  // Valid stop bit
                            rx_data <= rx_shift_reg;
                            rx_valid <= 1'b1;
                        end
                        rx_state <= RX_IDLE;
                    end else begin
                        rx_counter <= rx_counter + 1;
                    end
                end

                default: rx_state <= RX_IDLE;
            endcase
        end
    end

endmodule
