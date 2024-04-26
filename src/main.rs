#![no_std] // No standard library. We can't use this.
#![no_main] // We do have a main, but not in the standard Rust way.

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::arch::global_asm;
use core::panic::PanicInfo;
use risc0_zkvm_platform::syscall::sys_alloc_aligned;
global_asm!(include_str!("asm/init.s"));

mod env;
mod serde;
mod utils;

#[cfg(not(target_os = "none"))]
compile_error!("This crate only runs on bare metal");

#[no_mangle]
pub extern "C" fn runcontract(_: u32) -> u32 {
    const num: u32 = 901;

    let a: u32 = env::read();

    let res = if a == num || a == 1 {
        0
    } else {
        let rem = num % a;
        match rem {
            0 => 1,
            _ => 0,
        }
    };

    env::write(&res);
    return 0;
}

// Unlike C, Rust panics sometimes.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct BumpPointerAlloc;

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        sys_alloc_aligned(layout.size(), layout.align())
        //sys_alloc_aligned(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc;
