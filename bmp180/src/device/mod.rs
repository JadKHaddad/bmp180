//! Device definition and implementation.

use crate::{
    functionality::{BaseBMP180, PrivateBaseBMP180, PrivateUninitBMP180},
    Address,
};

use self::{calibration::Calibration, mode::Mode};

pub mod address;
pub mod calibration;
pub(crate) mod id;
pub mod mode;
pub(crate) mod register;

/// Builder for an uninitialized BMP180 device.
///
/// Helpful for using default values.
#[derive(Debug, Clone)]
pub struct UninitBMP180Builder<I2C, DELAY> {
    inner: UninitBMP180<I2C, DELAY>,
}

impl<I2C, DELAY> UninitBMP180Builder<I2C, DELAY> {
    /// Create a new builder.
    pub fn new(i2c: I2C, delay: DELAY) -> Self {
        Self {
            inner: UninitBMP180::new(Address::default(), Mode::default(), i2c, delay),
        }
    }

    /// Set the device address.
    pub fn addr(mut self, addr: Address) -> Self {
        self.inner.addr = addr;
        self
    }

    /// Set the device mode.
    pub fn mode(mut self, mode: Mode) -> Self {
        self.inner.mode = mode;
        self
    }

    /// Build the BMP180 device.
    pub fn build(self) -> UninitBMP180<I2C, DELAY> {
        self.inner
    }
}

/// Uninitialized BMP180 device.
///
/// This struct is used to initialize the BMP180 device using:
/// - [`AsyncInitBMP180`](crate::functionality::asynchronous::AsyncInitBMP180)
/// - [`BlockingInitBMP180`](crate::functionality::blocking::BlockingInitBMP180)
#[derive(Debug, Clone)]
pub struct UninitBMP180<I2C, DELAY> {
    /// Device I2C address.
    pub addr: Address,
    /// Device mode.
    pub mode: Mode,
    /// Device I2C bus.
    pub i2c: I2C,
    /// Delay provider.
    pub delay: DELAY,
}

impl<I2C, DELAY> UninitBMP180<I2C, DELAY> {
    /// Create a new uninitialized BMP180 device.
    pub fn new(addr: Address, mode: Mode, i2c: I2C, delay: DELAY) -> Self {
        Self {
            addr,
            mode,
            i2c,
            delay,
        }
    }

    /// Create a new builder.
    pub fn builder(i2c: I2C, delay: DELAY) -> UninitBMP180Builder<I2C, DELAY> {
        UninitBMP180Builder::new(i2c, delay)
    }

    /// Split the BMP180 device into its parts.
    pub fn into_parts(self) -> (Address, Mode, I2C, DELAY) {
        (self.addr, self.mode, self.i2c, self.delay)
    }
}

impl<I2C, DELAY> PrivateUninitBMP180<I2C, DELAY> for UninitBMP180<I2C, DELAY> {
    fn addr(&self) -> Address {
        self.addr
    }

    fn into_parts(self) -> (Address, Mode, I2C, DELAY) {
        self.into_parts()
    }
}

/// BMP180 device.
///
/// Represents an initialized BMP180 device valid id and its calibration data set.
/// Initialized using:
/// - [`AsyncInitBMP180`](crate::functionality::asynchronous::AsyncInitBMP180)
/// - [`BlockingInitBMP180`](crate::functionality::blocking::BlockingInitBMP180)
#[derive(Debug, Clone)]
pub struct BMP180<I2C, DELAY> {
    pub(crate) addr: Address,
    pub(crate) mode: Mode,
    pub(crate) calibration: Calibration,
    pub(crate) temperature: i32,
    pub(crate) pressure: i32,
    pub(crate) i2c: I2C,
    pub(crate) delay: DELAY,
}

#[cfg(feature = "i-know-what-i-am-doing")]
impl<I2C, DELAY> BMP180<I2C, DELAY> {
    /// Split the BMP180 device into its parts.
    ///
    /// Only available when the `i-know-what-i-am-doing` feature is enabled.
    pub fn into_parts(self) -> (u8, Mode, Calibration, i32, i32, I2C, DELAY) {
        (
            self.addr,
            self.mode,
            self.calibration,
            self.temperature,
            self.pressure,
            self.i2c,
            self.delay,
        )
    }

    /// Create a BMP180 device from its parts.
    ///
    /// Only available when the `i-know-what-i-am-doing` feature is enabled.
    pub fn from_parts(
        addr: u8,
        mode: Mode,
        calibration: Calibration,
        temperature: i32,
        pressure: i32,
        i2c: I2C,
        delay: DELAY,
    ) -> Self {
        Self {
            addr,
            mode,
            calibration,
            temperature,
            pressure,
            i2c,
            delay,
        }
    }
}

impl<I2C, DELAY> PrivateBaseBMP180<I2C, DELAY> for BMP180<I2C, DELAY> {
    fn set_temperature(&mut self, temperature: i32) {
        self.temperature = temperature;
    }

    fn set_pressure(&mut self, pressure: i32) {
        self.pressure = pressure;
    }
}

impl<I2C, DELAY> BaseBMP180<I2C, DELAY> for BMP180<I2C, DELAY> {
    fn addr(&self) -> Address {
        self.addr
    }

    fn mode(&self) -> Mode {
        self.mode
    }

    fn calibration(&self) -> &Calibration {
        &self.calibration
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

    use crate::{
        device::register::Register,
        functionality::{
            asynchronous::{AsyncBMP180, AsyncInitBMP180},
            PrivateUninitBMP180,
        },
        tri, BaseBMP180,
    };

    use super::{calibration::Calibration, UninitBMP180, BMP180};

    impl<I2C, DELAY> AsyncInitBMP180<I2C, DELAY> for UninitBMP180<I2C, DELAY>
    where
        I2C: I2c,
        DELAY: DelayNs,
    {
        type Error = I2C::Error;

        async fn read_id(&mut self) -> Result<u8, Self::Error> {
            let mut data = [0u8; 2];

            tri!(
                self.i2c
                    .write_read(self.addr_u8(), &[Register::ChipId as u8], &mut data)
                    .await
            );

            Ok(data[0])
        }

        async fn read_calibration(&mut self) -> Result<Calibration, Self::Error> {
            let mut data = [0u8; 22];

            tri!(
                self.i2c
                    .write_read(self.addr_u8(), &[Register::CalibrationAc1 as u8], &mut data)
                    .await
            );

            Ok(Calibration::from_slice(&data))
        }
    }

    impl<I2C, DELAY> AsyncBMP180<I2C, DELAY> for BMP180<I2C, DELAY>
    where
        I2C: I2c,
        DELAY: DelayNs,
    {
        type Error = I2C::Error;

        async fn read_raw_temperature(&mut self) -> Result<i16, Self::Error> {
            tri!(
                self.i2c
                    .write(
                        self.addr_u8(),
                        &[Register::Control as u8, Register::ReadTempCmd as u8]
                    )
                    .await
            );

            self.delay.delay_ms(5).await;

            let mut data = [0u8; 2];

            tri!(
                self.i2c
                    .write_read(
                        self.addr_u8(),
                        &[Register::TempPressureData as u8],
                        &mut data
                    )
                    .await
            );

            let raw_temperature = ((data[0] as i16) << 8) | data[1] as i16;

            Ok(raw_temperature)
        }

        async fn read_raw_pressure(&mut self) -> Result<i32, Self::Error> {
            let mode = self.mode();

            tri!(
                self.i2c
                    .write(
                        self.addr_u8(),
                        &[
                            Register::Control as u8,
                            Register::ReadPressureCmd as u8 + ((mode as u8) << 6)
                        ],
                    )
                    .await
            );

            self.delay.delay_ms(mode.delay_ms()).await;

            let mut data = [0u8; 3];

            tri!(
                self.i2c
                    .write_read(
                        self.addr_u8(),
                        &[Register::TempPressureData as u8],
                        &mut data
                    )
                    .await
            );

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

    use crate::{
        device::register::Register, functionality::blocking::BlockingBMP180, tri, BaseBMP180,
    };

    use super::{calibration::Calibration, BMP180};

    impl<I2C, DELAY> BlockingBMP180<I2C, DELAY> for BMP180<I2C, DELAY>
    where
        I2C: I2c,
        DELAY: DelayNs,
    {
        type Error = I2C::Error;

        fn read_id(&mut self) -> Result<u8, Self::Error> {
            let mut data = [0u8; 2];

            tri!(self
                .i2c
                .write_read(self.addr_u8(), &[Register::ChipId as u8], &mut data));

            Ok(data[0])
        }

        fn read_calibration(&mut self) -> Result<Calibration, Self::Error> {
            let mut data = [0u8; 22];

            tri!(self
                .i2c
                .write_read(self.addr_u8(), &[Register::CalibrationAc1 as u8], &mut data));

            Ok(Calibration::from_slice(&data))
        }

        fn read_raw_temperature(&mut self) -> Result<i16, Self::Error> {
            tri!(self.i2c.write(
                self.addr_u8(),
                &[Register::Control as u8, Register::ReadTempCmd as u8]
            ));

            self.delay.delay_ms(5);

            let mut data = [0u8; 2];

            tri!(self.i2c.write_read(
                self.addr_u8(),
                &[Register::TempPressureData as u8],
                &mut data
            ));

            let raw_temperature = ((data[0] as i16) << 8) | data[1] as i16;

            Ok(raw_temperature)
        }

        fn read_raw_pressure(&mut self) -> Result<i32, Self::Error> {
            let mode = self.mode();

            tri!(self.i2c.write(
                self.addr_u8(),
                &[
                    Register::Control as u8,
                    Register::ReadPressureCmd as u8 + ((mode as u8) << 6)
                ],
            ));

            self.delay.delay_ms(mode.delay_ms());

            let mut data = [0u8; 3];

            tri!(self.i2c.write_read(
                self.addr_u8(),
                &[Register::TempPressureData as u8],
                &mut data
            ));

            let raw_pressure =
                (((data[0] as i32) << 16) + ((data[1] as i32) << 8) + data[2] as i32)
                    >> (8 - mode as u8);

            Ok(raw_pressure)
        }
    }
}
