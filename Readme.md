# BMP180

A platform agnostic driver to interface with the BMP180 (pressure sensor) using the [`embedded-hal`](https://crates.io/crates/embedded-hal) and [`embedded-hal-async`](https://crates.io/crates/embedded-hal-async) traits.

## Features
The following features are available:
- `blocking`: enables blocking functionality.
- `async`: enables asynchronous functionality.
- `log`: enables debug logging.
- `i-know-what-i-am-doing`: allows you to split an initialized device into its parts and put it back together.
Usefull when you want to release the I2C bus and use it for something else.
This is not recommended though, you can use [`embedded-hal-bus`](https://crates.io/crates/embedded-hal-bus)
or [`embassy-embedded-hal`](https://crates.io/crates/embassy-embedded-hal) to share the I2C bus.

## Usage
See examples folder.