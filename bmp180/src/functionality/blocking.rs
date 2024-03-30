//! Blocking functionality.

use crate::{device::calibration::Calibration, tri, BMP180};

use super::{BMP180Error, BaseBMP180, PrivateUninitBMP180};

/// Blocking functionality.
///
/// Bring this trait into scope to enable blocking functionality for BMP180 devices.
#[allow(private_bounds)]
pub trait BlockingBMP180<I2C, DELAY>: BaseBMP180<I2C, DELAY> {
    /// Error type that can occur during blocking operations.
    type Error;

    /// Read device ID.
    fn read_id(&mut self) -> Result<u8, Self::Error>;

    /// Read calibration data.
    fn read_calibration(&mut self) -> Result<Calibration, Self::Error>;

    /// Read raw temperature.
    fn read_raw_temperature(&mut self) -> Result<i16, Self::Error>;

    /// Read raw pressure.
    fn read_raw_pressure(&mut self) -> Result<i32, Self::Error>;

    /// Update temperature in `self`.
    fn update_temperature(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = tri!(self.read_raw_temperature());

        self.set_temperature(self.compute_temperature(raw_temperature));

        Ok(())
    }

    /// Update pressure in `self`.
    fn update_pressure(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = tri!(self.read_raw_temperature());
        let raw_pressure = tri!(self.read_raw_pressure());

        self.set_pressure(self.compute_pressure(raw_temperature, raw_pressure));

        Ok(())
    }

    /// Update both temperature and pressure in `self`.
    fn update(&mut self) -> Result<(), Self::Error> {
        let raw_temperature = tri!(self.read_raw_temperature());
        let raw_pressure = tri!(self.read_raw_pressure());

        self.set_temperature(self.compute_temperature(raw_temperature));
        self.set_pressure(self.compute_pressure(raw_temperature, raw_pressure));

        Ok(())
    }
}

/// Blocking functionality for uninitialized BMP180 devices
///
/// Bring this trait into scope to enable blocking initialization for BMP180 devices.
#[allow(private_bounds)]
pub trait BlockingInitBMP180<I2C, DELAY>: PrivateUninitBMP180<I2C, DELAY> {
    /// Error type that can occur during initialization.
    type Error;

    /// Read device ID.
    fn read_id(&mut self) -> Result<u8, Self::Error>;

    /// Read calibration data.
    fn read_calibration(&mut self) -> Result<Calibration, Self::Error>;

    /// Initialize BMP180 device.
    fn initialize(mut self) -> Result<BMP180<I2C, DELAY>, BMP180Error<Self::Error>> {
        let id = match self.read_id() {
            Ok(id) => id,
            Err(err) => return Err(BMP180Error::I2C(err)),
        };

        if !Self::validate_id(id) {
            return Err(BMP180Error::InvalidId(id));
        }

        let calibration = match self.read_calibration() {
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
