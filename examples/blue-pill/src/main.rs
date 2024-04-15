#![no_main]
#![no_std]

use bmp180::blocking::UninitBMP180;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    dma::NoDma,
    i2c::{self, I2c},
    peripherals,
    time::Hertz,
};
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C2_EV => i2c::EventInterruptHandler<peripherals::I2C2>;
    I2C2_ER => i2c::ErrorInterruptHandler<peripherals::I2C2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new(
        p.I2C2,
        p.PB10, // scl
        p.PB11, // sda
        Irqs,
        NoDma,
        NoDma,
        Hertz(400),
        Default::default(),
    );

    let mut bmp180 = UninitBMP180::builder(i2c, Delay {})
        .mode(bmp180::Mode::UltraHighResolution)
        .build()
        .initialize()
        .unwrap();

    let calibration = bmp180.calibration();

    defmt::info!("calibration: {:?}", calibration);

    let mut delay = Delay {};
    loop {
        bmp180.update().ok();

        let tempreture = bmp180.temperature_celsius();
        defmt::info!("tempreture: {} *C", tempreture);

        let pressure = bmp180.pressure();
        defmt::info!("pressure: {} Pa", pressure);

        delay.delay_ms(3000)
    }
}
