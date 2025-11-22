// Timer/Counter IP Core
// Configurable timer with multiple modes
module timer #(
    parameter WIDTH = 32
)(
    input  wire              clk,
    input  wire              rst,

    // Configuration
    input  wire [WIDTH-1:0]  period,        // Timer period
    input  wire              enable,        // Timer enable
    input  wire              oneshot,       // 1=one-shot, 0=continuous
    input  wire              count_up,      // 1=count up, 0=count down

    // Status
    output reg  [WIDTH-1:0]  count,         // Current count value
    output reg               timeout,       // Timeout pulse
    output reg               running        // Timer is running
);

    reg [WIDTH-1:0] counter;
    reg oneshot_done;

    always @(posedge clk) begin
        if (rst) begin
            counter <= 0;
            count <= 0;
            timeout <= 1'b0;
            running <= 1'b0;
            oneshot_done <= 1'b0;
        end else begin
            timeout <= 1'b0;  // Default

            if (enable && !oneshot_done) begin
                running <= 1'b1;

                if (count_up) begin
                    // Count up mode
                    if (counter >= period) begin
                        counter <= 0;
                        timeout <= 1'b1;

                        if (oneshot) begin
                            oneshot_done <= 1'b1;
                            running <= 1'b0;
                        end
                    end else begin
                        counter <= counter + 1;
                    end
                end else begin
                    // Count down mode
                    if (counter == 0) begin
                        counter <= period;
                        timeout <= 1'b1;

                        if (oneshot) begin
                            oneshot_done <= 1'b1;
                            running <= 1'b0;
                        end
                    end else begin
                        counter <= counter - 1;
                    end
                end

                count <= counter;
            end else begin
                running <= 1'b0;

                if (!enable) begin
                    // Reset when disabled
                    counter <= count_up ? 0 : period;
                    count <= count_up ? 0 : period;
                    oneshot_done <= 1'b0;
                end
            end
        end
    end

endmodule
