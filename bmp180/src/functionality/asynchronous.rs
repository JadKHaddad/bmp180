//! Asynchronous functionality.

use crate::device::{calibration::Calibration, mode::Mode};

use super::{BMP180Error, BaseBMP180};

#[allow(private_bounds)]
#[allow(async_fn_in_trait)]
pub trait AsyncBMP180<I2C, DELAY>: BaseBMP180<I2C, DELAY> {
    type Error;

    /// Read device ID.
    async fn read_id(&mut self) -> Result<u8, Self::Error>;

    /// Read calibration data.
    async fn read_calibration(&mut self) -> Result<Calibration, Self::Error>;

    /// Read raw temperature.
    async fn read_raw_temperature(&mut self) -> Result<i16, Self::Error>;

    /// Read raw pressure.
    async fn read_raw_pressure(&mut self) -> Result<i32, Self::Error>;

    /// Initialize `BMP180` instance.
    ///
    /// Initialized instance will have its calibration data set.
    async fn initialize(&mut self) -> Result<(), BMP180Error<Self::Error>> {
        let id = self.read_id().await?;

        if !Self::validate_id(id) {
            return Err(BMP180Error::InvalidId(id));
        }

        let calibration = self.read_calibration().await?;

        self.set_calibration(calibration);

        Ok(())
    }

    /// Create a new initialized `BMP180` instance.
    ///
    /// Initialized instance will have its calibration data set.
    /// See [`BaseBMP180::new`](crate::functionality::BaseBMP180) if you want to create an uninitialized instance.
    async fn initialized(
        mode: Mode,
        i2c: I2C,
        delay: DELAY,
    ) -> Result<Self, BMP180Error<Self::Error>> {
        let mut bmp180 = <Self as BaseBMP180<I2C, DELAY>>::new(mode, i2c, delay);

        bmp180.initialize().await?;

        Ok(bmp180)
    }

    /// Update temperature in `self`.
    async fn update_temperature(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = self.read_raw_temperature().await?;

        self.set_temperature(self.compute_temperature(raw_temperature));

        Ok(())
    }

    /// Update pressure in `self`.
    async fn update_pressure(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = self.read_raw_temperature().await?;
        let raw_pressure = self.read_raw_pressure().await?;

        self.set_pressure(self.compute_pressure(raw_temperature, raw_pressure));

        Ok(())
    }

    /// Update both temperature and pressure in `self`.
    async fn update(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = self.read_raw_temperature().await?;
        let raw_pressure = self.read_raw_pressure().await?;

        self.set_temperature(self.compute_temperature(raw_temperature));
        self.set_pressure(self.compute_pressure(raw_temperature, raw_pressure));

        Ok(())
    }
}
