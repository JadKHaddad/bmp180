#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use bmp180::mode::Mode;
use bmp180::traits::AsyncBMP180;
use bmp180::BMP180;
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
        let tempreture = bmp180.read_temperature().await.unwrap();
        let tempreture = tempreture as f32 / 10.0;

        log::info!("tempreture: {} *C", tempreture);

        let pressure = bmp180.read_pressure().await.unwrap();

        log::info!("pressure: {} Pa", pressure);

        // let altidude = bmp180
        //     .read_altitude_with_sea_level_pressure(101325)
        //     .await
        //     .unwrap();
        // log::info!("altidude: {} m", altidude);

        // let pressure_at_sea_level = bmp180
        //     .read_sea_level_pressure_with_altitude_meters(0.0)
        //     .await
        //     .unwrap();
        // log::info!("pressure_at_sea_level: {} Pa", pressure_at_sea_level);

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
