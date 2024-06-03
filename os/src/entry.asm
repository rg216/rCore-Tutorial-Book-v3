    .attribute arch, "rv64gc" 
    .section .text.entry
    .globl _start

_start:
    la sp, boot_stack_top
    /* Initialize FPU if present */
    //fscsr x0
    #la t0, 31
    #csrw mtvec, t0
    call rust_main
    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: