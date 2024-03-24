#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    UltraLowPower = 0,
    Standard = 1,
    HighResolution = 2,
    UltraHighResolution = 3,
}

impl Mode {
    pub fn delay_ms(&self) -> u32 {
        match self {
            Mode::UltraLowPower => 5,
            Mode::Standard => 8,
            Mode::HighResolution => 14,
            Mode::UltraHighResolution => 26,
        }
    }
}
