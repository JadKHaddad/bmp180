#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use bmp180_embedded_hal::{asynch::UninitBMP180, Mode};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use embassy_time::{Delay, Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy, entry,
    i2c::I2C,
    macros::main,
    peripherals::{Peripherals, I2C0},
    system::SystemExt,
    timer::TimerGroup,
    IO,
};
use fugit::RateExtU32;
use static_cell::StaticCell;

static I2C_BUS: StaticCell<Mutex<NoopRawMutex, I2C<'_, I2C0>>> = StaticCell::new();

#[main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c0 = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21, // SDA
        io.pins.gpio22, // SCL
        400.kHz(),
        &clocks,
    );

    let i2c_bus = Mutex::new(i2c0);
    let i2c_bus = I2C_BUS.init(i2c_bus);
    let i2c_dev1 = I2cDevice::new(i2c_bus);

    embassy::init(&clocks, timg0);

    let mut bmp180 = UninitBMP180::builder(i2c_dev1, Delay {})
        .mode(Mode::UltraHighResolution)
        .build()
        .initialize()
        .await
        .unwrap();

    // let mut bmp180 = UninitBMP180::new(
    //     bmp180::Address::Default,
    //     Mode::UltraLowPower,
    //     i2c_dev1,
    //     embassy_time::Delay {},
    // )
    // .initialize()
    // .await
    // .unwrap();

    let calibration = bmp180.calibration();

    log::info!("calibration: {:?}", calibration);

    loop {
        bmp180.update().await.ok();

        let tempreture = bmp180.temperature_celsius();
        log::info!("tempreture: {} *C", tempreture);

        let pressure = bmp180.pressure();
        log::info!("pressure: {} Pa", pressure);

        Timer::after(Duration::from_secs(3)).await;
    }
}
