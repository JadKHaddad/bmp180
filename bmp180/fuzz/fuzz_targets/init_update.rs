#![no_main]

use bmp180_embedded_hal::{
    blocking::UninitBMP180,
    fuzz::{FuzzDelay, FuzzI2C},
};

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let fuzz_i2c = FuzzI2C::new(data);

    let mut bmp180 = UninitBMP180::builder(fuzz_i2c, FuzzDelay {})
        .build()
        .initialize()
        .expect("Could not initialize BMP180");

    bmp180.update().ok();
});
