// grape: oscillator-driven LED and first GPIO. rstn is the active-low user button.
module top(input clk, input rstn, output led0, output gpio1);
    reg [25:0] counter = 0;
    reg [1:0] reset_sync = 0;
    always @(posedge clk) begin
        reset_sync <= {reset_sync[0], rstn};
        if (!reset_sync[1]) counter <= 0;
        else counter <= counter + 1'b1;
    end
    assign led0 = counter[25];
    assign gpio1 = counter[24];
endmodule
