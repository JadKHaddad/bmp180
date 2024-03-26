#![no_std]

pub(crate) mod constants;
pub mod device;
pub mod functionality;

pub use crate::device::calibration::Calibration;
pub use crate::device::mode::Mode;
pub use crate::device::BMP180;

pub use crate::functionality::BMP180Error;
pub use crate::functionality::BaseBMP180;

#[cfg(feature = "async")]
pub use crate::functionality::asynchronous::AsyncBMP180;
#[cfg(feature = "blocking")]
pub use crate::functionality::blocking::BlockingBMP180;
