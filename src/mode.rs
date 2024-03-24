pub enum Mode {
    UltraLowPower,
    Standard,
    HighResolution,
    UltraHighResolution,
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
