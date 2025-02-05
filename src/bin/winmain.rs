// main.rs
#![no_main]
#![no_std]
#![cfg(target_os = "windows")]
#![windows_subsystem = "console"]

use core::ffi::c_void;
use core::panic::PanicInfo;

use windows_sys::Win32::System::Console::GetStdHandle;
use windows_sys::Win32::System::Console::WriteConsoleA;
use windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE;
use windows_sys::Win32::System::Threading::ExitProcess;

// used when `windows_subsystem = "windows"`
//use windows_sys::Win32::System::Console::AttachConsole;
//use windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS;

use embassy_runner::aoc;

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    unsafe {
        ExitProcess(1);
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
                // get a handle to the console output buffer
                let console = GetStdHandle(STD_OUTPUT_HANDLE);

                let mut system_time: windows_sys::Win32::Foundation::SYSTEMTIME =
                    core::mem::zeroed();
                windows_sys::Win32::System::SystemInformation::GetSystemTime(
                    &mut system_time as *mut windows_sys::Win32::Foundation::SYSTEMTIME,
                );
                let mut buf: arrayvec::ArrayString<1024> = arrayvec::ArrayString::new();
                write!(
                    buf,
                    "{:04}{:02}{:02} {:02}:{:02}:{:02}.{:02} ",
                    system_time.wYear,
                    system_time.wMonth,
                    system_time.wDay,
                    system_time.wHour,
                    system_time.wMinute,
                    system_time.wSecond,
                    system_time.wMilliseconds
                );

                write!(buf, "{} - {}\n", record.level(), record.args());
                // write the message to the console buffer
                WriteConsoleA(
                    console,
                    buf.as_ptr().cast::<c_void>(),
                    buf.len() as u32,
                    core::ptr::null_mut(),
                    core::ptr::null(),
                );
            }
        }
    }

    fn flush(&self) {}
}
static LOGGER: ConsoleLogger = ConsoleLogger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info))
}
#[link(name = "vcruntime")]
extern "C" {}

#[link(name = "ucrt")]
extern "C" {}

#[no_mangle]
#[allow(non_snake_case)]
fn mainCRTStartup() -> ! {
    init();

    let message = "hello world\n";
    unsafe {
        // need this when `windows_subsystem = "windows"`
        // AttachConsole(ATTACH_PARENT_PROCESS);

        // get a handle to the console output buffer
        let console = GetStdHandle(STD_OUTPUT_HANDLE);

        // write the message to the console buffer
        // alternatively, `WriteFile` can be used in this case too, need additional feature flags for `windows-sys` crate
        WriteConsoleA(
            console,
            message.as_ptr().cast::<c_void>(),
            message.len() as u32,
            core::ptr::null_mut(),
            core::ptr::null(),
        );

        let mut aoc = crate::aoc::Task::new();

        aoc.run();

        ExitProcess(0)
    }
}
