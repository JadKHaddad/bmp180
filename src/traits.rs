use crate::{calibration::Calibration, mode::Mode, BMP180_ID};

pub enum BMP180Error<I2CError> {
    I2C(I2CError),
    InvalidId(u8),
}

impl<I2CError> From<I2CError> for BMP180Error<I2CError> {
    fn from(error: I2CError) -> Self {
        Self::I2C(error)
    }
}

impl<I2CError> core::fmt::Debug for BMP180Error<I2CError>
where
    I2CError: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::I2C(err) => write!(f, "I2C error: {err:?}"),
            Self::InvalidId(id) => write!(
                f,
                "Invalid ID. Expected 0x{BMP180_ID:02X}, found 0x{id:02X}"
            ),
        }
    }
}
pub(crate) trait PrivateBaseBMP180<I2C, DELAY> {
    fn set_calibration(&mut self, calibration: Calibration);

    fn set_temperature(&mut self, temperature: i32);

    fn set_pressure(&mut self, pressure: i32);

    fn validate_id(id: u8) -> bool {
        id == BMP180_ID
    }
}

pub trait BaseBMP180<I2C, DELAY>: Sized {
    fn new(mode: Mode, i2c: I2C, delay: DELAY) -> Self;

    fn mode(&self) -> Mode;

    fn calibration(&self) -> &Calibration;

    /// True temperature according to the calibration data.
    fn temperature(&self) -> i32;

    /// True pressure in `Pa`according to the calibration data.
    fn pressure(&self) -> i32;

    /// Temperature in Celsius.
    fn temperature_celsius(&self) -> f32 {
        self.temperature() as f32 / 10.0
    }

    /// Compute B5 value.
    ///
    /// Exposed to the public API because why not.
    fn compute_b5(&self, raw_temperature: i16) -> i32 {
        let calibration = self.calibration();

        let x1 = ((raw_temperature as i32 - calibration.ac6 as i32) * calibration.ac5 as i32) >> 15;
        let x2 = ((calibration.mc as i32) << 11) / (x1 + calibration.md as i32);

        x1 + x2
    }

    /// Compute true temprature.
    ///
    /// Exposed to the public API because why not.
    fn compute_temperature(&self, raw_temperature: i16) -> i32 {
        let b5 = self.compute_b5(raw_temperature);

        #[cfg(feature = "log")]
        {
            log::debug!("Computing temperature");
            log::debug!("Raw temperature: {}", raw_temperature);
            log::debug!("B5: {}", b5);
        }

        (b5 + 8) >> 4
    }

    /// Compute true pressure in `Pa`.
    ///
    /// Exposed to the public API because why not.
    fn compute_pressure(&self, raw_temperature: i16, raw_pressure: i32) -> i32 {
        let calibration = self.calibration();
        let mode = self.mode();

        #[cfg(feature = "log")]
        {
            log::debug!("Computing pressure");
            log::debug!("Raw temperature: {}", raw_temperature);
            log::debug!("Raw pressure: {}", raw_pressure);
        }

        let b5 = self.compute_b5(raw_temperature);

        let b6 = b5 - 4000;
        let x1 = (calibration.b2 as i32 * ((b6 * b6) >> 12)) >> 11;
        let x2 = (calibration.ac2 as i32 * b6) >> 11;
        let x3 = x1 + x2;
        let b3 = ((((calibration.ac1 as i32) * 4 + x3) << mode as u8) + 2) / 4;

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

        let p = p + ((x1 + x2 + 3791_i32) >> 4);

        #[cfg(feature = "log")]
        {
            log::debug!("P: {}", p);
            log::debug!("X1: {}", x1);
            log::debug!("X2: {}", x2);
            log::debug!("P: {}", p);
        }

        p
    }

    /// Pressure in `Pa` at sea level.
    fn sea_level_pressure(&self, altitude_meters: f32) -> i32 {
        let pressure = self.pressure() as f32;

        (pressure / libm::powf(1.0 - altitude_meters / 44330.0, 5.255)) as i32
    }

    /// Altitude in meters.
    ///
    /// See [SeaLevelPressure](crate::SeaLevelPressure) wich has a default value of 101325 Pa.
    fn altitude(&self, sea_level_pressure: f32) -> f32 {
        let pressure = self.pressure();

        44330.0 * (1.0 - libm::powf(pressure as f32 / sea_level_pressure, 0.1903))
    }
}

#[allow(private_bounds)]
#[allow(async_fn_in_trait)]
pub trait AsyncBMP180<I2C, DELAY>: PrivateBaseBMP180<I2C, DELAY> + BaseBMP180<I2C, DELAY> {
    type Error;

    /// Read Device ID.
    async fn read_id(&mut self) -> Result<u8, Self::Error>;

    /// Read calibration data.
    async fn read_calibration(&mut self) -> Result<Calibration, Self::Error>;

    /// Read raw temperature.
    async fn read_raw_temperature(&mut self) -> Result<i16, Self::Error>;

    /// Read raw pressure.
    async fn read_raw_pressure(&mut self) -> Result<i32, Self::Error>;

    /// Create a new `BMP180` instance.
    fn new(mode: Mode, i2c: I2C, delay: DELAY) -> Self {
        <Self as BaseBMP180<I2C, DELAY>>::new(mode, i2c, delay)
    }

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
