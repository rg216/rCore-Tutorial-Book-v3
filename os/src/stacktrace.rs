use core::{arch::asm, ptr};

pub unsafe fn print_stack_trace() -> (){
	let mut fp: *const usize;
	asm!("mv {}, fp", out(reg) fp);
	println!("------------- Stack Trace -------------");
	while fp != ptr::null(){
		let ra = *fp.sub(1);
		let sepc = *fp.sub(2);
		println!("sepc: {:016x}, ra: {:016x}", sepc, ra);
		fp = sepc as *const usize;
	}
	println!("----------- Stack Trace End -----------");
}