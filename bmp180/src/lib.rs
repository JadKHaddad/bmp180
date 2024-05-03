//! A platform agnostic driver to interface with the `BMP180` (pressure sensor) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! # Features
//! The following features are available:
//! - `blocking`: enables blocking functionality.
//! - `async`: enables asynchronous functionality.
//! - `log`: enables debug logging using the `log` crate.
//! - `defmt`: enables debug logging using the `defmt` crate.
//! - `impl-debug`: implements `core::fmt::Debug` for structs and enums.
//! - `impl-defmt-format`: implements `defmt::Format` for structs and enums.
//! - `fuzz`: enables the `fuzz` module for fuzz testing.
//! - `disable-arithmetic-checks`: disables arithmetic checks.
//! - `i-know-what-i-am-doing`: allows you to split an initialized device into its parts and put it back together.
//! Useful when you want to release the I2C bus and use it for something else.
//! This is not recommended though, you can use [`embedded-hal-bus`](https://crates.io/crates/embedded-hal-bus)
//! or [`embassy-embedded-hal`](https://crates.io/crates/embassy-embedded-hal) to share the I2C bus.

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod address;
mod calibration;
mod device;
mod error;
mod id;
mod mode;
mod register;

#[cfg(feature = "fuzz")]
pub mod fuzz;

pub use crate::address::Address;
pub use crate::calibration::Calibration;
pub use crate::error::BMP180Error;
pub use crate::id::Id;
pub use crate::mode::Mode;

#[cfg(feature = "async")]
pub use crate::device::asynch;

#[cfg(feature = "blocking")]
pub use crate::device::blocking;

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
