//! A platform agnostic driver to interface with the `BMP180` (pressure sensor) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! # Features
//! The following features are available:
//! - `blocking`: enables blocking functionality.
//! - `async`: enables asynchronous functionality.
//! - `log`: enables debug logging.
//! - `i-know-what-i-am-doing`: allows you to split an initialized device into its parts and put it back together.
//! Usefull when you want to release the I2C bus and use it for something else.
//! This is not recommended though, you can use [`embedded-hal-bus`](https://crates.io/crates/embedded-hal-bus)
//! or [`embassy-embedded-hal`](https://crates.io/crates/embassy-embedded-hal) to share the I2C bus.

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod device;
pub mod functionality;

pub use crate::device::address::Address;
pub use crate::device::calibration::Calibration;
pub use crate::device::mode::Mode;
pub use crate::device::{UninitBMP180, UninitBMP180Builder, BMP180};

pub use crate::functionality::BMP180Error;
pub use crate::functionality::BaseBMP180;

#[cfg(feature = "async")]
pub use crate::functionality::asynchronous::{AsyncBMP180, AsyncInitBMP180};
#[cfg(feature = "blocking")]
pub use crate::functionality::blocking::{BlockingBMP180, BlockingInitBMP180};

// #[cfg(feature = "fuzz")]
pub mod fuzz;

/// Our custom `try!` macro aka `?`, to get rid of [`core::convert::From`]/[`core::convert::Into`] used by the `?` operator.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(value) => value,
            core::result::Result::Err(err) => {
                return core::result::Result::Err(err);
            }
        }
    };
}

use tri;
