// Single-Port RAM IP Core
// Synchronous read/write with byte-enable support
module sp_ram #(
    parameter DATA_WIDTH = 32,
    parameter ADDR_WIDTH = 10,
    parameter BYTE_ENABLE = 1
)(
    input  wire                       clk,

    // Port A
    input  wire [ADDR_WIDTH-1:0]      addr,
    input  wire [DATA_WIDTH-1:0]      wr_data,
    input  wire [(DATA_WIDTH/8)-1:0]  wr_en,     // Byte enable
    input  wire                       rd_en,
    output reg  [DATA_WIDTH-1:0]      rd_data
);

    localparam DEPTH = 2**ADDR_WIDTH;
    localparam BYTES = DATA_WIDTH / 8;

    reg [DATA_WIDTH-1:0] memory [0:DEPTH-1];

    integer i;

    always @(posedge clk) begin
        // Write operation with byte enables
        if (BYTE_ENABLE) begin
            for (i = 0; i < BYTES; i = i + 1) begin
                if (wr_en[i]) begin
                    memory[addr][i*8 +: 8] <= wr_data[i*8 +: 8];
                end
            end
        end else begin
            if (|wr_en) begin
                memory[addr] <= wr_data;
            end
        end

        // Read operation
        if (rd_en) begin
            rd_data <= memory[addr];
        end
    end

    // Initialize memory to zero
    integer j;
    initial begin
        for (j = 0; j < DEPTH; j = j + 1) begin
            memory[j] = {DATA_WIDTH{1'b0}};
        end
    end

endmodule
