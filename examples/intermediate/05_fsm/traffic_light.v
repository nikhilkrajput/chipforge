// Traffic light controller - FSM example
module traffic_light (
    input wire clk,
    input wire rst,
    input wire sensor,      // Vehicle sensor on side street
    output reg [1:0] main_light,   // 00=red, 01=yellow, 10=green
    output reg [1:0] side_light
);

    // Light encodings
    localparam RED    = 2'b00;
    localparam YELLOW = 2'b01;
    localparam GREEN  = 2'b10;

    // State encoding
    localparam S_MAIN_GREEN  = 3'b000;
    localparam S_MAIN_YELLOW = 3'b001;
    localparam S_SIDE_GREEN  = 3'b010;
    localparam S_SIDE_YELLOW = 3'b011;

    reg [2:0] state, next_state;
    reg [7:0] timer;
    reg [7:0] next_timer;

    // Timing constants
    localparam MAIN_GREEN_TIME  = 8'd100;  // Main street green duration
    localparam SIDE_GREEN_TIME  = 8'd50;   // Side street green duration
    localparam YELLOW_TIME      = 8'd20;   // Yellow light duration
    localparam MIN_GREEN_TIME   = 8'd30;   // Minimum green before change

    // State register
    always @(posedge clk) begin
        if (rst) begin
            state <= S_MAIN_GREEN;
            timer <= MAIN_GREEN_TIME;
        end else begin
            state <= next_state;
            timer <= next_timer;
        end
    end

    // Next state logic
    always @(*) begin
        next_state = state;
        next_timer = timer;

        if (timer > 0) begin
            next_timer = timer - 1;
        end

        case (state)
            S_MAIN_GREEN: begin
                if (timer == 0) begin
                    if (sensor) begin
                        next_state = S_MAIN_YELLOW;
                        next_timer = YELLOW_TIME;
                    end else begin
                        next_timer = MAIN_GREEN_TIME;
                    end
                end
            end

            S_MAIN_YELLOW: begin
                if (timer == 0) begin
                    next_state = S_SIDE_GREEN;
                    next_timer = SIDE_GREEN_TIME;
                end
            end

            S_SIDE_GREEN: begin
                if (timer == 0) begin
                    next_state = S_SIDE_YELLOW;
                    next_timer = YELLOW_TIME;
                end
            end

            S_SIDE_YELLOW: begin
                if (timer == 0) begin
                    next_state = S_MAIN_GREEN;
                    next_timer = MAIN_GREEN_TIME;
                end
            end

            default: begin
                next_state = S_MAIN_GREEN;
                next_timer = MAIN_GREEN_TIME;
            end
        endcase
    end

    // Output logic
    always @(*) begin
        case (state)
            S_MAIN_GREEN: begin
                main_light = GREEN;
                side_light = RED;
            end
            S_MAIN_YELLOW: begin
                main_light = YELLOW;
                side_light = RED;
            end
            S_SIDE_GREEN: begin
                main_light = RED;
                side_light = GREEN;
            end
            S_SIDE_YELLOW: begin
                main_light = RED;
                side_light = YELLOW;
            end
            default: begin
                main_light = RED;
                side_light = RED;
            end
        endcase
    end

endmodule
