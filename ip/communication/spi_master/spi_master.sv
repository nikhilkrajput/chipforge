// SPI Master IP Core
// Configurable clock polarity and phase (all 4 SPI modes)
module spi_master #(
    parameter CLK_DIV = 4,      // Clock divisor (system_clk / spi_clk)
    parameter DATA_WIDTH = 8    // Data width in bits
)(
    input  wire                    clk,
    input  wire                    rst,

    // Control interface
    input  wire [DATA_WIDTH-1:0]   tx_data,
    input  wire                    tx_valid,
    output reg                     tx_ready,

    output reg  [DATA_WIDTH-1:0]   rx_data,
    output reg                     rx_valid,

    // Configuration
    input  wire                    cpol,        // Clock polarity
    input  wire                    cpha,        // Clock phase

    // SPI interface
    output reg                     sclk,
    output reg                     mosi,
    input  wire                    miso,
    output reg                     cs_n         // Chip select (active low)
);

    localparam COUNTER_WIDTH = $clog2(CLK_DIV);

    typedef enum logic [1:0] {
        IDLE,
        TRANSFER,
        DONE
    } state_t;

    state_t state;

    reg [COUNTER_WIDTH-1:0] clk_count;
    reg [$clog2(DATA_WIDTH)-1:0] bit_count;
    reg [DATA_WIDTH-1:0] tx_shift_reg;
    reg [DATA_WIDTH-1:0] rx_shift_reg;
    reg sclk_en;

    // SPI clock generation
    wire sclk_rising = (clk_count == (CLK_DIV / 2) - 1);
    wire sclk_falling = (clk_count == CLK_DIV - 1);

    always @(posedge clk) begin
        if (rst) begin
            clk_count <= 0;
            sclk <= cpol;
        end else begin
            if (sclk_en) begin
                if (clk_count == CLK_DIV - 1) begin
                    clk_count <= 0;
                    sclk <= ~sclk;
                end else begin
                    clk_count <= clk_count + 1;
                end
            end else begin
                clk_count <= 0;
                sclk <= cpol;
            end
        end
    end

    // FSM and data shifting
    always @(posedge clk) begin
        if (rst) begin
            state <= IDLE;
            cs_n <= 1'b1;
            mosi <= 1'b0;
            tx_ready <= 1'b1;
            rx_valid <= 1'b0;
            sclk_en <= 1'b0;
            bit_count <= 0;
            tx_shift_reg <= 0;
            rx_shift_reg <= 0;
            rx_data <= 0;
        end else begin
            rx_valid <= 1'b0;  // Default

            case (state)
                IDLE: begin
                    cs_n <= 1'b1;
                    tx_ready <= 1'b1;
                    sclk_en <= 1'b0;
                    bit_count <= 0;

                    if (tx_valid && tx_ready) begin
                        tx_shift_reg <= tx_data;
                        tx_ready <= 1'b0;
                        cs_n <= 1'b0;
                        sclk_en <= 1'b1;
                        state <= TRANSFER;
                    end
                end

                TRANSFER: begin
                    // CPHA = 0: Sample on first edge, shift on second edge
                    // CPHA = 1: Shift on first edge, sample on second edge

                    if (!cpha) begin
                        // Mode 0 or 2
                        if (sclk_rising) begin
                            // Sample MISO
                            rx_shift_reg <= {rx_shift_reg[DATA_WIDTH-2:0], miso};
                        end

                        if (sclk_falling) begin
                            // Shift MOSI
                            mosi <= tx_shift_reg[DATA_WIDTH-1];
                            tx_shift_reg <= {tx_shift_reg[DATA_WIDTH-2:0], 1'b0};

                            if (bit_count == DATA_WIDTH - 1) begin
                                state <= DONE;
                                sclk_en <= 1'b0;
                            end else begin
                                bit_count <= bit_count + 1;
                            end
                        end
                    end else begin
                        // Mode 1 or 3
                        if (sclk_rising) begin
                            // Shift MOSI
                            mosi <= tx_shift_reg[DATA_WIDTH-1];
                            tx_shift_reg <= {tx_shift_reg[DATA_WIDTH-2:0], 1'b0};
                        end

                        if (sclk_falling) begin
                            // Sample MISO
                            rx_shift_reg <= {rx_shift_reg[DATA_WIDTH-2:0], miso};

                            if (bit_count == DATA_WIDTH - 1) begin
                                state <= DONE;
                                sclk_en <= 1'b0;
                            end else begin
                                bit_count <= bit_count + 1;
                            end
                        end
                    end
                end

                DONE: begin
                    cs_n <= 1'b1;
                    rx_data <= rx_shift_reg;
                    rx_valid <= 1'b1;
                    state <= IDLE;
                end

                default: state <= IDLE;
            endcase
        end
    end

endmodule
