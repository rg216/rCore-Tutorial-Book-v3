pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;
pub const MAX_SYSCALL_NUM: usize = 512;
/*#[cfg(feature = "board_qemu")]
pub const MEMORY_END: usize = 0x80a00000;*/

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

#[cfg(feature = "board_k210")]
pub const CLOCK_FREQ: usize = 403000000 / 62;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12500000;

/// the physical memory end, 128 MB
pub const MEMORY_END: usize = 0x80a00000;

// virtual memory space settings
pub const MAXVA: usize = usize::MAX;
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;

pub const BIG_STRIDE: u64 = u64::MAX;