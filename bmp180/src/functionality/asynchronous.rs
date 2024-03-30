//! Asynchronous functionality.

use crate::{device::calibration::Calibration, tri, BMP180};

use super::{BMP180Error, BaseBMP180, PrivateUninitBMP180};

/// Asynchronous functionality.
///
/// Bring this trait into scope to enable asynchronous functionality for BMP180 devices.
#[allow(private_bounds)]
#[allow(async_fn_in_trait)]
pub trait AsyncBMP180<I2C, DELAY>: BaseBMP180<I2C, DELAY> {
    /// Error type that can occur during asynchronous operations.
    type Error;

    /// Read raw temperature.
    async fn read_raw_temperature(&mut self) -> Result<i16, Self::Error>;

    /// Read raw pressure.
    async fn read_raw_pressure(&mut self) -> Result<i32, Self::Error>;

    /// Update temperature in `self`.
    async fn update_temperature(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = tri!(self.read_raw_temperature().await);

        self.set_temperature(self.compute_temperature(raw_temperature));

        Ok(())
    }

    /// Update pressure in `self`.
    async fn update_pressure(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = tri!(self.read_raw_temperature().await);
        let raw_pressure = tri!(self.read_raw_pressure().await);

        self.set_pressure(self.compute_pressure(raw_temperature, raw_pressure));

        Ok(())
    }

    /// Update both temperature and pressure in `self`.
    async fn update(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = tri!(self.read_raw_temperature().await);
        let raw_pressure = tri!(self.read_raw_pressure().await);

        self.set_temperature(self.compute_temperature(raw_temperature));
        self.set_pressure(self.compute_pressure(raw_temperature, raw_pressure));

        Ok(())
    }
}

/// Asynchronous functionality for uninitialized BMP180 devices
///
/// Bring this trait into scope to enable asynchronous initialization for BMP180 devices.
#[allow(private_bounds)]
#[allow(async_fn_in_trait)]
pub trait AsyncInitBMP180<I2C, DELAY>: PrivateUninitBMP180<I2C, DELAY> {
    /// Error type that can occur during initialization.
    type Error;

    /// Read device ID.
    async fn read_id(&mut self) -> Result<u8, Self::Error>;

    /// Read calibration data.
    async fn read_calibration(&mut self) -> Result<Calibration, Self::Error>;

    /// Initialize BMP180 device.
    async fn initialize(mut self) -> Result<BMP180<I2C, DELAY>, BMP180Error<Self::Error>> {
        let id = match self.read_id().await {
            Ok(id) => id,
            Err(err) => return Err(BMP180Error::I2C(err)),
        };

        if !Self::validate_id(id) {
            return Err(BMP180Error::InvalidId(id));
        }

        let calibration = match self.read_calibration().await {
            Ok(calibration) => calibration,
            Err(err) => return Err(BMP180Error::I2C(err)),
        };

        let (addr, mode, i2c, delay) = self.into_parts();

        let bmp180 = BMP180 {
            addr,
            mode,
            calibration,
            temperature: 0,
            pressure: 0,
            i2c,
            delay,
        };

        Ok(bmp180)
    }
}
