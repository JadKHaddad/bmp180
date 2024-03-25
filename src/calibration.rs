#[derive(Debug, Default, Clone)]
pub struct Calibration {
    pub ac1: i16,
    pub ac2: i16,
    pub ac3: i16,
    pub ac4: u16,
    pub ac5: u16,
    pub ac6: u16,

    pub b1: i16,
    pub b2: i16,

    pub mb: i16,
    pub mc: i16,
    pub md: i16,
}

impl Calibration {
    pub fn from_slice(slice: &[u8; 22]) -> Self {
        let ac1 = i16::from_be_bytes([slice[0], slice[1]]);
        let ac2 = i16::from_be_bytes([slice[2], slice[3]]);
        let ac3 = i16::from_be_bytes([slice[4], slice[5]]);
        let ac4 = u16::from_be_bytes([slice[6], slice[7]]);
        let ac5 = u16::from_be_bytes([slice[8], slice[9]]);
        let ac6 = u16::from_be_bytes([slice[10], slice[11]]);

        let b1 = i16::from_be_bytes([slice[12], slice[13]]);
        let b2 = i16::from_be_bytes([slice[14], slice[15]]);

        let mb = i16::from_be_bytes([slice[16], slice[17]]);
        let mc = i16::from_be_bytes([slice[18], slice[19]]);
        let md = i16::from_be_bytes([slice[20], slice[21]]);

        Self {
            ac1,
            ac2,
            ac3,
            ac4,
            ac5,
            ac6,
            b1,
            b2,
            mb,
            mc,
            md,
        }
    }
}
