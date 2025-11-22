// SPI Slave IP Core
// Full-featured SPI slave supporting all 4 modes
module spi_slave #(
    parameter DATA_WIDTH = 8
)(
    input  wire                    clk,
    input  wire                    rst,

    // Data interface
    input  wire [DATA_WIDTH-1:0]   tx_data,
    input  wire                    tx_valid,
    output reg                     tx_ready,

    output reg  [DATA_WIDTH-1:0]   rx_data,
    output reg                     rx_valid,

    // Configuration
    input  wire                    cpol,        // Clock polarity
    input  wire                    cpha,        // Clock phase

    // SPI interface
    input  wire                    sclk,
    input  wire                    mosi,
    output reg                     miso,
    input  wire                    cs_n         // Chip select (active low)
);

    // Synchronize SPI signals
    reg sclk_sync1, sclk_sync2, sclk_sync3;
    reg mosi_sync1, mosi_sync2;
    reg cs_n_sync1, cs_n_sync2;

    // Edge detection
    wire sclk_rising = sclk_sync2 && !sclk_sync3;
    wire sclk_falling = !sclk_sync2 && sclk_sync3;
    wire cs_n_rising = cs_n_sync2 && !cs_n_sync1;

    // Shift registers
    reg [DATA_WIDTH-1:0] tx_shift_reg;
    reg [DATA_WIDTH-1:0] rx_shift_reg;
    reg [$clog2(DATA_WIDTH)-1:0] bit_count;

    // Synchronize inputs
    always @(posedge clk) begin
        if (rst) begin
            sclk_sync1 <= cpol;
            sclk_sync2 <= cpol;
            sclk_sync3 <= cpol;
            mosi_sync1 <= 1'b0;
            mosi_sync2 <= 1'b0;
            cs_n_sync1 <= 1'b1;
            cs_n_sync2 <= 1'b1;
        end else begin
            sclk_sync1 <= sclk;
            sclk_sync2 <= sclk_sync1;
            sclk_sync3 <= sclk_sync2;
            mosi_sync1 <= mosi;
            mosi_sync2 <= mosi_sync1;
            cs_n_sync1 <= cs_n;
            cs_n_sync2 <= cs_n_sync1;
        end
    end

    // Main SPI logic
    always @(posedge clk) begin
        if (rst) begin
            tx_shift_reg <= 0;
            rx_shift_reg <= 0;
            miso <= 1'b0;
            tx_ready <= 1'b1;
            rx_valid <= 1'b0;
            rx_data <= 0;
            bit_count <= 0;
        end else begin
            rx_valid <= 1'b0;  // Default

            if (cs_n_sync2) begin
                // Not selected - idle
                tx_ready <= 1'b1;
                bit_count <= 0;

                if (tx_valid && tx_ready) begin
                    tx_shift_reg <= tx_data;
                    tx_ready <= 1'b0;
                end
            end else begin
                // Selected - active transfer
                if (bit_count == 0 && !cpha) begin
                    // CPHA=0: Put first bit immediately
                    miso <= tx_shift_reg[DATA_WIDTH-1];
                end

                if (!cpha) begin
                    // Mode 0 or 2: Sample on first edge, shift on second
                    if ((cpol && sclk_falling) || (!cpol && sclk_rising)) begin
                        // Sample edge
                        rx_shift_reg <= {rx_shift_reg[DATA_WIDTH-2:0], mosi_sync2};
                    end

                    if ((cpol && sclk_rising) || (!cpol && sclk_falling)) begin
                        // Shift edge
                        tx_shift_reg <= {tx_shift_reg[DATA_WIDTH-2:0], 1'b0};
                        miso <= tx_shift_reg[DATA_WIDTH-1];

                        if (bit_count == DATA_WIDTH - 1) begin
                            bit_count <= 0;
                            rx_data <= {rx_shift_reg[DATA_WIDTH-2:0], mosi_sync2};
                            rx_valid <= 1'b1;
                            tx_ready <= 1'b1;
                        end else begin
                            bit_count <= bit_count + 1;
                        end
                    end
                end else begin
                    // Mode 1 or 3: Shift on first edge, sample on second
                    if ((cpol && sclk_falling) || (!cpol && sclk_rising)) begin
                        // Shift edge
                        tx_shift_reg <= {tx_shift_reg[DATA_WIDTH-2:0], 1'b0};
                        miso <= tx_shift_reg[DATA_WIDTH-1];
                    end

                    if ((cpol && sclk_rising) || (!cpol && sclk_falling)) begin
                        // Sample edge
                        rx_shift_reg <= {rx_shift_reg[DATA_WIDTH-2:0], mosi_sync2};

                        if (bit_count == DATA_WIDTH - 1) begin
                            bit_count <= 0;
                            rx_data <= {rx_shift_reg[DATA_WIDTH-2:0], mosi_sync2};
                            rx_valid <= 1'b1;
                            tx_ready <= 1'b1;
                        end else begin
                            bit_count <= bit_count + 1;
                        end
                    end
                end
            end

            // Handle CS rising edge - end of transaction
            if (cs_n_rising) begin
                tx_ready <= 1'b1;
                bit_count <= 0;
            end
        end
    end

endmodule
