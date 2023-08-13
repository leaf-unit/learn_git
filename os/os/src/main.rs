#![no_std]
#![no_main]
#![allow(unused)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_item;
mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}