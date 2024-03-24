pub const BMP180_I2CADDR: u8 = 0x77;

pub const BMP180_ID: u8 = 0x55;

pub const BMP180_REGISTER_CHIPID: u8 = 0xD0;

pub const BMP180_CAL_AC1: u8 = 0xAA;
pub const BMP180_CAL_AC2: u8 = 0xAC;
pub const BMP180_CAL_AC3: u8 = 0xAE;
pub const BMP180_CAL_AC4: u8 = 0xB0;
pub const BMP180_CAL_AC5: u8 = 0xB2;
pub const BMP180_CAL_AC6: u8 = 0xB4;
pub const BMP180_CAL_B1: u8 = 0xB6;
pub const BMP180_CAL_B2: u8 = 0xB8;
pub const BMP180_CAL_MB: u8 = 0xBA;
pub const BMP180_CAL_MC: u8 = 0xBC;
pub const BMP180_CAL_MD: u8 = 0xBE;

pub const BMP180_CONTROL: u8 = 0xF4;
pub const BMP180_TEMPDATA: u8 = 0xF6;
pub const BMP180_PRESSUREDATA: u8 = 0xF6;
pub const BMP180_READTEMPCMD: u8 = 0x2E;
pub const BMP180_READPRESSURECMD: u8 = 0x34;
