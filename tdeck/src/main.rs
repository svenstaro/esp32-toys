use std::time::Duration;

use anyhow::Result;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriverConfig, SpiError};
use esp_idf_sys::EspError;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use esp_idf_sys as _;
use log::*;
use mipidsi::{Builder, Orientation};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("SPI")]
    Spi(#[from] SpiError),
    #[error("ESP")]
    Esp(#[from] EspError),
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello");
    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;
    let mut delay = Ets;

    let rst = PinDriver::output(peripherals.pins.gpio17)?;
    let dc = PinDriver::output(peripherals.pins.gpio11)?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio42)?;
    let sclk = peripherals.pins.gpio40;
    let sdo = peripherals.pins.gpio41;
    let sdi = peripherals.pins.gpio38;
    let cs = peripherals.pins.gpio12;

    // configuring the spi interface, note that in order for the ST7789 to work, the data_mode needs to be set to MODE_3
    let config = esp_idf_hal::spi::config::Config::new()
        .baudrate(26.MHz().into())
        .data_mode(esp_idf_hal::spi::config::MODE_3);

    let device = SpiDeviceDriver::new_single(
        spi,
        sclk,
        sdo,
        Some(sdi),
        Some(cs),
        &SpiDriverConfig::new(),
        &config,
    )?;

    // display interface abstraction from SPI and DC
    let di = SPIInterfaceNoCS::new(device, dc);

    // create driver
    let mut display = Builder::st7789(di)
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .with_display_size(320, 240)
        // set default orientation
        .with_orientation(Orientation::Portrait(false))
        // initialize
        .init(&mut delay, Some(rst))
        .unwrap();

    // turn on the backlight
    backlight.set_high()?;

    // let raw_image_data = ImageRawLE::new(include_bytes!("../examples/assets/ferris.raw"), 86);
    // let ferris = Image::new(&raw_image_data, Point::new(0, 0));

    // draw image on black background
    // ferris.draw(&mut display).unwrap();

    println!("Image printed!");

    // let g = colorgrad::rainbow();

    loop {
        // Line::new(Point::new(0, 0), Point::new(320, 240))
        //     .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
        //     .draw(&mut display)
        //     .unwrap();
        println!("Looping");
        display.clear(Rgb565::RED).unwrap();
        std::thread::sleep(Duration::from_millis(500));
        display.clear(Rgb565::BLACK).unwrap();
        std::thread::sleep(Duration::from_millis(500));
    }
    // std::thread::spawn(move || loop {
    // for y in 0..240 {
    //     let color_at = g.at((y / 240) as f64);
    //     let rgb = Rgb565::new(color_at.r as u8, color_at.g as u8, color_at.b as u8);
    //     Line::new(Point::new(0, y), Point::new(320, y))
    //         .into_styled(PrimitiveStyle::with_stroke(rgb, 1))
    //         .draw(&mut display)
    //         .unwrap();
    // }
    // })
    // .join()
    // .unwrap();

    // Ok(())
}