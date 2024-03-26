//! Device definition and implementation.

use crate::functionality::{BaseBMP180, PrivateBaseBMP180};

use self::{calibration::Calibration, mode::Mode};

pub mod calibration;
pub mod mode;

#[derive(Debug, Clone)]
pub struct BMP180<I2C, DELAY> {
    mode: Mode,
    calibration: Calibration,
    temperature: i32,
    pressure: i32,
    i2c: I2C,
    delay: DELAY,
}

impl<I2C, DELAY> BMP180<I2C, DELAY> {
    fn mode(&self) -> Mode {
        self.mode
    }

    fn calibration(&self) -> &Calibration {
        &self.calibration
    }
}

impl<I2C, DELAY> PrivateBaseBMP180<I2C, DELAY> for BMP180<I2C, DELAY> {
    fn set_temperature(&mut self, temperature: i32) {
        self.temperature = temperature;
    }

    fn set_pressure(&mut self, pressure: i32) {
        self.pressure = pressure;
    }

    fn set_calibration(&mut self, calibration: Calibration) {
        self.calibration = calibration;
    }
}

impl<I2C, DELAY> BaseBMP180<I2C, DELAY> for BMP180<I2C, DELAY> {
    fn new(mode: Mode, i2c: I2C, delay: DELAY) -> Self {
        Self {
            mode,
            calibration: Calibration::default(),
            temperature: 0,
            pressure: 0,
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

    fn temperature(&self) -> i32 {
        self.temperature
    }

    fn pressure(&self) -> i32 {
        self.pressure
    }
}

#[cfg(feature = "async")]
mod impl_async {
    use embedded_hal_async::{delay::DelayNs, i2c::I2c};

    use crate::{constants::*, functionality::asynchronous::AsyncBMP180};

    use super::{calibration::Calibration, BMP180};

    impl<I2C, DELAY> AsyncBMP180<I2C, DELAY> for BMP180<I2C, DELAY>
    where
        I2C: I2c,
        DELAY: DelayNs,
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

        async fn read_raw_temperature(&mut self) -> Result<i16, Self::Error> {
            self.i2c
                .write(BMP180_I2CADDR, &[BMP180_CONTROL, BMP180_READTEMPCMD])
                .await?;

            self.delay.delay_ms(5).await;

            let mut data = [0u8; 2];

            self.i2c
                .write_read(BMP180_I2CADDR, &[BMP180_TEMPDATA], &mut data)
                .await?;

            let raw_temperature = ((data[0] as i16) << 8) | data[1] as i16;

            Ok(raw_temperature)
        }

        async fn read_raw_pressure(&mut self) -> Result<i32, Self::Error> {
            let mode = self.mode();

            self.i2c
                .write(
                    BMP180_I2CADDR,
                    &[BMP180_CONTROL, BMP180_READPRESSURECMD + ((mode as u8) << 6)],
                )
                .await?;

            self.delay.delay_ms(mode.delay_ms()).await;

            let mut data = [0u8; 3];

            self.i2c
                .write_read(BMP180_I2CADDR, &[BMP180_PRESSUREDATA], &mut data)
                .await?;

            let raw_pressure =
                (((data[0] as i32) << 16) + ((data[1] as i32) << 8) + data[2] as i32)
                    >> (8 - mode as u8);

            Ok(raw_pressure)
        }
    }
}

#[cfg(feature = "blocking")]
mod impl_blocking {
    use embedded_hal::{delay::DelayNs, i2c::I2c};

    use crate::{constants::*, functionality::blocking::BlockingBMP180};

    use super::{calibration::Calibration, BMP180};

    impl<I2C, DELAY> BlockingBMP180<I2C, DELAY> for BMP180<I2C, DELAY>
    where
        I2C: I2c,
        DELAY: DelayNs,
    {
        type Error = I2C::Error;

        fn read_id(&mut self) -> Result<u8, Self::Error> {
            let mut data = [0u8; 2];

            self.i2c
                .write_read(BMP180_I2CADDR, &[BMP180_REGISTER_CHIPID], &mut data)?;

            Ok(data[0])
        }

        fn read_calibration(&mut self) -> Result<Calibration, Self::Error> {
            let mut data = [0u8; 22];

            self.i2c
                .write_read(BMP180_I2CADDR, &[BMP180_CAL_AC1], &mut data)?;

            Ok(Calibration::from_slice(&data))
        }

        fn read_raw_temperature(&mut self) -> Result<i16, Self::Error> {
            self.i2c
                .write(BMP180_I2CADDR, &[BMP180_CONTROL, BMP180_READTEMPCMD])?;

            self.delay.delay_ms(5);

            let mut data = [0u8; 2];

            self.i2c
                .write_read(BMP180_I2CADDR, &[BMP180_TEMPDATA], &mut data)?;

            let raw_temperature = ((data[0] as i16) << 8) | data[1] as i16;

            Ok(raw_temperature)
        }

        fn read_raw_pressure(&mut self) -> Result<i32, Self::Error> {
            let mode = self.mode();

            self.i2c.write(
                BMP180_I2CADDR,
                &[BMP180_CONTROL, BMP180_READPRESSURECMD + ((mode as u8) << 6)],
            )?;

            self.delay.delay_ms(mode.delay_ms());

            let mut data = [0u8; 3];

            self.i2c
                .write_read(BMP180_I2CADDR, &[BMP180_PRESSUREDATA], &mut data)?;

            let raw_pressure =
                (((data[0] as i32) << 16) + ((data[1] as i32) << 8) + data[2] as i32)
                    >> (8 - mode as u8);

            Ok(raw_pressure)
        }
    }
}
