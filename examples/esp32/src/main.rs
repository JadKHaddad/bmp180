#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

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
        io.pins.gpio4,
        io.pins.gpio5,
        400.kHz(),
        &clocks,
    );

    embassy::init(&clocks, timg0);

    spawner.spawn(logger()).ok();

    loop {
        let mut data = [0u8; 22];
        i2c0.write_read(0x77, &[0xaa], &mut data).await.ok();

        log::info!("{:02x?}", data);

        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn logger() {
    loop {
        log::info!("up");
        Timer::after(Duration::from_secs(3)).await;
    }
}
