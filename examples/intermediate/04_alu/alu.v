// Arithmetic Logic Unit
module alu #(
    parameter WIDTH = 32
)(
    input wire [WIDTH-1:0] a,
    input wire [WIDTH-1:0] b,
    input wire [3:0] op,
    output reg [WIDTH-1:0] result,
    output reg zero,
    output reg overflow,
    output reg carry
);

    // ALU operation codes
    localparam OP_ADD  = 4'b0000;
    localparam OP_SUB  = 4'b0001;
    localparam OP_AND  = 4'b0010;
    localparam OP_OR   = 4'b0011;
    localparam OP_XOR  = 4'b0100;
    localparam OP_SLL  = 4'b0101;  // Shift left logical
    localparam OP_SRL  = 4'b0110;  // Shift right logical
    localparam OP_SRA  = 4'b0111;  // Shift right arithmetic
    localparam OP_SLT  = 4'b1000;  // Set less than (signed)
    localparam OP_SLTU = 4'b1001;  // Set less than (unsigned)
    localparam OP_NOR  = 4'b1010;
    localparam OP_NAND = 4'b1011;

    reg [WIDTH:0] temp_result;  // Extra bit for carry

    always @(*) begin
        temp_result = {1'b0, {WIDTH{1'b0}}};
        carry = 1'b0;
        overflow = 1'b0;

        case (op)
            OP_ADD: begin
                temp_result = {1'b0, a} + {1'b0, b};
                carry = temp_result[WIDTH];
                overflow = (a[WIDTH-1] == b[WIDTH-1]) &&
                          (result[WIDTH-1] != a[WIDTH-1]);
            end

            OP_SUB: begin
                temp_result = {1'b0, a} - {1'b0, b};
                carry = temp_result[WIDTH];
                overflow = (a[WIDTH-1] != b[WIDTH-1]) &&
                          (result[WIDTH-1] != a[WIDTH-1]);
            end

            OP_AND:  temp_result = {1'b0, a & b};
            OP_OR:   temp_result = {1'b0, a | b};
            OP_XOR:  temp_result = {1'b0, a ^ b};
            OP_NOR:  temp_result = {1'b0, ~(a | b)};
            OP_NAND: temp_result = {1'b0, ~(a & b)};

            OP_SLL:  temp_result = {1'b0, a << b[4:0]};
            OP_SRL:  temp_result = {1'b0, a >> b[4:0]};
            OP_SRA:  temp_result = {1'b0, $signed(a) >>> b[4:0]};

            OP_SLT:  temp_result = {1'b0, ($signed(a) < $signed(b)) ? {{WIDTH-1{1'b0}}, 1'b1} : {WIDTH{1'b0}}};
            OP_SLTU: temp_result = {1'b0, (a < b) ? {{WIDTH-1{1'b0}}, 1'b1} : {WIDTH{1'b0}}};

            default: temp_result = {1'b0, {WIDTH{1'b0}}};
        endcase

        result = temp_result[WIDTH-1:0];
        zero = (result == {WIDTH{1'b0}});
    end

endmodule
