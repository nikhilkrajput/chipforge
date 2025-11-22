// ROM IP Core
// Parameterized read-only memory with initialization file support
module rom #(
    parameter DATA_WIDTH = 32,
    parameter ADDR_WIDTH = 10,
    parameter INIT_FILE = ""
)(
    input  wire                   clk,
    input  wire [ADDR_WIDTH-1:0]  addr,
    input  wire                   rd_en,
    output reg  [DATA_WIDTH-1:0]  rd_data
);

    localparam DEPTH = 2**ADDR_WIDTH;

    reg [DATA_WIDTH-1:0] memory [0:DEPTH-1];

    // Read operation
    always @(posedge clk) begin
        if (rd_en) begin
            rd_data <= memory[addr];
        end
    end

    // Initialize ROM contents
    integer i;
    initial begin
        if (INIT_FILE != "") begin
            $readmemh(INIT_FILE, memory);
        end else begin
            // Default initialization
            for (i = 0; i < DEPTH; i = i + 1) begin
                memory[i] = i[DATA_WIDTH-1:0];
            end
        end
    end

endmodule
