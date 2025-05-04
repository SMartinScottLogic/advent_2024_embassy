// https://zenn.dev/zulinx86/articles/rust-nostd-101

// main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![cfg(target_os = "linux")]
#![feature(lang_items)]

use core::arch::asm;
use core::panic::PanicInfo;

use embassy_runner::{aoc, info};

// https://man7.org/linux/man-pages/man2/write.2.html
// ```c
// ssize_t write(int fd, const void buf[.count], size_t count);
// ```
fn sys_write(fd: i32, buf: *const u8, count: usize) -> usize {
    unsafe {
        let ret: usize;

        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
        );

        ret
    }
}

// https://man7.org/linux/man-pages/man3/exit.3.html
// ```c
// [[noreturn]] void exit(int status);
// ```
fn sys_exit(status: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") status,
            options(noreturn)
        );
    }
}

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        use core::fmt::Write;
        if self.enabled(record.metadata()) {
            unsafe {
                let mut buf: arrayvec::ArrayString<1024> = arrayvec::ArrayString::new();

                write!(buf, "{} - {}\n", record.level(), record.args());

                let ret = sys_write(1, buf.as_ptr(), buf.len());
                if ret != buf.len() {
                    sys_exit(1);
                }
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: ConsoleLogger = ConsoleLogger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info))
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    init();
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let msg = b"Hello, world!\n";
    let ret = sys_write(1, msg.as_ptr(), msg.len());
    if ret != msg.len() {
        sys_exit(1);
    }

    info!("test");
    let mut aoc = crate::aoc::Task::new();

    aoc.run();

    sys_exit(0);
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}
