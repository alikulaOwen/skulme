//! Convert between different units of temperature
//!
//! Supports conversions between 8 temperature scales using Kelvin as an intermediary:
//! - Kelvin (K) - SI base unit, absolute scale
//! - Celsius (°C) - Standard metric scale
//! - Fahrenheit (°F) - Imperial scale
//! - Rankine (°R) - Absolute Fahrenheit scale
//! - Delisle (°De) - Historical inverted scale (higher values = colder)
//! - Newton (°N) - Historical scale by Isaac Newton
//! - Réaumur (°Ré) - Historical European scale
//! - Rømer (°Rø) - Historical Danish scale

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Kelvin,
    Celsius,
    Fahrenheit,
    Rankine,
    Delisle,
    Newton,
    Reaumur,
    Romer,
}

impl TemperatureUnit {
    fn to_kelvin(self, value: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (to_kelvin)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement to_kelvin");
}

    fn kelvin_to_unit(self, kelvin: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (kelvin_to_unit)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement kelvin_to_unit");
}
}

pub fn convert_temperature(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (convert_temperature)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement convert_temperature");
}


#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_celsius_conversions() {
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit),
            32.0
        ));
        assert!(approx_eq(
            convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit),
            212.0
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Kelvin),
            273.15
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Rankine),
            491.67
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Delisle),
            150.0
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Newton),
            0.0
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Reaumur),
            0.0
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Romer),
            7.5
        ));
    }

    #[test]
    fn test_fahrenheit_conversions() {
        assert!(approx_eq(
            convert_temperature(32.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius),
            0.0
        ));
        assert!(approx_eq(
            convert_temperature(212.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius),
            100.0
        ));
        assert!(approx_eq(
            convert_temperature(32.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin),
            273.15
        ));
        assert!(approx_eq(
            convert_temperature(32.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Rankine),
            491.67
        ));
    }

    #[test]
    fn test_kelvin_conversions() {
        assert!(approx_eq(
            convert_temperature(273.15, TemperatureUnit::Kelvin, TemperatureUnit::Celsius),
            0.0
        ));
        assert!(approx_eq(
            convert_temperature(273.15, TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit),
            32.0
        ));
        assert!(approx_eq(
            convert_temperature(273.15, TemperatureUnit::Kelvin, TemperatureUnit::Rankine),
            491.67
        ));
    }

    #[test]
    fn test_round_trip_conversions() {
        let temp = 25.0;
        let units = [
            TemperatureUnit::Celsius,
            TemperatureUnit::Fahrenheit,
            TemperatureUnit::Kelvin,
            TemperatureUnit::Rankine,
            TemperatureUnit::Delisle,
            TemperatureUnit::Newton,
            TemperatureUnit::Reaumur,
            TemperatureUnit::Romer,
        ];

        for from_unit in units.iter() {
            for to_unit in units.iter() {
                let converted = convert_temperature(temp, *from_unit, *to_unit);
                let back = convert_temperature(converted, *to_unit, *from_unit);
                assert!(
                    approx_eq(back, temp),
                    "Round trip failed: {from_unit:?} -> {to_unit:?} -> {from_unit:?}: {back} != {temp}"
                );
            }
        }
    }

    #[test]
    fn test_special_temperatures() {
        // Absolute zero
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Kelvin, TemperatureUnit::Celsius),
            -273.15
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit),
            -459.67
        ));

        // Water freezing point
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit),
            32.0
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Kelvin),
            273.15
        ));

        // Water boiling point
        assert!(approx_eq(
            convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit),
            212.0
        ));
        assert!(approx_eq(
            convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Kelvin),
            373.15
        ));

        // Celsius equals Fahrenheit
        assert!(approx_eq(
            convert_temperature(-40.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit),
            -40.0
        ));
    }

    #[test]
    fn test_historical_scales() {
        // Delisle (inverted scale)
        assert!(approx_eq(
            convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Delisle),
            0.0
        ));
        assert!(approx_eq(
            convert_temperature(0.0, TemperatureUnit::Delisle, TemperatureUnit::Celsius),
            100.0
        ));

        // Newton scale
        assert!(approx_eq(
            convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Newton),
            33.0
        ));
        assert!(approx_eq(
            convert_temperature(33.0, TemperatureUnit::Newton, TemperatureUnit::Celsius),
            100.0
        ));

        // Rømer scale
        assert!(approx_eq(
            convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Romer),
            60.0
        ));
        assert!(approx_eq(
            convert_temperature(60.0, TemperatureUnit::Romer, TemperatureUnit::Celsius),
            100.0
        ));
    }

    #[test]
    fn test_same_unit_conversion() {
        let temp = 42.0;
        for unit in [
            TemperatureUnit::Celsius,
            TemperatureUnit::Fahrenheit,
            TemperatureUnit::Kelvin,
            TemperatureUnit::Rankine,
            TemperatureUnit::Delisle,
            TemperatureUnit::Newton,
            TemperatureUnit::Reaumur,
            TemperatureUnit::Romer,
        ] {
            assert!(approx_eq(convert_temperature(temp, unit, unit), temp));
        }
    }
}
