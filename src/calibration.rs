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
        let ac1 = slice[1] as i16 | ((slice[0] as i16) << 8);
        let ac2 = slice[3] as i16 | ((slice[2] as i16) << 8);
        let ac3 = slice[5] as i16 | ((slice[4] as i16) << 8);
        let ac4 = slice[7] as u16 | ((slice[6] as u16) << 8);
        let ac5 = slice[9] as u16 | ((slice[8] as u16) << 8);
        let ac6 = slice[11] as u16 | ((slice[10] as u16) << 8);

        let b1 = slice[13] as i16 | ((slice[12] as i16) << 8);
        let b2 = slice[15] as i16 | ((slice[14] as i16) << 8);

        let mb = slice[17] as i16 | ((slice[16] as i16) << 8);
        let mc = slice[19] as i16 | ((slice[18] as i16) << 8);
        let md = slice[21] as i16 | ((slice[20] as i16) << 8);

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
