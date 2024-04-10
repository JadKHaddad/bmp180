//! Register addresses for the `BMP180` device.

/// Register addresses for the `BMP180` device.
#[repr(u8)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Register {
    ChipId = 0xD0,
    CalibrationAc1 = 0xAA,
    Control = 0xF4,
    TempPressureData = 0xF6,
    ReadTempCmd = 0x2E,
    ReadPressureCmd = 0x34,
}
