// SPI Master controller
module spi_master #(
    parameter CLK_DIV = 4  // SPI clock = clk / (2 * CLK_DIV)
)(
    input wire clk,
    input wire rst,

    // Control interface
    input wire [7:0] tx_data,
    input wire start,
    output reg busy,
    output reg [7:0] rx_data,
    output reg done,

    // SPI interface
    output reg sclk,
    output reg mosi,
    input wire miso,
    output reg cs_n
);

    reg [7:0] tx_shift_reg;
    reg [7:0] rx_shift_reg;
    reg [2:0] bit_counter;
    reg [3:0] clk_counter;
    reg sclk_en;

    localparam IDLE     = 2'd0;
    localparam TRANSFER = 2'd1;
    localparam FINISH   = 2'd2;

    reg [1:0] state;

    // SPI clock generation
    always @(posedge clk) begin
        if (rst || !sclk_en) begin
            clk_counter <= 0;
            sclk <= 0;
        end else begin
            if (clk_counter == CLK_DIV - 1) begin
                clk_counter <= 0;
                sclk <= ~sclk;
            end else begin
                clk_counter <= clk_counter + 1;
            end
        end
    end

    // Main state machine
    always @(posedge clk) begin
        if (rst) begin
            state <= IDLE;
            cs_n <= 1'b1;
            busy <= 1'b0;
            done <= 1'b0;
            sclk_en <= 1'b0;
            bit_counter <= 0;
        end else begin
            done <= 1'b0;  // Default

            case (state)
                IDLE: begin
                    cs_n <= 1'b1;
                    sclk_en <= 1'b0;
                    busy <= 1'b0;

                    if (start) begin
                        tx_shift_reg <= tx_data;
                        bit_counter <= 0;
                        cs_n <= 1'b0;
                        busy <= 1'b1;
                        sclk_en <= 1'b1;
                        state <= TRANSFER;
                    end
                end

                TRANSFER: begin
                    if (clk_counter == 0) begin
                        if (!sclk) begin  // Rising edge of SCLK
                            mosi <= tx_shift_reg[7];
                        end else begin  // Falling edge of SCLK
                            rx_shift_reg <= {rx_shift_reg[6:0], miso};
                            tx_shift_reg <= {tx_shift_reg[6:0], 1'b0};

                            if (bit_counter == 7) begin
                                state <= FINISH;
                                sclk_en <= 1'b0;
                            end else begin
                                bit_counter <= bit_counter + 1;
                            end
                        end
                    end
                end

                FINISH: begin
                    cs_n <= 1'b1;
                    rx_data <= rx_shift_reg;
                    done <= 1'b1;
                    busy <= 1'b0;
                    state <= IDLE;
                end

                default: state <= IDLE;
            endcase
        end
    end

endmodule
