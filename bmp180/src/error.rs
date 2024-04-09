/// Error type for BMP180 devices.
pub enum BMP180Error<I2CError> {
    /// I2C error.
    I2C(I2CError),
    /// Invalid device ID.
    InvalidId(u8),
    /// Aretmetic error.
    ///
    /// Accurs on:
    /// - Deviding by zero.
    /// - Overflow.
    /// - Underflow.
    Arithmetic,
}
