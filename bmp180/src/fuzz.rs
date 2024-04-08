//! Fuzzing utilities.

use core::convert::Infallible;

use crate::device::{id::Id, register::Register};

/// Fuzzing delay.
///
/// Does nothing.
pub struct FuzzDelay;

impl embedded_hal::delay::DelayNs for FuzzDelay {
    fn delay_ns(&mut self, _: u32) {}
}

/// Fuzzing I2C.
///
/// Responds with the correct id. Erverything else is random.
pub struct FuzzI2C<'data> {
    /// Data to respond with.
    data: &'data [u8],

    /// Check if the current write is to the id register.
    is_id_write: bool,
}

impl<'data> FuzzI2C<'data> {
    /// Create a new `FuzzI2C`.
    pub fn new(data: &'data [u8]) -> Self {
        Self {
            data,
            is_id_write: false,
        }
    }
}

impl embedded_hal::i2c::ErrorType for FuzzI2C<'_> {
    type Error = Infallible;
}

impl embedded_hal::i2c::I2c for FuzzI2C<'_> {
    fn transaction(
        &mut self,
        _address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                embedded_hal::i2c::Operation::Write(write) => {
                    if write[0] == Register::ChipId as u8 {
                        self.is_id_write = true;
                    } else {
                        self.is_id_write = false;
                    }
                }
                embedded_hal::i2c::Operation::Read(read) => {
                    if self.is_id_write {
                        read[0] = Id::Valid as u8;
                    } else {
                        if self.data.len() == read.len() {
                            read.copy_from_slice(self.data);
                        } else if self.data.len() < read.len() {
                            read[..self.data.len()].copy_from_slice(self.data);
                        } else {
                            read.copy_from_slice(&self.data[..read.len()]);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
