// #[macro_export]
// macro_rules! info {
//     ($fmt: literal $(, $($arg: tt)+)?) => {    
//         $crate::console::print(format_args!("\u{1B}[{}m{}\u{1B}[0m", 34, format_args!(concat!("[INFO] ", $fmt, "\n") $(, $($arg)+)?)));
//     }
// }

// #[macro_export]
// macro_rules! trace {
//     ($fmt: literal $(, $($arg: tt)+)?) => {    
//         $crate::console::print(format_args!("\u{1B}[{}m{}\u{1B}[0m", 90, format_args!(concat!("[TRACE] ", $fmt, "\n") $(, $($arg)+)?)));
//     }
// }

// #[macro_export]
// macro_rules! debug {
//     ($fmt: literal $(, $($arg: tt)+)?) => {    
//         $crate::console::print(format_args!("\u{1B}[{}m{}\u{1B}[0m", 32, format_args!(concat!("[DEBUG] ", $fmt, "\n") $(, $($arg)+)?)));
//     }
// }

// #[macro_export]
// macro_rules! warn {
//     ($fmt: literal $(, $($arg: tt)+)?) => {    
//         $crate::console::print(format_args!("\u{1B}[{}m{}\u{1B}[0m", 93, format_args!(concat!("[WARN] ", $fmt, "\n") $(, $($arg)+)?)));
//     }
// }

// #[macro_export]
// macro_rules! error {
//     ($fmt: literal $(, $($arg: tt)+)?) => {    
//         $crate::console::print(format_args!("\u{1B}[{}m{}\u{1B}[0m", 31, format_args!(concat!("[ERROR] ", $fmt, "\n") $(, $($arg)+)?)));
//     }
// }

// /// Add escape sequence to print with color in Linux console
// #[macro_export]
// macro_rules! with_color {
//     ($args: ident, $color_code: ident) => {{
//         format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
//     }};
// }

use log::{self, Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let max_level = log::max_level();
        metadata.level() <= max_level
    }
    fn log(&self, record: &Record) {
        let color_code = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        println!(
            "\u{1B}[{}m[{:?}] {}\u{1B}[0m",
            color_code,
            record.level(),
            // core::arch::cpu::id(),
            record.args(),
        );
    }
    fn flush(&self) {
    }
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    let env = match option_env!("LOG") {
                Some("ERROR") => LevelFilter::Error,
                Some("WARN") => LevelFilter::Warn,
                Some("INFO") => LevelFilter::Info,
                Some("DEBUG") => LevelFilter::Debug,
                Some("TRACE") => LevelFilter::Trace,
                _ => LevelFilter::Off,
            };
    let log_res = log::set_logger(&LOGGER);
    log::set_max_level(env);
    return log_res;
}