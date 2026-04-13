#include <verilated.h>
#include <verilated_vcd_c.h>
#include "Vtb.h"

int main(int argc, char** argv) {
    Verilated::commandArgs(argc, argv);
    Verilated::traceEverOn(true);

    Vtb* dut = new Vtb;
    VerilatedVcdC* tfp = new VerilatedVcdC;
    dut->trace(tfp, 99);
    tfp->open("simulation/out.vcd");

    // 16 MHz clock: half period = 31.25 ns = 31250 ps
    const uint64_t HALF_PERIOD = 31250;
    const uint64_t MAX_CYCLES = 1000000;

    uint64_t sim_time = 0;

    for (uint64_t cycle = 0; cycle < MAX_CYCLES; cycle++) {
        dut->clk = 0;
        dut->eval();
        tfp->dump(sim_time);
        sim_time += HALF_PERIOD;

        dut->clk = 1;
        dut->eval();
        tfp->dump(sim_time);
        sim_time += HALF_PERIOD;
    }

    tfp->close();
    delete dut;
    return 0;
}
