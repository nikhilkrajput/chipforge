// Asynchronous FIFO IP Core
// Safe for crossing clock domains using Gray code pointers
module async_fifo #(
    parameter DATA_WIDTH = 8,
    parameter DEPTH = 16,
    parameter ALMOST_FULL_THRESHOLD = DEPTH - 2,
    parameter ALMOST_EMPTY_THRESHOLD = 2
)(
    // Write clock domain
    input  wire                   wr_clk,
    input  wire                   wr_rst,
    input  wire [DATA_WIDTH-1:0]  wr_data,
    input  wire                   wr_en,
    output wire                   full,
    output wire                   almost_full,

    // Read clock domain
    input  wire                   rd_clk,
    input  wire                   rd_rst,
    output reg  [DATA_WIDTH-1:0]  rd_data,
    input  wire                   rd_en,
    output wire                   empty,
    output wire                   almost_empty,

    // Status (in respective clock domains)
    output wire [$clog2(DEPTH):0] wr_count,
    output wire [$clog2(DEPTH):0] rd_count
);

    localparam ADDR_WIDTH = $clog2(DEPTH);
    localparam PTR_WIDTH = ADDR_WIDTH + 1;

    // Memory
    reg [DATA_WIDTH-1:0] memory [0:DEPTH-1];

    // Write domain pointers
    reg [PTR_WIDTH-1:0] wr_ptr;
    reg [PTR_WIDTH-1:0] wr_ptr_gray;
    reg [PTR_WIDTH-1:0] rd_ptr_gray_sync1, rd_ptr_gray_sync2;

    // Read domain pointers
    reg [PTR_WIDTH-1:0] rd_ptr;
    reg [PTR_WIDTH-1:0] rd_ptr_gray;
    reg [PTR_WIDTH-1:0] wr_ptr_gray_sync1, wr_ptr_gray_sync2;

    // Convert binary to Gray code
    function [PTR_WIDTH-1:0] bin2gray;
        input [PTR_WIDTH-1:0] bin;
        begin
            bin2gray = bin ^ (bin >> 1);
        end
    endfunction

    // Convert Gray code to binary
    function [PTR_WIDTH-1:0] gray2bin;
        input [PTR_WIDTH-1:0] gray;
        integer i;
        begin
            gray2bin[PTR_WIDTH-1] = gray[PTR_WIDTH-1];
            for (i = PTR_WIDTH-2; i >= 0; i = i - 1) begin
                gray2bin[i] = gray2bin[i+1] ^ gray[i];
            end
        end
    endfunction

    // Write domain logic
    always @(posedge wr_clk) begin
        if (wr_rst) begin
            wr_ptr <= 0;
            wr_ptr_gray <= 0;
        end else if (wr_en && !full) begin
            memory[wr_ptr[ADDR_WIDTH-1:0]] <= wr_data;
            wr_ptr <= wr_ptr + 1;
            wr_ptr_gray <= bin2gray(wr_ptr + 1);
        end
    end

    // Synchronize read pointer to write clock domain
    always @(posedge wr_clk) begin
        if (wr_rst) begin
            rd_ptr_gray_sync1 <= 0;
            rd_ptr_gray_sync2 <= 0;
        end else begin
            rd_ptr_gray_sync1 <= rd_ptr_gray;
            rd_ptr_gray_sync2 <= rd_ptr_gray_sync1;
        end
    end

    // Read domain logic
    always @(posedge rd_clk) begin
        if (rd_rst) begin
            rd_ptr <= 0;
            rd_ptr_gray <= 0;
            rd_data <= 0;
        end else if (rd_en && !empty) begin
            rd_data <= memory[rd_ptr[ADDR_WIDTH-1:0]];
            rd_ptr <= rd_ptr + 1;
            rd_ptr_gray <= bin2gray(rd_ptr + 1);
        end
    end

    // Synchronize write pointer to read clock domain
    always @(posedge rd_clk) begin
        if (rd_rst) begin
            wr_ptr_gray_sync1 <= 0;
            wr_ptr_gray_sync2 <= 0;
        end else begin
            wr_ptr_gray_sync1 <= wr_ptr_gray;
            wr_ptr_gray_sync2 <= wr_ptr_gray_sync1;
        end
    end

    // Status flags
    wire [PTR_WIDTH-1:0] wr_ptr_next = wr_ptr + 1;
    wire [PTR_WIDTH-1:0] rd_ptr_sync_wr = gray2bin(rd_ptr_gray_sync2);
    wire [PTR_WIDTH-1:0] wr_ptr_sync_rd = gray2bin(wr_ptr_gray_sync2);

    assign full = (wr_ptr_next == rd_ptr_sync_wr);
    assign empty = (rd_ptr == wr_ptr_sync_rd);

    assign wr_count = wr_ptr - rd_ptr_sync_wr;
    assign rd_count = wr_ptr_sync_rd - rd_ptr;

    assign almost_full = (wr_count >= ALMOST_FULL_THRESHOLD);
    assign almost_empty = (rd_count <= ALMOST_EMPTY_THRESHOLD);

endmodule
