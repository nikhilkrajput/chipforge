// I2C Master IP Core
// Supports standard (100kHz) and fast mode (400kHz)
module i2c_master #(
    parameter CLK_FREQ = 100_000_000,   // System clock frequency
    parameter I2C_FREQ = 100_000        // I2C clock frequency
)(
    input  wire        clk,
    input  wire        rst,

    // Command interface
    input  wire [6:0]  slave_addr,
    input  wire        rw,              // 0=write, 1=read
    input  wire [7:0]  tx_data,
    input  wire        tx_valid,
    output reg         tx_ready,

    output reg  [7:0]  rx_data,
    output reg         rx_valid,

    output reg         busy,
    output reg         ack_error,

    // I2C interface
    inout  wire        sda,
    inout  wire        scl
);

    localparam DIVIDER = CLK_FREQ / (4 * I2C_FREQ);

    typedef enum logic [3:0] {
        IDLE,
        START,
        ADDR,
        ADDR_ACK,
        DATA_WR,
        DATA_WR_ACK,
        DATA_RD,
        DATA_RD_ACK,
        STOP
    } state_t;

    state_t state;

    reg [$clog2(DIVIDER)-1:0] clk_count;
    reg [2:0] bit_count;
    reg [7:0] shift_reg;
    reg sda_out, scl_out;
    reg sda_oe, scl_oe;

    wire sda_in = sda;
    wire scl_in = scl;

    // Tristate control
    assign sda = sda_oe ? sda_out : 1'bz;
    assign scl = scl_oe ? scl_out : 1'bz;

    // Clock generation
    wire tick = (clk_count == DIVIDER - 1);

    always @(posedge clk) begin
        if (rst || state == IDLE) begin
            clk_count <= 0;
        end else if (tick) begin
            clk_count <= 0;
        end else begin
            clk_count <= clk_count + 1;
        end
    end

    // FSM
    always @(posedge clk) begin
        if (rst) begin
            state <= IDLE;
            sda_out <= 1'b1;
            scl_out <= 1'b1;
            sda_oe <= 1'b0;
            scl_oe <= 1'b0;
            tx_ready <= 1'b1;
            rx_valid <= 1'b0;
            busy <= 1'b0;
            ack_error <= 1'b0;
            bit_count <= 0;
            shift_reg <= 8'h00;
            rx_data <= 8'h00;
        end else begin
            rx_valid <= 1'b0;  // Default

            case (state)
                IDLE: begin
                    sda_out <= 1'b1;
                    scl_out <= 1'b1;
                    sda_oe <= 1'b0;
                    scl_oe <= 1'b0;
                    tx_ready <= 1'b1;
                    busy <= 1'b0;
                    ack_error <= 1'b0;

                    if (tx_valid && tx_ready) begin
                        shift_reg <= {slave_addr, rw};
                        tx_ready <= 1'b0;
                        busy <= 1'b1;
                        state <= START;
                    end
                end

                START: begin
                    if (tick) begin
                        sda_out <= 1'b0;
                        sda_oe <= 1'b1;
                        scl_oe <= 1'b1;
                        bit_count <= 7;
                        state <= ADDR;
                    end
                end

                ADDR: begin
                    if (tick) begin
                        if (bit_count == 7) begin
                            scl_out <= 1'b0;
                        end

                        sda_out <= shift_reg[7];
                        shift_reg <= {shift_reg[6:0], 1'b0};

                        if (bit_count == 0) begin
                            state <= ADDR_ACK;
                        end else begin
                            bit_count <= bit_count - 1;
                        end

                        scl_out <= ~scl_out;
                    end
                end

                ADDR_ACK: begin
                    if (tick) begin
                        sda_oe <= 1'b0;  // Release SDA for ACK

                        if (!scl_out) begin
                            scl_out <= 1'b1;
                        end else begin
                            if (sda_in) begin
                                // NAK received
                                ack_error <= 1'b1;
                                state <= STOP;
                            end else begin
                                // ACK received
                                if (rw) begin
                                    state <= DATA_RD;
                                    bit_count <= 7;
                                end else begin
                                    shift_reg <= tx_data;
                                    state <= DATA_WR;
                                    bit_count <= 7;
                                end
                            end
                            scl_out <= 1'b0;
                        end
                    end
                end

                DATA_WR: begin
                    if (tick) begin
                        sda_oe <= 1'b1;
                        sda_out <= shift_reg[7];
                        shift_reg <= {shift_reg[6:0], 1'b0};

                        if (bit_count == 0) begin
                            state <= DATA_WR_ACK;
                        end else begin
                            bit_count <= bit_count - 1;
                        end

                        scl_out <= ~scl_out;
                    end
                end

                DATA_WR_ACK: begin
                    if (tick) begin
                        sda_oe <= 1'b0;

                        if (!scl_out) begin
                            scl_out <= 1'b1;
                        end else begin
                            if (sda_in) begin
                                ack_error <= 1'b1;
                            end
                            scl_out <= 1'b0;
                            state <= STOP;
                        end
                    end
                end

                DATA_RD: begin
                    if (tick) begin
                        sda_oe <= 1'b0;  // Release SDA for reading

                        if (scl_out) begin
                            shift_reg <= {shift_reg[6:0], sda_in};

                            if (bit_count == 0) begin
                                state <= DATA_RD_ACK;
                            end else begin
                                bit_count <= bit_count - 1;
                            end
                        end

                        scl_out <= ~scl_out;
                    end
                end

                DATA_RD_ACK: begin
                    if (tick) begin
                        sda_oe <= 1'b1;
                        sda_out <= 1'b0;  // Send ACK

                        if (!scl_out) begin
                            scl_out <= 1'b1;
                        end else begin
                            rx_data <= shift_reg;
                            rx_valid <= 1'b1;
                            scl_out <= 1'b0;
                            state <= STOP;
                        end
                    end
                end

                STOP: begin
                    if (tick) begin
                        if (!scl_out) begin
                            sda_out <= 1'b0;
                            sda_oe <= 1'b1;
                            scl_out <= 1'b1;
                        end else begin
                            sda_out <= 1'b1;
                            state <= IDLE;
                        end
                    end
                end

                default: state <= IDLE;
            endcase
        end
    end

endmodule
