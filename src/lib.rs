#![no_std]

use core::arch::global_asm;

use riscv::register::{mtvec, mtvec::TrapMode};
use riscv_rt as _;




//point mtvec to the defined vector table.
#[no_mangle]
pub unsafe extern "Rust" fn _setup_interrupts(){
    extern "C" {
        fn _vector_table();
    }
    mtvec::write(_vector_table as usize, TrapMode::Vectored)
}

//the ESP32-C3 will hardfault on the CSR writes 
//performed in the riscv-rt crate entry, so it must be overwritten.
global_asm!("
.section .init.esp, \"ax\"
.global _start_esp
_start_esp:
    lui ra, %hi(_start_esp)
    jr 32(ra)
");
global_asm!("
.section .trap, \"ax\"
.global default_start_trap

default_start_trap:
    sw t0, 1*4(sp)
    sw t1, 2*4(sp)
    sw t2, 3*4(sp)
    sw t3, 4*4(sp)
    sw t4, 5*4(sp)
    sw t5, 6*4(sp)
    sw t6, 7*4(sp)
    sw a0, 8*4(sp)
    sw a1, 9*4(sp)
    sw a2, 10*4(sp)
    sw a3, 11*4(sp)
    sw a4, 12*4(sp)
    sw a5, 13*4(sp)
    sw a6, 14*4(sp)
    sw a7, 15*4(sp)
    sw s0, 16*4(sp)
    sw s1, 17*4(sp)
    sw s2, 18*4(sp)
    sw s3, 19*4(sp)
    sw s4, 20*4(sp)
    sw s5, 21*4(sp)
    sw s6, 22*4(sp)
    sw s7, 23*4(sp)
    sw s8, 24*4(sp)
    sw s9, 25*4(sp)
    sw s10, 26*4(sp)
    sw s11, 27*4(sp)
    sw gp, 28*4(sp)
    sw tp, 29*4(sp)
    csrrs t1, mepc, x0
    sw t1, 31*4(sp)
    csrrs t1, mstatus, x0
    sw t1, 32*4(sp)
    csrrs t1, mcause, x0
    sw t1, 33*4(sp)
    csrrs t1, mtval, x0
    sw t1, 34*4(sp)
    addi s0, sp, 40*4
    sw s0, 30*4(sp)
    add a0, sp, zero

    /* move ra to callee preserved register */
    add s0, ra, zero


    jalr ra, s0



    lw t1, 31*4(sp)
    csrrw x0, mepc, t1
    lw t1, 32*4(sp)
    csrrw x0, mstatus, t1
    lw ra, 0*4(sp)
    lw t0, 1*4(sp)
    lw t1, 2*4(sp)
    lw t2, 3*4(sp)
    lw t3, 4*4(sp)
    lw t4, 5*4(sp)
    lw t5, 6*4(sp)
    lw t6, 7*4(sp)
    lw a0, 8*4(sp)
    lw a1, 9*4(sp)
    lw a2, 10*4(sp)
    lw a3, 11*4(sp)
    lw a4, 12*4(sp)
    lw a5, 13*4(sp)
    lw a6, 14*4(sp)
    lw a7, 15*4(sp)
    lw s0, 16*4(sp)
    lw s1, 17*4(sp)
    lw s2, 18*4(sp)
    lw s3, 19*4(sp)
    lw s4, 20*4(sp)
    lw s5, 21*4(sp)
    lw s6, 22*4(sp)
    lw s7, 23*4(sp)
    lw s8, 24*4(sp)
    lw s9, 25*4(sp)
    lw s10, 26*4(sp)
    lw s11, 27*4(sp)
    lw gp, 28*4(sp)
    lw tp, 29*4(sp)
    lw sp, 30*4(sp)
    # SP was restored from the original SP
    mret
");
global_asm!("
.section .trap, \"ax\"
.global _vector_table
.type _vector_table, @function

.option push
.balign 0x100
.option norelax
.option norvc

_vector_table:
    j _default_handler //ID 0 is reserved for exceptions, just use default for now to take advantage of atomic emulation
    j _handler_1
    j _handler_2
    j _handler_3
    j _handler_4
    j _handler_5
    j _handler_6
    j _handler_7
    j _handler_8
    j _handler_9
    j _handler_10
    j _handler_11
    j _handler_12
    j _handler_13
    j _handler_14
    j _handler_15
    j _handler_16
    j _handler_17
    j _handler_18
    j _handler_19
    j _handler_20
    j _handler_21
    j _handler_22
    j _handler_23
    j _handler_24
    j _handler_25
    j _handler_26
    j _handler_27
    j _handler_28
    j _handler_29
    j _handler_30
    j _handler_31

.option pop
_handler_1:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero
    la ra, cpu_int_1_handler
    jalr ra, ra
    jr s2, 0
_handler_2:
        
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_2_handler
    jalr ra, ra
    jr s2, 0
_handler_3:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_3_handler
    jalr ra, ra
    jr s2, 0
_handler_4:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_4_handler
    jalr ra, ra
    jr s2, 0
_handler_5:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_5_handler
    jalr ra, ra
    jr s2, 0
_handler_6:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_6_handler
    jalr ra, ra
    jr s2, 0
_handler_7:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_7_handler
    jalr ra, ra
    jr s2, 0
_handler_8:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_8_handler
    jalr ra, ra
    jr s2, 0
_handler_9:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_9_handler
    jalr ra, ra
    jr s2, 0
_handler_10:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_10_handler
    jalr ra, ra
    jr s2, 0
_handler_11:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_11_handler
    jalr ra, ra
    jr s2, 0
_handler_12:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_12_handler
    jalr ra, ra
    jr s2, 0
_handler_13:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_13_handler
    jalr ra, ra
    jr s2, 0
_handler_14:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_14_handler
    jalr ra, ra
    jr s2, 0
    _handler_15:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_15_handler
    jalr ra, ra
    jr s2, 0
_handler_16:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_16_handler
    jalr ra, ra
    jr s2, 0
_handler_17:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_17_handler
    jalr ra, ra
    jr s2, 0
_handler_18:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_18_handler
    jalr ra, ra
    jr s2, 0
_handler_19:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_19_handler
    jalr ra, ra
    jr s2, 0
_handler_20:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_20_handler
    jalr ra, ra
    jr s2, 0
_handler_21:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_21_handler
    jalr ra, ra
    jr s2, 0
_handler_22:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_22_handler
    jalr ra, ra
    jr s2, 0
_handler_23:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_23_handler
    jalr ra, ra
    jr s2, 0
_handler_24:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_24_handler
    jalr ra, ra
    jr s2, 0
_handler_25:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_25_handler
    jalr ra, ra
    jr s2, 0
_handler_26:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_26_handler
    jalr ra, ra
    jr s2, 0
_handler_27:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_27_handler
    jalr ra, ra
    jr s2, 0
_handler_28:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_28_handler
    jalr ra, ra
    jr s2, 0
_handler_29:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_29_handler
    jalr ra, ra
    jr s2, 0
_handler_30:
    
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_30_handler
    jalr ra, ra
    jr s2, 0
_handler_31:
    addi sp, sp, -40*4
    sw ra, 0*4(sp)
    jal ra, _start_trap
    add s2 , ra, zero 
    la ra, cpu_int_31_handler
    jalr ra, ra
    jr s2, 0
_default_handler:
    j _default_handler /* panic loop ig */
");