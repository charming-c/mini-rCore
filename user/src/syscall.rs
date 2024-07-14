use core::arch::asm;

// 遵循一定规范的系统调用号
const SYSTEMCALL_WRITE: usize = 64;
const SYSTEMCALL_EXIT: usize = 93;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    systemcall(SYSTEMCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    systemcall(SYSTEMCALL_EXIT, [xstate as usize, 0, 0])
}

fn systemcall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}