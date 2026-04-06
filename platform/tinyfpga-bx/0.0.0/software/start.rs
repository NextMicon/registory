// PicoRV32 startup and IRQ handler

use core::arch::global_asm;

global_asm!(r#"
.section .data

.global irq_regs
irq_regs:
    .fill 32, 4, 0
    .fill 128, 4, 0
irq_stack:

.section .text
.global _start

// ===== Reset vector (PROGADDR_RESET = 0x00050000) =====
_start:
    j start

// ===== IRQ vector (PROGADDR_IRQ = 0x00050010, 16-byte aligned) =====
.balign 16
irq_vec:
    // setq q2, x1  (save ra)
    .word 0x0200a10b
    // setq q3, x2  (save sp)
    .word 0x0201218b

    // Load irq_regs base into x1
    lui  x1, %hi(irq_regs)
    addi x1, x1, %lo(irq_regs)

    // getq x6, q0  (get return address)
    .word 0x0000430b
    sw   x6, 0*4(x1)

    // getq x6, q2  (get original x1)
    .word 0x0001430b
    sw   x6, 1*4(x1)

    // getq x6, q3  (get original x2)
    .word 0x0001c30b
    sw   x6, 2*4(x1)

    sw x3,   3*4(x1)
    sw x4,   4*4(x1)
    sw x5,   5*4(x1)
    sw x6,   6*4(x1)
    sw x7,   7*4(x1)
    sw x8,   8*4(x1)
    sw x9,   9*4(x1)
    sw x10, 10*4(x1)
    sw x11, 11*4(x1)
    sw x12, 12*4(x1)
    sw x13, 13*4(x1)
    sw x14, 14*4(x1)
    sw x15, 15*4(x1)
    sw x16, 16*4(x1)
    sw x17, 17*4(x1)
    sw x18, 18*4(x1)
    sw x19, 19*4(x1)
    sw x20, 20*4(x1)
    sw x21, 21*4(x1)
    sw x22, 22*4(x1)
    sw x23, 23*4(x1)
    sw x24, 24*4(x1)
    sw x25, 25*4(x1)
    sw x26, 26*4(x1)
    sw x27, 27*4(x1)
    sw x28, 28*4(x1)
    sw x29, 29*4(x1)
    sw x30, 30*4(x1)
    sw x31, 31*4(x1)

    // Call irq(regs, irq_bits)
    lui  sp, %hi(irq_stack)
    addi sp, sp, %lo(irq_stack)
    lui  a0, %hi(irq_regs)
    addi a0, a0, %lo(irq_regs)
    // getq a1, q1  (irq bits)
    .word 0x0000c58b
    jal  ra, irq

    // Restore registers (a0 = new irq_regs pointer)
    mv   x1, a0

    lw   x6, 0*4(x1)
    // setq q0, x6  (set return address)
    .word 0x0203200b

    lw   x6, 1*4(x1)
    // setq q2, x6  (restore x1 later)
    .word 0x0203210b

    lw   x6, 2*4(x1)
    // setq q3, x6  (restore x2 later)
    .word 0x0203218b

    lw x3,   3*4(x1)
    lw x4,   4*4(x1)
    lw x5,   5*4(x1)
    lw x6,   6*4(x1)
    lw x7,   7*4(x1)
    lw x8,   8*4(x1)
    lw x9,   9*4(x1)
    lw x10, 10*4(x1)
    lw x11, 11*4(x1)
    lw x12, 12*4(x1)
    lw x13, 13*4(x1)
    lw x14, 14*4(x1)
    lw x15, 15*4(x1)
    lw x16, 16*4(x1)
    lw x17, 17*4(x1)
    lw x18, 18*4(x1)
    lw x19, 19*4(x1)
    lw x20, 20*4(x1)
    lw x21, 21*4(x1)
    lw x22, 22*4(x1)
    lw x23, 23*4(x1)
    lw x24, 24*4(x1)
    lw x25, 25*4(x1)
    lw x26, 26*4(x1)
    lw x27, 27*4(x1)
    lw x28, 28*4(x1)
    lw x29, 29*4(x1)
    lw x30, 30*4(x1)
    lw x31, 31*4(x1)

    // getq x1, q2  (restore original x1/ra)
    .word 0x0001408b
    // getq x2, q3  (restore original x2/sp)
    .word 0x0001c10b

    // retirq
    .word 0x0400000b

// ===== Main entry =====
start:
    // Zero-initialize all registers
    addi x1, zero, 0
    addi x3, zero, 0
    addi x4, zero, 0
    addi x5, zero, 0
    addi x6, zero, 0
    addi x7, zero, 0
    addi x8, zero, 0
    addi x9, zero, 0
    addi x10, zero, 0
    addi x11, zero, 0
    addi x12, zero, 0
    addi x13, zero, 0
    addi x14, zero, 0
    addi x15, zero, 0
    addi x16, zero, 0
    addi x17, zero, 0
    addi x18, zero, 0
    addi x19, zero, 0
    addi x20, zero, 0
    addi x21, zero, 0
    addi x22, zero, 0
    addi x23, zero, 0
    addi x24, zero, 0
    addi x25, zero, 0
    addi x26, zero, 0
    addi x27, zero, 0
    addi x28, zero, 0
    addi x29, zero, 0
    addi x30, zero, 0
    addi x31, zero, 0

    call main

loop:
    j loop
"#);
