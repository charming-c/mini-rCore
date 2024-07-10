// os/src/main.rs
#![no_std]
#![no_main]

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod log;

use crate::log::sleep;

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("hello world!!!");
    sleep(5000);
    panic!("oh no");
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