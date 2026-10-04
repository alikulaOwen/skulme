//! Convert speed units
//!
//! References:
//! - <https://en.wikipedia.org/wiki/Kilometres_per_hour>
//! - <https://en.wikipedia.org/wiki/Miles_per_hour>
//! - <https://en.wikipedia.org/wiki/Knot_(unit)>
//! - <https://en.wikipedia.org/wiki/Metre_per_second>
//! - <https://en.wikipedia.org/wiki/Foot_per_second>
//! - <https://en.wikipedia.org/wiki/Mach_number>
//! - <https://en.wikipedia.org/wiki/Speed_of_light>

use std::fmt;

/// Supported speed units
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeedUnit {
    /// Kilometers per hour (km/h)
    KilometersPerHour,
    /// Meters per second (m/s) - SI derived unit
    MetersPerSecond,
    /// Miles per hour (mph)
    MilesPerHour,
    /// Nautical miles per hour (knot)
    Knot,
    /// Feet per second (fps or ft/s)
    FeetPerSecond,
    /// Mach number (dimensionless) - speed divided by speed of sound at sea level (340.3 m/s)
    Mach,
    /// Speed of light (c) - speed divided by speed of light in vacuum (299,792,458 m/s)
    SpeedOfLight,
}

impl fmt::Display for SpeedUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fmt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fmt");
}
}

impl SpeedUnit {
    /// Get the conversion factor to km/h
    fn as_kmh_multiplier(self) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (as_kmh_multiplier)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement as_kmh_multiplier");
}

    /// Get the conversion factor from km/h to this unit
    fn kmh_multiplier(self) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (kmh_multiplier)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement kmh_multiplier");
}
}

/// Convert speed from one unit to another
///
/// # Arguments
///
/// * `speed` - The speed value to convert
/// * `from` - The unit to convert from
/// * `to` - The unit to convert to
///
/// # Returns
///
/// The converted speed value rounded to 3 decimal places
pub fn convert_speed(speed: f64, from: SpeedUnit, to: SpeedUnit) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (convert_speed)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement convert_speed");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_conversion() {
        assert_eq!(
            convert_speed(
                100.0,
                SpeedUnit::KilometersPerHour,
                SpeedUnit::MetersPerSecond
            ),
            27.778
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::KilometersPerHour, SpeedUnit::MilesPerHour),
            62.137
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::KilometersPerHour, SpeedUnit::Knot),
            53.996
        );
        assert_eq!(
            convert_speed(
                100.0,
                SpeedUnit::MetersPerSecond,
                SpeedUnit::KilometersPerHour
            ),
            360.0
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::MetersPerSecond, SpeedUnit::MilesPerHour),
            223.694
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::MetersPerSecond, SpeedUnit::Knot),
            194.384
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::MilesPerHour, SpeedUnit::KilometersPerHour),
            160.934
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::MilesPerHour, SpeedUnit::MetersPerSecond),
            44.704
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::MilesPerHour, SpeedUnit::Knot),
            86.898
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::Knot, SpeedUnit::KilometersPerHour),
            185.2
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::Knot, SpeedUnit::MetersPerSecond),
            51.444
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::Knot, SpeedUnit::MilesPerHour),
            115.078
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::FeetPerSecond, SpeedUnit::MetersPerSecond),
            30.48
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::MetersPerSecond, SpeedUnit::FeetPerSecond),
            328.084
        );
        assert_eq!(
            convert_speed(
                100.0,
                SpeedUnit::FeetPerSecond,
                SpeedUnit::KilometersPerHour
            ),
            109.728
        );
        assert_eq!(
            convert_speed(100.0, SpeedUnit::FeetPerSecond, SpeedUnit::MilesPerHour),
            68.182
        );
        assert_eq!(
            convert_speed(1.0, SpeedUnit::Mach, SpeedUnit::KilometersPerHour),
            1225.08
        );
        assert_eq!(
            convert_speed(1.0, SpeedUnit::Mach, SpeedUnit::MetersPerSecond),
            340.3
        );
        assert_eq!(
            convert_speed(1000.0, SpeedUnit::KilometersPerHour, SpeedUnit::Mach),
            0.816
        );
        assert_eq!(
            convert_speed(2.0, SpeedUnit::Mach, SpeedUnit::KilometersPerHour),
            2450.16
        );
        assert_eq!(
            convert_speed(1.0, SpeedUnit::SpeedOfLight, SpeedUnit::MetersPerSecond),
            299792458.24
        );
        assert_eq!(
            convert_speed(1.0, SpeedUnit::SpeedOfLight, SpeedUnit::KilometersPerHour),
            1079252848.8
        );
        assert_eq!(
            convert_speed(
                299792458.0,
                SpeedUnit::MetersPerSecond,
                SpeedUnit::SpeedOfLight
            ),
            1.0
        );
        assert_eq!(
            convert_speed(0.1, SpeedUnit::SpeedOfLight, SpeedUnit::MetersPerSecond),
            29979245.824
        );
        assert_eq!(
            convert_speed(
                100.0,
                SpeedUnit::KilometersPerHour,
                SpeedUnit::KilometersPerHour
            ),
            100.0
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(SpeedUnit::KilometersPerHour.to_string(), "km/h");
        assert_eq!(SpeedUnit::MetersPerSecond.to_string(), "m/s");
        assert_eq!(SpeedUnit::MilesPerHour.to_string(), "mph");
        assert_eq!(SpeedUnit::Knot.to_string(), "knot");
        assert_eq!(SpeedUnit::FeetPerSecond.to_string(), "ft/s");
        assert_eq!(SpeedUnit::Mach.to_string(), "Mach");
        assert_eq!(SpeedUnit::SpeedOfLight.to_string(), "c");
    }
}
