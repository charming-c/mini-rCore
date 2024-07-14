#![allow(unused)]
use core::arch::asm;

pub fn console_putchar(c: usize) {
    // #[allow(deprecated)]
    // sbi_rt::legacy::console_putchar(c);
    sbi_call(1 as usize, c, 0, 0);
}

/// use sbi call to getchar from console (qemu uart handler)
pub fn console_getchar() -> usize {
    #[allow(deprecated)]
    sbi_rt::legacy::console_getchar()
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}

// 这里也可以不使用 sbi_rt 的接口，而是通过执行一个 ecall 指令，发起一个 trap，将特权升级为 M，cpu 执行完这里的指令后，
// 会自动跳转到应该由 sbi 设置的 trap 处理，我们根据约定好的 call num，调用 SBI 应该支持的功能。
// 这里和之后 syscall 的流程一样，只不过层次更加底层.
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        asm!("ecall",
                inlateout("x10") arg0 => ret,
                in("x11") arg1,
                in("x12") arg2,
                in("x17") which,
        )
    }
    ret
}
