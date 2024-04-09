//! Valid BMP180 device ID.

/// Valid BMP180 device ID.
#[repr(u8)]
pub enum Id {
    Valid = 0x55,
}

impl Id {
    pub fn is_valid(id: u8) -> bool {
        id == Id::Valid as u8
    }
}
