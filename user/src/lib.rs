#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;


#[no_mangle]
#[link_section = ".text.entry"]     // 将本函数编译后的汇编放到 .text.entry 代码段中
pub extern "C" fn _start() -> ! {
    clear_bss();    // 手动清空.bss 代码段
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]     // 弱链接，在 lib 和 bin 目录下都有 main 符号时，优先选择 bin 目录的 main 作为 main
#[no_mangle]
fn main() -> i32 {
    panic!("Cannnot find main!");
}

use syscall::*;

pub fn write(fd: usize, buffer: &[u8]) -> isize { sys_write(fd, buffer) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }


pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
