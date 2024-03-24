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

pub(crate) trait BaseBMP180<I2C, DELAY>: Sized {
    fn new(mode: Mode, i2c: I2C, delay: DELAY) -> Self;

    fn set_calibration(&mut self, calibration: Calibration);

    fn validate_id(id: u8) -> bool {
        id == BMP180_ID
    }

    fn compute_b5(calibration: &Calibration, raw_temperature: i32) -> i32 {
        let x1 = ((raw_temperature - calibration.ac6 as i32) * calibration.ac5 as i32) >> 15;
        let x2 = ((calibration.mc as i32) << 11) / (x1 + calibration.md as i32);
        x1 + x2
    }

    // fn compute_pressure(
    //     mode: Mode,
    //     calibration: &Calibration,
    //     raw_temperature: i32,
    //     raw_pressure: u32,
    // ) -> i32 {
    //     let b5 = Self::compute_b5(calibration, raw_temperature);

    //     let b6 = b5 - 4000;
    //     let x1 = (calibration.b2 as i32 * (b6 * b6 >> 12)) >> 11;
    //     let x2 = (calibration.ac2 as i32 * b6) >> 11;
    //     let x3 = x1 + x2;
    //     let b3 = ((((calibration.ac1 as i32) * 4 + x3) << mode as u8) + 2) / 4;

    //     let x1 = (calibration.ac3 as i32 * b6) >> 13;
    //     let x2 = (calibration.b1 as i32 * (b6 * b6 >> 12)) >> 16;
    //     let x3 = ((x1 + x2) + 2) >> 2;
    //     let b4 = (calibration.ac4 as u32) * ((x3 + 32768) as u32) >> 15;

    //     // B7 = ((uint32_t)UP - B3) * (uint32_t)(50000UL >> oversampling);

    //     let b7 = ((raw_pressure as i32) - b3) * (50000 >> mode as u8);

    //     let p = if b7 < 0x80000000 {
    //         (b7 * 2) / b4
    //     } else {
    //         (b7 / b4) * 2
    //     };

    //     let x1 = (p >> 8) * (p >> 8);
    //     let x1 = (x1 * 3038) >> 16;
    //     let x2 = (-7357 * p) >> 16;

    //     p + ((x1 + x2 + 3791_i32) >> 4)
    // }
}

#[allow(private_bounds)]
#[allow(async_fn_in_trait)]
pub trait AsyncBMP180<I2C, DELAY>: BaseBMP180<I2C, DELAY> {
    type Error;

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

    async fn read_id(&mut self) -> Result<u8, Self::Error>;

    async fn read_calibration(&mut self) -> Result<Calibration, Self::Error>;

    async fn read_raw_temperature(&mut self) -> u16;

    async fn read_temperature(&mut self) -> f32;

    async fn read_raw_pressure(&mut self) -> u32;

    async fn read_pressure(&mut self) -> i32;
}
