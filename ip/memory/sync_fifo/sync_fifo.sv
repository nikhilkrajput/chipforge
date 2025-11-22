// Synchronous FIFO IP Core
// Production-ready FIFO with status flags and configurable thresholds
module sync_fifo #(
    parameter DATA_WIDTH = 8,
    parameter DEPTH = 16,
    parameter ALMOST_FULL_THRESHOLD = DEPTH - 2,
    parameter ALMOST_EMPTY_THRESHOLD = 2
)(
    input  wire                   clk,
    input  wire                   rst,

    // Write interface
    input  wire [DATA_WIDTH-1:0]  wr_data,
    input  wire                   wr_en,
    output wire                   full,
    output wire                   almost_full,

    // Read interface
    output reg  [DATA_WIDTH-1:0]  rd_data,
    input  wire                   rd_en,
    output wire                   empty,
    output wire                   almost_empty,

    // Status
    output wire [$clog2(DEPTH):0] count
);

    localparam ADDR_WIDTH = $clog2(DEPTH);

    reg [DATA_WIDTH-1:0] memory [0:DEPTH-1];
    reg [ADDR_WIDTH:0] wr_ptr, rd_ptr;

    wire [ADDR_WIDTH:0] wr_ptr_next = wr_ptr + 1;
    wire [ADDR_WIDTH:0] rd_ptr_next = rd_ptr + 1;

    // Status flags
    assign full = (wr_ptr_next == rd_ptr);
    assign empty = (wr_ptr == rd_ptr);
    assign count = wr_ptr - rd_ptr;
    assign almost_full = (count >= ALMOST_FULL_THRESHOLD);
    assign almost_empty = (count <= ALMOST_EMPTY_THRESHOLD);

    // Write logic
    always @(posedge clk) begin
        if (rst) begin
            wr_ptr <= 0;
        end else if (wr_en && !full) begin
            memory[wr_ptr[ADDR_WIDTH-1:0]] <= wr_data;
            wr_ptr <= wr_ptr_next;
        end
    end

    // Read logic
    always @(posedge clk) begin
        if (rst) begin
            rd_ptr <= 0;
            rd_data <= 0;
        end else if (rd_en && !empty) begin
            rd_data <= memory[rd_ptr[ADDR_WIDTH-1:0]];
            rd_ptr <= rd_ptr_next;
        end
    end

endmodule
