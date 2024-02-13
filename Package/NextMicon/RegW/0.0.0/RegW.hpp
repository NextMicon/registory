#pragma once
#include <stdint.h>

class Reg {
  volatile uint32_t* reg;
public:
  Reg(volatile uint32_t* addr) : reg(addr) {}

  // Read value
  uint32_t read() { return reg[0]; }

  // Write value
  void write(uint32_t val) { reg[0] = val; }
};
