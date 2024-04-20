//! Device definition and implementation.

use duplicate::duplicate_item;

/// Identity trait.
///
/// Used to trick the compiler while using [`duplicate_item`] to implement `async` and `blocking` versions of the same module.
/// Using this trait, we can write normal rust code that can also be formatted by `rustfmt`.
trait Identity: Sized {
    fn identity(self) -> Self {
        self
    }
}

impl<T: Sized> Identity for T {}

#[duplicate_item(
    feature_        module        async     await               i2c_trait                       delay_trait;
    ["async"]       [asynch]      [async]   [await.identity()]  [embedded_hal_async::i2c::I2c]  [embedded_hal_async::delay::DelayNs];
    ["blocking"]    [blocking]    []        [identity()]        [embedded_hal::i2c::I2c]        [embedded_hal::delay::DelayNs];
)]
pub mod module {
    //! Device definition and implementation.

    #[cfg(feature=feature_)]
    mod inner {
        use super::super::Identity;

        use crate::{
            address::Address, calibration::Calibration, error::BMP180Error, id::Id, mode::Mode,
            register::Register, tri, tri_opt,
        };

        /// Builder for an uninitialized `BMP180` device.
        ///
        /// Helpful for using default values.
        #[derive(Clone)]
        #[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
        #[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
        pub struct UninitBMP180Builder<I2C, DELAY> {
            inner: UninitBMP180<I2C, DELAY>,
        }

        impl<I2C, DELAY> UninitBMP180Builder<I2C, DELAY>
        where
            I2C: i2c_trait,
            DELAY: delay_trait,
        {
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

            /// Build the `BMP180` device.
            pub fn build(self) -> UninitBMP180<I2C, DELAY> {
                self.inner
            }
        }

        /// Uninitialized `BMP180` device.
        ///
        /// This struct is used to initialize the `BMP180` device.
        #[derive(Clone)]
        #[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
        #[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
        pub struct UninitBMP180<I2C, DELAY> {
            /// Device I2C address.
            addr: Address,
            /// Device mode.
            mode: Mode,
            /// Device I2C bus.
            i2c: I2C,
            /// Delay provider.
            delay: DELAY,
        }

        impl<I2C, DELAY> UninitBMP180<I2C, DELAY>
        where
            I2C: i2c_trait,
            DELAY: delay_trait,
        {
            /// Create a new uninitialized `BMP180` device.
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

            /// Device I2C address as `u8`.
            fn addr_u8(&self) -> u8 {
                self.addr.into()
            }

            /// Read device ID.
            async fn read_id(&mut self) -> Result<u8, I2C::Error> {
                let mut data = [0u8; 2];

                tri!(
                    self.i2c
                        .write_read(self.addr_u8(), &[Register::ChipId as u8], &mut data)
                        .await
                );

                Ok(data[0])
            }

            /// Validate device ID.
            fn validate_id(id: u8) -> bool {
                Id::is_valid(id)
            }

            /// Read calibration data.
            async fn read_calibration(&mut self) -> Result<Calibration, I2C::Error> {
                let mut data = [0u8; 22];

                tri!(
                    self.i2c
                        .write_read(self.addr_u8(), &[Register::CalibrationAc1 as u8], &mut data)
                        .await
                );

                Ok(Calibration::from_slice(&data))
            }

            /// Initialize `BMP180` device.
            pub async fn initialize(
                mut self,
            ) -> Result<BMP180<I2C, DELAY>, BMP180Error<I2C::Error>> {
                let id = tri!(self.read_id().await.map_err(BMP180Error::I2C));

                if !Self::validate_id(id) {
                    return Err(BMP180Error::InvalidId(id));
                }

                let calibration = tri!(self.read_calibration().await.map_err(BMP180Error::I2C));

                let bmp180 = BMP180 {
                    addr: self.addr,
                    mode: self.mode,
                    calibration,
                    temperature: 0,
                    pressure: 0,
                    i2c: self.i2c,
                    delay: self.delay,
                };

                Ok(bmp180)
            }
        }

        /// `BMP180` device.
        ///
        /// Represents an initialized BMP180 device valid id and its calibration data set.
        #[derive(Clone)]
        #[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
        #[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
        pub struct BMP180<I2C, DELAY> {
            addr: Address,
            mode: Mode,
            calibration: Calibration,
            temperature: i32,
            pressure: i32,
            i2c: I2C,
            delay: DELAY,
        }

        impl<I2C, DELAY> BMP180<I2C, DELAY>
        where
            I2C: i2c_trait,
            DELAY: delay_trait,
        {
            /// Device I2C address.
            pub fn addr(&self) -> Address {
                self.addr
            }

            /// Device I2C address as `u8`.
            fn addr_u8(&self) -> u8 {
                self.addr.into()
            }

            /// Device operating mode.
            pub fn mode(&self) -> Mode {
                self.mode
            }

            /// Device calibration data.
            pub fn calibration(&self) -> &Calibration {
                &self.calibration
            }

            /// True temperature in `0.1 C` according to the calibration data.
            pub fn temperature(&self) -> i32 {
                self.temperature
            }

            /// Temperature in Celsius.
            pub fn temperature_celsius(&self) -> f32 {
                self.temperature() as f32 / 10.0
            }

            /// True pressure in `Pa`according to the calibration data.
            pub fn pressure(&self) -> i32 {
                self.pressure
            }

            /// Compute B5 value.
            fn compute_b5(&self, raw_temperature: i16) -> Option<i32> {
                let calibration = self.calibration();

                let x1 = ((raw_temperature as i32 - calibration.ac6 as i32)
                    * calibration.ac5 as i32)
                    >> 15;
                let x2 = ((calibration.mc as i32) << 11) / (x1 + calibration.md as i32);

                Some(x1 + x2)
            }

            /// Compute true temprature in `0.1 C`.
            fn compute_temperature(&self, raw_temperature: i16) -> Option<i32> {
                let b5 = tri_opt!(self.compute_b5(raw_temperature));

                #[cfg(feature = "defmt")]
                {
                    defmt::debug!("Computing temperature");
                    defmt::debug!("Raw temperature: {}", raw_temperature);
                    defmt::debug!("B5: {}", b5);
                }

                #[cfg(feature = "log")]
                {
                    log::debug!("Computing temperature");
                    log::debug!("Raw temperature: {}", raw_temperature);
                    log::debug!("B5: {}", b5);
                }

                Some((b5 + 8) >> 4)
            }

            /// Compute true pressure in `Pa`.
            fn compute_pressure(&self, raw_temperature: i16, raw_pressure: i32) -> Option<i32> {
                let calibration = self.calibration();
                let mode = self.mode();

                #[cfg(feature = "defmt")]
                {
                    defmt::debug!("Computing pressure");
                    defmt::debug!("Raw temperature: {}", raw_temperature);
                    defmt::debug!("Raw pressure: {}", raw_pressure);
                }

                #[cfg(feature = "log")]
                {
                    log::debug!("Computing pressure");
                    log::debug!("Raw temperature: {}", raw_temperature);
                    log::debug!("Raw pressure: {}", raw_pressure);
                }

                let b5 = tri_opt!(self.compute_b5(raw_temperature));

                let b6 = b5 - 4000;
                let x1 = (calibration.b2 as i32 * ((b6 * b6) >> 12)) >> 11;
                let x2 = (calibration.ac2 as i32 * b6) >> 11;
                let x3 = x1 + x2;
                let b3 = ((((calibration.ac1 as i32) * 4 + x3) << mode as u8) + 2) / 4;

                #[cfg(feature = "defmt")]
                {
                    defmt::debug!("B5: {}", b5);
                    defmt::debug!("B6: {}", b6);
                    defmt::debug!("X1: {}", x1);
                    defmt::debug!("X2: {}", x2);
                    defmt::debug!("X3: {}", x3);
                    defmt::debug!("B3: {}", b3);
                }

                #[cfg(feature = "log")]
                {
                    log::debug!("B5: {}", b5);
                    log::debug!("B6: {}", b6);
                    log::debug!("X1: {}", x1);
                    log::debug!("X2: {}", x2);
                    log::debug!("X3: {}", x3);
                    log::debug!("B3: {}", b3);
                }

                let x1 = (calibration.ac3 as i32 * b6) >> 13;
                let x2 = (calibration.b1 as i32 * ((b6 * b6) >> 12)) >> 16;
                let x3 = ((x1 + x2) + 2) >> 2;
                let b4 = ((calibration.ac4 as u32) * ((x3 + 32768) as u32)) >> 15;
                let b7 = (raw_pressure as u32 - b3 as u32) * (50000 >> mode as u8);

                #[cfg(feature = "defmt")]
                {
                    defmt::debug!("X1: {}", x1);
                    defmt::debug!("X2: {}", x2);
                    defmt::debug!("X3: {}", x3);
                    defmt::debug!("B4: {}", b4);
                    defmt::debug!("B7: {}", b7);
                }

                #[cfg(feature = "log")]
                {
                    log::debug!("X1: {}", x1);
                    log::debug!("X2: {}", x2);
                    log::debug!("X3: {}", x3);
                    log::debug!("B4: {}", b4);
                    log::debug!("B7: {}", b7);
                }

                let p = if b7 < 0x80000000 {
                    (b7 * 2) / b4
                } else {
                    (b7 / b4) * 2
                } as i32;

                let x1 = (p >> 8) * (p >> 8);
                let x1 = (x1 * 3038) >> 16;
                let x2 = (-7357 * p) >> 16;

                #[cfg(feature = "defmt")]
                {
                    let p = p + ((x1 + x2 + 3791_i32) >> 4);

                    defmt::debug!("X1: {}", x1);
                    defmt::debug!("X2: {}", x2);
                    defmt::debug!("P: {}", p);
                }

                #[cfg(feature = "log")]
                {
                    let p = p + ((x1 + x2 + 3791_i32) >> 4);

                    log::debug!("X1: {}", x1);
                    log::debug!("X2: {}", x2);
                    log::debug!("P: {}", p);
                }

                Some(p + ((x1 + x2 + 3791_i32) >> 4))
            }

            /// Pressure in `Pa` at sea level.
            pub fn sea_level_pressure(&self, altitude_meters: f32) -> i32 {
                let pressure = self.pressure() as f32;

                (pressure / libm::powf(1.0 - altitude_meters / 44330.0, 5.255)) as i32
            }

            /// Altitude in meters.
            ///
            /// Standard pressure at sea level is `101325 Pa`.
            pub fn altitude(&self, sea_level_pressure: f32) -> f32 {
                let pressure = self.pressure();

                44330.0 * (1.0 - libm::powf(pressure as f32 / sea_level_pressure, 0.1903))
            }

            /// Read raw temperature.
            async fn read_raw_temperature(&mut self) -> Result<i16, BMP180Error<I2C::Error>> {
                tri!(self
                    .i2c
                    .write(
                        self.addr_u8(),
                        &[Register::Control as u8, Register::ReadTempCmd as u8]
                    )
                    .await
                    .map_err(BMP180Error::I2C));

                self.delay.delay_ms(5).await;

                let mut data = [0u8; 2];

                tri!(self
                    .i2c
                    .write_read(
                        self.addr_u8(),
                        &[Register::TempPressureData as u8],
                        &mut data
                    )
                    .await
                    .map_err(BMP180Error::I2C));

                let raw_temperature = ((data[0] as i16) << 8) | data[1] as i16;

                Ok(raw_temperature)
            }

            /// Read raw pressure.
            async fn read_raw_pressure(&mut self) -> Result<i32, BMP180Error<I2C::Error>> {
                let mode = self.mode();

                tri!(self
                    .i2c
                    .write(
                        self.addr_u8(),
                        &[
                            Register::Control as u8,
                            Register::ReadPressureCmd as u8 + ((mode as u8) << 6)
                        ],
                    )
                    .await
                    .map_err(BMP180Error::I2C));

                self.delay.delay_ms(mode.delay_ms()).await;

                let mut data = [0u8; 3];

                tri!(self
                    .i2c
                    .write_read(
                        self.addr_u8(),
                        &[Register::TempPressureData as u8],
                        &mut data
                    )
                    .await
                    .map_err(BMP180Error::I2C));

                let raw_pressure =
                    (((data[0] as i32) << 16) + ((data[1] as i32) << 8) + data[2] as i32)
                        >> (8 - mode as u8);

                Ok(raw_pressure)
            }

            /// Update temperature in `self`.
            pub async fn update_temperature(&mut self) -> Result<(), BMP180Error<I2C::Error>> {
                let raw_temperature = tri!(self.read_raw_temperature().await);

                self.temperature = tri!(self
                    .compute_temperature(raw_temperature)
                    .ok_or(BMP180Error::Arithmetic));

                Ok(())
            }

            /// Update pressure in `self`.
            pub async fn update_pressure(&mut self) -> Result<(), BMP180Error<I2C::Error>> {
                let raw_temperature = tri!(self.read_raw_temperature().await);
                let raw_pressure = tri!(self.read_raw_pressure().await);

                self.pressure = tri!(self
                    .compute_pressure(raw_temperature, raw_pressure)
                    .ok_or(BMP180Error::Arithmetic));

                Ok(())
            }

            /// Update both temperature and pressure in `self`.
            pub async fn update(&mut self) -> Result<(), BMP180Error<I2C::Error>> {
                let raw_temperature = tri!(self.read_raw_temperature().await);
                let raw_pressure = tri!(self.read_raw_pressure().await);

                self.temperature = tri!(self
                    .compute_temperature(raw_temperature)
                    .ok_or(BMP180Error::Arithmetic));

                self.pressure = tri!(self
                    .compute_pressure(raw_temperature, raw_pressure)
                    .ok_or(BMP180Error::Arithmetic));

                Ok(())
            }
        }

        #[cfg(feature = "i-know-what-i-am-doing")]
        impl<I2C, DELAY> BMP180<I2C, DELAY> {
            /// Split the `BMP180` device into its parts.
            ///
            /// Only available when the `i-know-what-i-am-doing` feature is enabled.
            pub fn into_parts(self) -> (Address, Mode, Calibration, i32, i32, I2C, DELAY) {
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

            /// Create a `BMP180` device from its parts.
            ///
            /// Only available when the `i-know-what-i-am-doing` feature is enabled.
            pub fn from_parts(
                addr: Address,
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
    }

    #[cfg(feature=feature_)]
    pub use inner::*;
}
