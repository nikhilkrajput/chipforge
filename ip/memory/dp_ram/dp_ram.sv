// True Dual-Port RAM IP Core
// Two independent read/write ports
module dp_ram #(
    parameter DATA_WIDTH = 32,
    parameter ADDR_WIDTH = 10
)(
    // Port A
    input  wire                   clk_a,
    input  wire [ADDR_WIDTH-1:0]  addr_a,
    input  wire [DATA_WIDTH-1:0]  wr_data_a,
    input  wire                   wr_en_a,
    input  wire                   rd_en_a,
    output reg  [DATA_WIDTH-1:0]  rd_data_a,

    // Port B
    input  wire                   clk_b,
    input  wire [ADDR_WIDTH-1:0]  addr_b,
    input  wire [DATA_WIDTH-1:0]  wr_data_b,
    input  wire                   wr_en_b,
    input  wire                   rd_en_b,
    output reg  [DATA_WIDTH-1:0]  rd_data_b
);

    localparam DEPTH = 2**ADDR_WIDTH;

    reg [DATA_WIDTH-1:0] memory [0:DEPTH-1];

    // Port A
    always @(posedge clk_a) begin
        if (wr_en_a) begin
            memory[addr_a] <= wr_data_a;
        end

        if (rd_en_a) begin
            rd_data_a <= memory[addr_a];
        end
    end

    // Port B
    always @(posedge clk_b) begin
        if (wr_en_b) begin
            memory[addr_b] <= wr_data_b;
        end

        if (rd_en_b) begin
            rd_data_b <= memory[addr_b];
        end
    end

    // Initialize memory
    integer i;
    initial begin
        for (i = 0; i < DEPTH; i = i + 1) begin
            memory[i] = {DATA_WIDTH{1'b0}};
        end
    end

endmodule
