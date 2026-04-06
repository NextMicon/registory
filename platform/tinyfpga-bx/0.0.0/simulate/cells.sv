`timescale 1 ns / 1 ps

// Simulation-friendly replacement for SB_IO-based tristate module
module tristate (
    inout  pin,
    input  iosel,
    input  out,
    output in
);
  assign pin = iosel ? out : 1'bz;
  assign in  = pin;
endmodule
