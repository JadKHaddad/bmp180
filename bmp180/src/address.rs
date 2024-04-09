//! BMP180 device I2C address.

/// BMP180 device I2C address.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub enum Address {
    /// Default I2C address.
    #[default]
    Default = 0x77,
    /// User-defined I2C address.
    Other(u8),
}

impl From<Address> for u8 {
    fn from(address: Address) -> u8 {
        match address {
            Address::Default => 0x77,
            Address::Other(addr) => addr,
        }
    }
}

impl From<u8> for Address {
    fn from(addr: u8) -> Address {
        match addr {
            0x77 => Address::Default,
            addr => Address::Other(addr),
        }
    }
}
