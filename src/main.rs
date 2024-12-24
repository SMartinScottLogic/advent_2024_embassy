#![no_std]
#![no_main]

use panic_probe as _;

use assign_resources::assign_resources;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::peripherals;
use embassy_rp::usb::Driver;
use embassy_sync::{
    blocking_mutex::raw::{NoopRawMutex, ThreadModeRawMutex},
    signal::Signal,
};
use embassy_usb::{Builder, Config};
use panic_probe as _;

use advent_2024_embassy as lib;

mod server;
mod wifi;

assign_resources! {
    wifi: Wifi {
        pwr: PIN_23,
        dio: PIN_24,
        cs: PIN_25,
        clk: PIN_29,
        dma: DMA_CH0,
        pio: PIO0
    },
    usb: Usb {
        usb: USB
    },
    display: Display {
        sda: PIN_0,
        scl: PIN_1,
        i2c: I2C0
    }
}

#[embassy_executor::main]
async fn main(#[allow(unused_variables)] spawner: Spawner) {
    info!("Hello, world!");

    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);
    let wifi = r.wifi;
    let usb = r.usb.usb;
    let display = r.display;
    let driver = Driver::new(usb, lib::Irqs);

    let mut config = Config::new(0xabcd, 0xabcd);
    config.manufacturer = Some("Chris Price");
    config.product = Some("100k of your finest bytes");
    config.serial_number = Some("CP4096OYFB");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    #[cfg(feature = "wifi")]
    let mut wifi = {
        use cyw43_pio::PioSpi;
        use embassy_rp::{
            gpio::{Level, Output},
            pio::Pio,
        };

        let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
        let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

        // To make flashing faster for development, you may want to flash the firmwares independently
        // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
        //     probe-rs download 43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
        //     probe-rs download 43439A0_clm.bin --format bin --chip RP2040 --base-address 0x10140000
        //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
        //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

        let pwr = Output::new(wifi.pwr, Level::Low);
        let cs = Output::new(wifi.cs, Level::High);
        let mut pio = Pio::new(wifi.pio, lib::Irqs);
        let spi = PioSpi::new(
            &mut pio.common,
            pio.sm0,
            pio.irq0,
            cs,
            wifi.dio,
            wifi.clk,
            wifi.dma,
        );

        //let mut blinky = Blinky::build(fw, clm, pwr, spi, spawner).await;
        //let server = server::echo::Server::new();
        let server = server::aoc::Server::new();
        wifi::server::Server::build(fw, clm, pwr, spi, spawner, server).await
    };

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut mos_descriptor = [0; 0];
    let mut control_buf = [0; 64];

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut mos_descriptor,
        &mut control_buf,
    );

    let vendor_id = b"CHRISP  "; // per the spec, unused bytes should be a space
    let product_id = b"100k of trunc   ";
    let product_revision = b"1.24";

    let mut usb = builder.build();
    let usb_fut = usb.run();

    #[cfg(feature = "wifi")]
    {
        let wifi_fut = wifi.run();
        embassy_futures::join::join(usb_fut, wifi_fut).await;
    }
}
