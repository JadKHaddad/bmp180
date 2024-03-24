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

    fn validate_id(id: u8) -> bool {
        id == BMP180_ID
    }
}

pub trait BaseBMP180<I2C, DELAY>: Sized {
    fn new(mode: Mode, i2c: I2C, delay: DELAY) -> Self;

    fn mode(&self) -> Mode;

    fn calibration(&self) -> &Calibration;

    fn compute_b5(calibration: &Calibration, raw_temperature: i32) -> i32 {
        let x1 = ((raw_temperature - calibration.ac6 as i32) * calibration.ac5 as i32) >> 15;
        let x2 = ((calibration.mc as i32) << 11) / (x1 + calibration.md as i32);
        x1 + x2
    }

    fn compute_temperature(calibration: &Calibration, raw_temperature: i32) -> f32 {
        let b5 = Self::compute_b5(calibration, raw_temperature);
        ((b5 + 8) >> 4) as f32 / 10.0
    }

    fn compute_pressure(
        mode: Mode,
        calibration: &Calibration,
        raw_temperature: i32,
        raw_pressure: u32,
    ) -> i32 {
        #[cfg(feature = "log")]
        {
            log::debug!("Raw temperature: {}", raw_temperature);
            log::debug!("Raw pressure: {}", raw_pressure);
        }

        let b5 = Self::compute_b5(calibration, raw_temperature);

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
        let b7 = (raw_pressure - b3 as u32) * (50000 >> mode as u8);

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

        #[cfg(feature = "log")]
        {
            log::debug!("P: {}", p);
            log::debug!("X1: {}", x1);
            log::debug!("X2: {}", x2);
        }

        let p = p + ((x1 + x2 + 3791_i32) >> 4);

        #[cfg(feature = "log")]
        {
            log::debug!("P: {}", p);
        }

        p
    }

    fn calculate_sea_level_pressure(pressure: i32, altitude_meters: f32) -> i32 {
        let pressure = pressure as f32;

        (pressure / libm::powf(1.0 - altitude_meters / 44330.0, 5.255)) as i32
    }

    fn compute_altitude(pressure: i32, sea_level_pressure: i32) -> f32 {
        44330.0 * (1.0 - libm::powf(pressure as f32 / sea_level_pressure as f32, 0.1903))
    }
}

#[allow(private_bounds)]
#[allow(async_fn_in_trait)]
pub trait AsyncBMP180<I2C, DELAY>: PrivateBaseBMP180<I2C, DELAY> + BaseBMP180<I2C, DELAY> {
    type Error;

    async fn read_id(&mut self) -> Result<u8, Self::Error>;

    async fn read_calibration(&mut self) -> Result<Calibration, Self::Error>;

    async fn read_raw_temperature(&mut self) -> Result<u16, Self::Error>;

    async fn read_raw_pressure(&mut self) -> Result<u32, Self::Error>;

    fn new(mode: Mode, i2c: I2C, delay: DELAY) -> Self {
        <Self as BaseBMP180<I2C, DELAY>>::new(mode, i2c, delay)
    }

    async fn initialize(&mut self) -> Result<(), BMP180Error<Self::Error>> {
        let id = self.read_id().await?;

        if !Self::validate_id(id) {
            return Err(BMP180Error::InvalidId(id));
        }

        let calibration = self.read_calibration().await?;

        self.set_calibration(calibration);

        Ok(())
    }

    async fn initialized(
        mode: Mode,
        i2c: I2C,
        delay: DELAY,
    ) -> Result<Self, BMP180Error<Self::Error>> {
        let mut bmp180 = <Self as BaseBMP180<I2C, DELAY>>::new(mode, i2c, delay);

        bmp180.initialize().await?;

        Ok(bmp180)
    }

    async fn read_temperature(&mut self) -> Result<f32, Self::Error> {
        let raw_temperature = self.read_raw_temperature().await? as i32;
        let calibration = self.calibration();

        Ok(Self::compute_temperature(calibration, raw_temperature))
    }

    async fn read_pressure(&mut self) -> Result<i32, Self::Error> {
        let raw_temperature = self.read_raw_temperature().await? as i32;
        let raw_pressure = self.read_raw_pressure().await?;

        let calibration = self.calibration();
        let mode = self.mode();

        Ok(Self::compute_pressure(
            mode,
            calibration,
            raw_temperature,
            raw_pressure,
        ))
    }

    async fn read_sea_level_pressure_with_altitude_meters(
        &mut self,
        altitude_meters: f32,
    ) -> Result<i32, Self::Error> {
        let pressure = self.read_pressure().await?;

        Ok(Self::calculate_sea_level_pressure(
            pressure,
            altitude_meters,
        ))
    }

    async fn read_altitude_with_sea_level_pressure(
        &mut self,
        sea_level_pressure: i32,
    ) -> Result<f32, Self::Error> {
        let pressure = self.read_pressure().await?;

        Ok(Self::compute_altitude(pressure, sea_level_pressure))
    }

    async fn read_altitude_with_altitude_meters(
        &mut self,
        altitude_meters: f32,
    ) -> Result<f32, Self::Error> {
        let pressure = self.read_pressure().await?;
        let sea_level_pressure = Self::calculate_sea_level_pressure(pressure, altitude_meters);

        Ok(Self::compute_altitude(pressure, sea_level_pressure))
    }
}
