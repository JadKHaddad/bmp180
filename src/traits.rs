use crate::{calibration::Calibration, mode::Mode, BMP180_ID};

pub enum BMP180Error<E> {
    I2C(E),
    InvalidId,
}

impl<E> From<E> for BMP180Error<E> {
    fn from(error: E) -> Self {
        Self::I2C(error)
    }
}

pub(crate) trait Sealed {}

#[allow(private_bounds)]
pub trait BaseBMP180<I2C, DELAY>: Sealed + Sized {
    fn new(mode: Mode, i2c: I2C, delay: DELAY) -> Self;

    fn set_calibration(&mut self, calibration: Calibration);

    fn validate_id(id: u8) -> bool {
        id == BMP180_ID
    }
}

#[allow(async_fn_in_trait)]
pub trait AsyncBMP180<I2C, DELAY>: BaseBMP180<I2C, DELAY> {
    type Error;

    async fn initialized(
        mode: Mode,
        i2c: I2C,
        delay: DELAY,
    ) -> Result<Self, BMP180Error<Self::Error>> {
        let mut bmp180 = Self::new(mode, i2c, delay);

        let id = bmp180.read_id().await?;

        if !Self::validate_id(id) {
            return Err(BMP180Error::InvalidId);
        }

        let calibration = bmp180.read_calibration().await?;

        bmp180.set_calibration(calibration);

        Ok(bmp180)
    }

    async fn read_id(&mut self) -> Result<u8, Self::Error>;

    async fn read_calibration(&mut self) -> Result<Calibration, Self::Error>;

    async fn read_raw_temperature(&mut self) -> u16;

    async fn read_temperature(&mut self) -> f32;

    async fn read_raw_pressure(&mut self) -> u32;

    async fn read_pressure(&mut self) -> i32;
}
