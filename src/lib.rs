#![no_std]

use calibration::Calibration;
use consts::*;
use mode::Mode;

pub mod calibration;
mod consts;
pub mod mode;
pub mod traits;

pub struct BMP180<I2C, DELAY> {
    mode: Mode,
    calibration: Calibration,
    i2c: I2C,
    delay: DELAY,
}

impl<I2C, DELAY> BMP180<I2C, DELAY> {
    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn calibration(&self) -> &Calibration {
        &self.calibration
    }
}
impl<I2C, DELAY> traits::PrivateBaseBMP180<I2C, DELAY> for BMP180<I2C, DELAY> {
    fn set_calibration(&mut self, calibration: Calibration) {
        self.calibration = calibration;
    }
}

impl<I2C, DELAY> traits::BaseBMP180<I2C, DELAY> for BMP180<I2C, DELAY> {
    fn new(mode: Mode, i2c: I2C, delay: DELAY) -> Self {
        Self {
            mode,
            calibration: Calibration::default(),
            i2c,
            delay,
        }
    }

    fn mode(&self) -> Mode {
        self.mode()
    }

    fn calibration(&self) -> &Calibration {
        self.calibration()
    }
}

impl<I2C, DELAY> traits::AsyncBMP180<I2C, DELAY> for BMP180<I2C, DELAY>
where
    I2C: embedded_hal_async::i2c::I2c,
    DELAY: embedded_hal_async::delay::DelayNs,
{
    type Error = I2C::Error;

    async fn read_id(&mut self) -> Result<u8, Self::Error> {
        let mut data = [0u8; 2];

        self.i2c
            .write_read(BMP180_I2CADDR, &[BMP180_REGISTER_CHIPID], &mut data)
            .await?;

        Ok(data[0])
    }

    async fn read_calibration(&mut self) -> Result<Calibration, Self::Error> {
        let mut data = [0u8; 22];

        self.i2c
            .write_read(BMP180_I2CADDR, &[BMP180_CAL_AC1], &mut data)
            .await?;

        Ok(Calibration::from_slice(&data))
    }

    async fn read_raw_temperature(&mut self) -> Result<u16, Self::Error> {
        self.i2c
            .write(BMP180_I2CADDR, &[BMP180_CONTROL, BMP180_READTEMPCMD])
            .await?;

        self.delay.delay_ns(4500).await;

        let mut data = [0u8; 2];

        self.i2c
            .write_read(BMP180_I2CADDR, &[BMP180_TEMPDATA], &mut data)
            .await?;

        let raw_temperature = data[1] as u16 | ((data[0] as u16) << 8);

        Ok(raw_temperature)
    }

    async fn read_raw_pressure(&mut self) -> Result<u32, Self::Error> {
        let mode = self.mode();

        self.i2c
            .write(
                BMP180_I2CADDR,
                &[BMP180_CONTROL, BMP180_READPRESSURECMD + (mode as u8) << 6],
            )
            .await?;

        self.delay.delay_ms(mode.delay_ms()).await;

        let mut data = [0u8; 2];

        self.i2c
            .write_read(BMP180_I2CADDR, &[BMP180_PRESSUREDATA], &mut data)
            .await?;

        let raw_pressure = data[1] as u32 | ((data[0] as u32) << 8);
        let raw_pressure = raw_pressure << 8;

        let mut data = [0u8; 1];

        self.i2c
            .write_read(BMP180_I2CADDR, &[BMP180_PRESSUREDATA + 2], &mut data)
            .await?;

        let raw_pressure = raw_pressure | data[0] as u32;
        let raw_pressure = raw_pressure >> (8 - mode as u32);

        Ok(raw_pressure)
    }
}
