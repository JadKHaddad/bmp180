#![no_main]

use bmp180::{
    fuzz::{FuzzDelay, FuzzI2C},
    BlockingBMP180, BlockingInitBMP180, UninitBMP180,
};

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let fuzz_i2c = FuzzI2C::new(data);

    let mut bmp180 = UninitBMP180::builder(fuzz_i2c, FuzzDelay {})
        .build()
        .initialize()
        .unwrap();

    bmp180.update().unwrap();
});
