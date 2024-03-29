//! Constant addresses and values for the BMP180 sensor.

pub const BMP180_I2C_ADDR: u8 = 0x77;

pub(crate) const BMP180_ID: u8 = 0x55;

pub(crate) const BMP180_REGISTER_CHIPID: u8 = 0xD0;

pub(crate) const BMP180_CAL_AC1: u8 = 0xAA;

pub(crate) const BMP180_CONTROL: u8 = 0xF4;
pub(crate) const BMP180_TEMPDATA: u8 = 0xF6;
pub(crate) const BMP180_PRESSUREDATA: u8 = 0xF6;
pub(crate) const BMP180_READTEMPCMD: u8 = 0x2E;
pub(crate) const BMP180_READPRESSURECMD: u8 = 0x34;
