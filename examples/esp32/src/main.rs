#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use bmp180::traits::AsyncBMP180;
use bmp180::BMP180;
use bmp180::{mode::Mode, traits::BaseBMP180};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::I2c;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, embassy, entry, i2c::I2C, macros::main, peripherals::Peripherals,
    system::SystemExt, timer::TimerGroup, IO,
};
use fugit::RateExtU32;

#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut i2c0 = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        400.kHz(),
        &clocks,
    );

    embassy::init(&clocks, timg0);

    let mut bmp180 = BMP180::initialized(Mode::UltraLowPower, i2c0, embassy_time::Delay {})
        .await
        .unwrap();

    let calibration = bmp180.calibration();

    log::info!("calibration: {:?}", calibration);

    spawner.spawn(logger()).ok();

    loop {
        bmp180.update().await.ok();

        let tempreture = bmp180.temperature_celsius();
        log::info!("tempreture: {} *C", tempreture);

        let pressure = bmp180.pressure();
        log::info!("pressure: {} Pa", pressure);

        Timer::after(Duration::from_secs(3)).await;
    }
}

#[embassy_executor::task]
async fn logger() {
    loop {
        // log::info!("up");
        Timer::after(Duration::from_secs(3)).await;
    }
}
