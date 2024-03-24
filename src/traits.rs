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
