//! Valid `BMP180` device ID.

/// Valid `BMP180` device ID.
#[repr(u8)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Id {
    /// Valid device ID.
    Valid = 0x55,
}

impl Id {
    /// Check if the device ID is valid.
    pub fn is_valid(id: u8) -> bool {
        id == Id::Valid as u8
    }
}
