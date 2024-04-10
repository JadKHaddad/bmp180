//! Operating mode.

/// Operating mode.
#[repr(u8)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Mode {
    /// Ultra low power mode.
    UltraLowPower = 0,
    /// Standard mode.
    #[default]
    Standard = 1,
    /// High resolution mode.
    HighResolution = 2,
    /// Ultra high resolution mode.
    UltraHighResolution = 3,
}

impl Mode {
    /// Delay in milliseconds for the given mode.
    pub fn delay_ms(&self) -> u32 {
        match self {
            Mode::UltraLowPower => 5,
            Mode::Standard => 8,
            Mode::HighResolution => 14,
            Mode::UltraHighResolution => 26,
        }
    }
}
