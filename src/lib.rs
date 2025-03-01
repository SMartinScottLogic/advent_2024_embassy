#![no_std]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_rp::{
    bind_interrupts,
    peripherals::{PIO0, USB},
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
bind_interrupts!(pub struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
});

pub mod aoc;

#[macro_use]
pub mod fmt;
