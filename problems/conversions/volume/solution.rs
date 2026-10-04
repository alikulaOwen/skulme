//! Convert between different units of volume
//!
//! Supports conversions between various volume units using cubic meters as an intermediary:
//! - Metric: cubic meter, cubic centimeter, cubic millimeter, liter, milliliter, centiliter, deciliter, kiloliter, hectoliter
//! - Imperial: gallon, quart, pint, fluid ounce, tablespoon, teaspoon, barrel
//! - US Customary: gallon, quart (liquid/dry), pint (liquid/dry), cup, fluid ounce, tablespoon, teaspoon, barrel (oil/liquid)
//! - Cubic: cubic yard, cubic foot, cubic inch
//! - Other: board foot, cord, metric cup, Canadian tablespoon/teaspoon

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeUnit {
    // Metric units
    CubicMeter,
    CubicCentimeter,
    CubicMillimeter,
    Liter,
    Milliliter,
    Centiliter,
    Deciliter,
    Kiloliter,
    Hectoliter,

    // Imperial units
    GallonImperial,
    QuartImperial,
    PintImperial,
    FluidOunceImperial,
    TablespoonImperial,
    TeaspoonImperial,
    BarrelImperial,

    // US customary units (liquid)
    GallonUs,
    QuartUsLiquid,
    PintUsLiquid,
    CupUs,
    FluidOunceUs,
    TablespoonUs,
    TeaspoonUs,

    // US customary units (dry)
    QuartUsDry,
    PintUsDry,

    // US barrels
    BarrelUsOil,
    BarrelUsLiquid,

    // Cubic units
    CubicYard,
    CubicFoot,
    CubicInch,

    // Other units
    BoardFoot,
    Cord,
    CupMetric,
    TablespoonCanadian,
    TeaspoonCanadian,
}

impl VolumeUnit {
    /// Convert from this unit to cubic meters
    fn to_cubic_meters(self, value: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (to_cubic_meters)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement to_cubic_meters");
}

    /// Convert from cubic meters to this unit
    fn cubic_meters_to_unit(self, cubic_meters: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cubic_meters_to_unit)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cubic_meters_to_unit");
}
}

/// Convert a volume value from one unit to another
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::conversions::{convert_volume, VolumeUnit};
///
/// let liters = convert_volume(1.0, VolumeUnit::CubicMeter, VolumeUnit::Liter);
/// assert_eq!(liters, 1000.0);
///
/// let gallons = convert_volume(1.0, VolumeUnit::Liter, VolumeUnit::GallonUs);
/// assert!((gallons - 0.264172).abs() < 0.0001);
/// ```
pub fn convert_volume(value: f64, from: VolumeUnit, to: VolumeUnit) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (convert_volume)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement convert_volume");
}


#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
        (a - b).abs() < tolerance
    }

    #[test]
    fn test_volume_conversions() {
        // Metric conversions
        assert_eq!(
            convert_volume(4.0, VolumeUnit::CubicMeter, VolumeUnit::Liter),
            4000.0
        );
        assert_eq!(
            convert_volume(1000.0, VolumeUnit::Milliliter, VolumeUnit::Liter),
            1.0
        );
        assert_eq!(
            convert_volume(1.0, VolumeUnit::CubicCentimeter, VolumeUnit::Milliliter),
            1.0
        );
        assert_eq!(
            convert_volume(1.0, VolumeUnit::Kiloliter, VolumeUnit::CubicMeter),
            1.0
        );
        assert_eq!(
            convert_volume(100.0, VolumeUnit::Centiliter, VolumeUnit::Liter),
            1.0
        );
        assert_eq!(
            convert_volume(10.0, VolumeUnit::Deciliter, VolumeUnit::Liter),
            1.0
        );
        assert_eq!(
            convert_volume(10.0, VolumeUnit::Hectoliter, VolumeUnit::CubicMeter),
            1.0
        );

        // Imperial conversions
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::GallonImperial, VolumeUnit::Liter),
            4.54609,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::PintImperial, VolumeUnit::Milliliter),
            568.261,
            0.1
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::FluidOunceImperial, VolumeUnit::Milliliter),
            28.413,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(4.0, VolumeUnit::QuartImperial, VolumeUnit::GallonImperial),
            1.0,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(
                20.0,
                VolumeUnit::FluidOunceImperial,
                VolumeUnit::PintImperial
            ),
            1.0,
            0.001
        ));

        // US customary liquid conversions
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::GallonUs, VolumeUnit::Liter),
            3.785,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::PintUsLiquid, VolumeUnit::Milliliter),
            473.176,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::CupUs, VolumeUnit::Milliliter),
            236.588,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(16.0, VolumeUnit::TablespoonUs, VolumeUnit::CupUs),
            1.0,
            EPSILON
        ));
        assert!(approx_eq(
            convert_volume(3.0, VolumeUnit::TeaspoonUs, VolumeUnit::TablespoonUs),
            1.0,
            EPSILON
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::FluidOunceUs, VolumeUnit::Milliliter),
            29.574,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(4.0, VolumeUnit::QuartUsLiquid, VolumeUnit::GallonUs),
            1.0,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(2.0, VolumeUnit::PintUsLiquid, VolumeUnit::QuartUsLiquid),
            1.0,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(2.0, VolumeUnit::CupUs, VolumeUnit::PintUsLiquid),
            1.0,
            EPSILON
        ));
        assert!(approx_eq(
            convert_volume(8.0, VolumeUnit::FluidOunceUs, VolumeUnit::CupUs),
            1.0,
            0.001
        ));

        // US dry conversions
        assert!(approx_eq(
            convert_volume(2.0, VolumeUnit::PintUsDry, VolumeUnit::QuartUsDry),
            1.0,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::QuartUsDry, VolumeUnit::Liter),
            1.101,
            0.001
        ));

        // Cubic units
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::CubicFoot, VolumeUnit::Liter),
            28.317,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::CubicYard, VolumeUnit::CubicMeter),
            0.764555,
            0.0001
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::CubicInch, VolumeUnit::Milliliter),
            16.387,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(27.0, VolumeUnit::CubicFoot, VolumeUnit::CubicYard),
            1.0,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(1728.0, VolumeUnit::CubicInch, VolumeUnit::CubicFoot),
            1.0,
            0.1
        ));

        // Mixed imperial/US conversions
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::GallonImperial, VolumeUnit::GallonUs),
            1.20095,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::PintImperial, VolumeUnit::PintUsLiquid),
            1.20095,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(
                1.0,
                VolumeUnit::FluidOunceImperial,
                VolumeUnit::FluidOunceUs
            ),
            0.96076,
            0.001
        ));

        // Barrel conversions
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::BarrelUsOil, VolumeUnit::Liter),
            158.987,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::BarrelUsOil, VolumeUnit::GallonUs),
            42.0,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::BarrelUsLiquid, VolumeUnit::GallonUs),
            31.5,
            0.1
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::BarrelImperial, VolumeUnit::GallonImperial),
            36.0,
            0.1
        ));

        // Other units
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::Cord, VolumeUnit::CubicMeter),
            3.62456,
            0.001
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::BoardFoot, VolumeUnit::CubicFoot),
            0.08333,
            0.001
        ));

        // Metric cup
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::CupMetric, VolumeUnit::Milliliter),
            250.0,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::CupUs, VolumeUnit::CupMetric),
            0.9464,
            0.001
        ));

        // Canadian units
        assert!(approx_eq(
            convert_volume(1.0, VolumeUnit::TablespoonCanadian, VolumeUnit::Milliliter),
            14.207,
            0.01
        ));
        assert!(approx_eq(
            convert_volume(
                3.0,
                VolumeUnit::TeaspoonCanadian,
                VolumeUnit::TablespoonCanadian
            ),
            1.0,
            0.001
        ));

        // Edge cases - converting to same unit
        assert!(approx_eq(
            convert_volume(5.0, VolumeUnit::Liter, VolumeUnit::Liter),
            5.0,
            EPSILON
        ));
        assert!(approx_eq(
            convert_volume(10.0, VolumeUnit::GallonUs, VolumeUnit::GallonUs),
            10.0,
            EPSILON
        ));

        // Large values
        assert!(approx_eq(
            convert_volume(1000.0, VolumeUnit::CubicMeter, VolumeUnit::Liter),
            1_000_000.0,
            0.1
        ));

        // Small values
        assert!(approx_eq(
            convert_volume(0.001, VolumeUnit::Milliliter, VolumeUnit::CubicMeter),
            1e-9,
            1e-12
        ));
    }

    #[test]
    fn test_round_trip_conversions() {
        let volume = 5.0;
        let units = [
            VolumeUnit::CubicMeter,
            VolumeUnit::Liter,
            VolumeUnit::Milliliter,
            VolumeUnit::GallonUs,
            VolumeUnit::GallonImperial,
            VolumeUnit::CupUs,
            VolumeUnit::CubicFoot,
            VolumeUnit::CubicYard,
        ];

        for from_unit in units.iter() {
            for to_unit in units.iter() {
                let converted = convert_volume(volume, *from_unit, *to_unit);
                let back = convert_volume(converted, *to_unit, *from_unit);
                assert!(
                    approx_eq(back, volume, EPSILON * volume.abs().max(1.0)),
                    "Round trip failed: {from_unit:?} -> {to_unit:?} -> {from_unit:?}: {back} != {volume}"
                );
            }
        }
    }

    #[test]
    fn test_same_unit_conversion() {
        let volume = 42.0;
        for unit in [
            VolumeUnit::CubicMeter,
            VolumeUnit::Liter,
            VolumeUnit::GallonUs,
            VolumeUnit::GallonImperial,
            VolumeUnit::CubicFoot,
            VolumeUnit::CupUs,
        ] {
            assert!(approx_eq(
                convert_volume(volume, unit, unit),
                volume,
                EPSILON
            ));
        }
    }
}
