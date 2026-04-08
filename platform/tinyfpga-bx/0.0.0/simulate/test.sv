`timescale 1 ns / 1 ps

module tb;

  /////////////////////////////////////
  // Clock

  reg clk = 0;
  always #5 clk = ~clk;

  /////////////////////////////////////
  // Simulation

  parameter CYCLE = 500_0000;
  initial begin
    $dumpfile("build/simulation.vcd");
    $dumpvars(0, tb);
    // Initialize POR counter (iverilog needs this)
    dut.u_por.cnt = 0;
    repeat (CYCLE) @(posedge clk);
    $finish;
  end

  /////////////////////////////////////
  // Async events

  integer cycle_cnt = 0;
  reg irq_5 = 0;

  always @(posedge clk) begin
    cycle_cnt <= cycle_cnt + 1;
    irq_5 <= 0;
    case (cycle_cnt)
      10_0000: irq_5 <= 1;
    endcase
  end

  /////////////////////////////////////
  // DUT

  wire flash_csb, flash_clk, flash_io0, flash_io1, flash_io2, flash_io3;

  top dut (
      .clk       (clk),
      .flash_csb (flash_csb),
      .flash_clk (flash_clk),
      .flash_io0 (flash_io0),
      .flash_io1 (flash_io1),
      .flash_io2 (flash_io2),
      .flash_io3 (flash_io3)
  );

  /////////////////////////////////////
  // SPI Flash

  spiflash spiflash (
      .csb(flash_csb),
      .clk(flash_clk),
      .io0(flash_io0),
      .io1(flash_io1),
      .io2(flash_io2),
      .io3(flash_io3)
  );

  /////////////////////////////////////
  // UART Serial Monitor
  //
  // __       ______ ______   ______ _____
  //   |__S__/__D0__X__D1__XXX__D7__/  S
  //    Start                        Stop
  //

  parameter CLKDIV = 139;  // 16MHz / 115200

  reg [7:0] serial_receive_buffer = 0;
  always begin
    @(negedge dut.u_main.tx);  // begin receiving
    repeat (CLKDIV) @(posedge clk);  // start bit
    repeat (8) begin
      repeat (CLKDIV) @(posedge clk);  // data bit
      serial_receive_buffer = {dut.u_main.tx, serial_receive_buffer[7:1]};
    end
    repeat (CLKDIV) @(posedge clk);  // stop bit
    if (serial_receive_buffer < 32 || serial_receive_buffer >= 127)
      $display("Serial data: %d", serial_receive_buffer);
    else $display("Serial data: '%c'", serial_receive_buffer);
  end

endmodule


`timescale 1 ns / 1 ps

module spiflash (
    input csb,
    input clk,
    inout io0,
    inout io1,
    inout io2,
    inout io3
);
  localparam verbose = 0;
  localparam integer latency = 8;

  reg [7:0] buffer;
  integer bitcount = 0;
  integer bytecount = 0;
  integer dummycount = 0;

  reg [7:0] spi_cmd;
  reg [7:0] xip_cmd = 0;
  reg [23:0] spi_addr;

  reg [7:0] spi_in;
  reg [7:0] spi_out;
  reg spi_io_vld;

  reg powered_up = 0;

  localparam [3:0] mode_spi = 1;
  localparam [3:0] mode_dspi_rd = 2;
  localparam [3:0] mode_dspi_wr = 3;
  localparam [3:0] mode_qspi_rd = 4;
  localparam [3:0] mode_qspi_wr = 5;
  localparam [3:0] mode_qspi_ddr_rd = 6;
  localparam [3:0] mode_qspi_ddr_wr = 7;

  reg [3:0] mode = 0;
  reg [3:0] next_mode = 0;

  reg io0_oe = 0;
  reg io1_oe = 0;
  reg io2_oe = 0;
  reg io3_oe = 0;

  reg io0_dout = 0;
  reg io1_dout = 0;
  reg io2_dout = 0;
  reg io3_dout = 0;

  assign #1 io0 = io0_oe ? io0_dout : 1'bz;
  assign #1 io1 = io1_oe ? io1_dout : 1'bz;
  assign #1 io2 = io2_oe ? io2_dout : 1'bz;
  assign #1 io3 = io3_oe ? io3_dout : 1'bz;

  wire io0_delayed;
  wire io1_delayed;
  wire io2_delayed;
  wire io3_delayed;

  assign #1 io0_delayed = io0;
  assign #1 io1_delayed = io1;
  assign #1 io2_delayed = io2;
  assign #1 io3_delayed = io3;

  reg [7:0] memory[0:16*1024*1024-1];

  initial begin
    $readmemh("build/simu_flash.hex", memory);
  end

  task spi_action;
    begin
      spi_in = buffer;
      if (bytecount == 1) begin
        spi_cmd = buffer;
        if (spi_cmd == 8'hab) powered_up = 1;
        if (spi_cmd == 8'hb9) powered_up = 0;
        if (spi_cmd == 8'hff) xip_cmd = 0;
      end
      if (powered_up && spi_cmd == 'h03) begin
        if (bytecount == 2) spi_addr[23:16] = buffer;
        if (bytecount == 3) spi_addr[15:8] = buffer;
        if (bytecount == 4) spi_addr[7:0] = buffer;
        if (bytecount >= 4) begin
          buffer = memory[spi_addr];
          spi_addr = spi_addr + 1;
        end
      end
      if (powered_up && spi_cmd == 'hbb) begin
        if (bytecount == 1) mode = mode_dspi_rd;
        if (bytecount == 2) spi_addr[23:16] = buffer;
        if (bytecount == 3) spi_addr[15:8] = buffer;
        if (bytecount == 4) spi_addr[7:0] = buffer;
        if (bytecount == 5) begin
          xip_cmd = (buffer == 8'ha5) ? spi_cmd : 8'h00;
          mode = mode_dspi_wr;
          dummycount = latency;
        end
        if (bytecount >= 5) begin
          buffer = memory[spi_addr];
          spi_addr = spi_addr + 1;
        end
      end
      if (powered_up && spi_cmd == 'heb) begin
        if (bytecount == 1) mode = mode_qspi_rd;
        if (bytecount == 2) spi_addr[23:16] = buffer;
        if (bytecount == 3) spi_addr[15:8] = buffer;
        if (bytecount == 4) spi_addr[7:0] = buffer;
        if (bytecount == 5) begin
          xip_cmd = (buffer == 8'ha5) ? spi_cmd : 8'h00;
          mode = mode_qspi_wr;
          dummycount = latency;
        end
        if (bytecount >= 5) begin
          buffer = memory[spi_addr];
          spi_addr = spi_addr + 1;
        end
      end
      if (powered_up && spi_cmd == 'hed) begin
        if (bytecount == 1) next_mode = mode_qspi_ddr_rd;
        if (bytecount == 2) spi_addr[23:16] = buffer;
        if (bytecount == 3) spi_addr[15:8] = buffer;
        if (bytecount == 4) spi_addr[7:0] = buffer;
        if (bytecount == 5) begin
          xip_cmd = (buffer == 8'ha5) ? spi_cmd : 8'h00;
          mode = mode_qspi_ddr_wr;
          dummycount = latency;
        end
        if (bytecount >= 5) begin
          buffer = memory[spi_addr];
          spi_addr = spi_addr + 1;
        end
      end
      spi_out = buffer;
      spi_io_vld = 1;
      if (verbose) begin
        if (bytecount == 1) $write("<SPI-START>");
        $write("<SPI:%02x:%02x>", spi_in, spi_out);
      end
    end
  endtask

  task ddr_rd_edge;
    begin
      buffer = {buffer, io3_delayed, io2_delayed, io1_delayed, io0_delayed};
      bitcount = bitcount + 4;
      if (bitcount == 8) begin
        bitcount = 0;
        bytecount = bytecount + 1;
        spi_action;
      end
    end
  endtask

  task ddr_wr_edge;
    begin
      io0_oe = 1; io1_oe = 1; io2_oe = 1; io3_oe = 1;
      io0_dout = buffer[4]; io1_dout = buffer[5];
      io2_dout = buffer[6]; io3_dout = buffer[7];
      buffer = {buffer, 4'h0};
      bitcount = bitcount + 4;
      if (bitcount == 8) begin
        bitcount = 0;
        bytecount = bytecount + 1;
        spi_action;
      end
    end
  endtask

  always @(csb) begin
    if (csb) begin
      if (verbose) begin $display(""); $fflush; end
      buffer = 0; bitcount = 0; bytecount = 0;
      mode = mode_spi;
      io0_oe = 0; io1_oe = 0; io2_oe = 0; io3_oe = 0;
    end else if (xip_cmd) begin
      buffer = xip_cmd; bitcount = 0; bytecount = 1;
      spi_action;
    end
  end

  always @(csb, clk) begin
    spi_io_vld = 0;
    if (!csb && !clk) begin
      if (dummycount > 0) begin
        io0_oe = 0; io1_oe = 0; io2_oe = 0; io3_oe = 0;
      end else
        case (mode)
          mode_spi:      begin io0_oe=0; io1_oe=1; io2_oe=0; io3_oe=0; io1_dout=buffer[7]; end
          mode_dspi_rd:  begin io0_oe=0; io1_oe=0; io2_oe=0; io3_oe=0; end
          mode_dspi_wr:  begin io0_oe=1; io1_oe=1; io2_oe=0; io3_oe=0; io0_dout=buffer[6]; io1_dout=buffer[7]; end
          mode_qspi_rd:  begin io0_oe=0; io1_oe=0; io2_oe=0; io3_oe=0; end
          mode_qspi_wr:  begin io0_oe=1; io1_oe=1; io2_oe=1; io3_oe=1; io0_dout=buffer[4]; io1_dout=buffer[5]; io2_dout=buffer[6]; io3_dout=buffer[7]; end
          mode_qspi_ddr_rd: ddr_rd_edge;
          mode_qspi_ddr_wr: ddr_wr_edge;
        endcase
      if (next_mode) begin
        case (next_mode)
          mode_qspi_ddr_rd: begin io0_oe=0; io1_oe=0; io2_oe=0; io3_oe=0; end
          mode_qspi_ddr_wr: begin io0_oe=1; io1_oe=1; io2_oe=1; io3_oe=1; io0_dout=buffer[4]; io1_dout=buffer[5]; io2_dout=buffer[6]; io3_dout=buffer[7]; end
        endcase
        mode = next_mode;
        next_mode = 0;
      end
    end
  end

  always @(posedge clk) begin
    if (!csb) begin
      if (dummycount > 0) dummycount = dummycount - 1;
      else case (mode)
        mode_spi: begin buffer={buffer,io0}; bitcount=bitcount+1; if(bitcount==8) begin bitcount=0; bytecount=bytecount+1; spi_action; end end
        mode_dspi_rd, mode_dspi_wr: begin buffer={buffer,io1,io0}; bitcount=bitcount+2; if(bitcount==8) begin bitcount=0; bytecount=bytecount+1; spi_action; end end
        mode_qspi_rd, mode_qspi_wr: begin buffer={buffer,io3,io2,io1,io0}; bitcount=bitcount+4; if(bitcount==8) begin bitcount=0; bytecount=bytecount+1; spi_action; end end
        mode_qspi_ddr_rd: ddr_rd_edge;
        mode_qspi_ddr_wr: ddr_wr_edge;
      endcase
    end
  end
endmodule
