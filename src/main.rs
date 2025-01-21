#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg(all(target_arch = "arm", target_os = "none"))]
//extern crate alloc;
extern crate core;

use panic_probe as _;

use assign_resources::assign_resources;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::peripherals;
use embassy_rp::usb::Driver;
use embassy_usb::{Builder, Config};
use panic_probe as _;

use embassy_runner as lib;

mod aoc;

assign_resources! {
    usb: Usb {
        usb: USB
    },
}

#[embassy_executor::main]
async fn main(#[allow(unused_variables)] spawner: Spawner) {
    info!("Hello, world!");

    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);
    let usb = r.usb.usb;
    let driver = Driver::new(usb, lib::Irqs);

    let mut config = Config::new(0xabcd, 0xabcd);
    config.manufacturer = Some("Chris Price");
    config.product = Some("100k of your finest bytes");
    config.serial_number = Some("CP4096OYFB");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut mos_descriptor = [0; 0];
    let mut control_buf = [0; 64];

    let builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut mos_descriptor,
        &mut control_buf,
    );

    let _vendor_id = b"CHRISP  "; // per the spec, unused bytes should be a space
    let _product_id = b"100k of trunc   ";
    let _product_revision = b"1.24";

    let mut usb = builder.build();
    let usb_fut = usb.run();

    let mut aoc = aoc::Task::new();

    let aoc_fut = aoc.run();
    embassy_futures::join::join(usb_fut, aoc_fut).await;
}
