use core::arch::asm;

pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
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

pub fn sbi_sleep(mills_second: u64) {
    // Qemu 的mtime寄存器的时钟频率为10MHz，每次增加0.1微妙（100纳秒）1比10_000_000
    // 入参数为毫秒为1:10_000
    let nano_duration = mills_second
        .checked_mul(10_000)
        .expect("Overflow in time calculation");
    let start_time = unsafe { get_current_time() };
    // 计算唤醒时间，同时注意处理可能的溢出
    let wake_time = start_time
        .checked_add(nano_duration)
        .expect("Overflow in wake time calculation");
    // 使用SBI调用设置唤醒时间
    #[allow(deprecated)]
    sbi_rt::legacy::set_timer(wake_time);
    println!("Wake time = {}", wake_time);
    loop {
        if unsafe { get_current_time() } > wake_time {
            break;
        }

        // unsafe { asm!("wfi") }
    }
}
const MTIME_ADDR: *const u64 = 0x200BFF8 as *const u64; // 假定的mtime地址，根据你的具体硬件来更改

unsafe fn get_current_time() -> u64 {
    // println!("now time = {}", *MTIME_ADDR);
    return *MTIME_ADDR;
}
