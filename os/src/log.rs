use crate::sbi::sbi_sleep;

pub fn sleep(sec: u64) {
    sbi_sleep(sec);
}