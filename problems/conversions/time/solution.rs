//! # Time Unit Conversion
//!
//! A unit of time is any particular time interval, used as a standard way of
//! measuring or expressing duration. The base unit of time in the International
//! System of Units (SI), and by extension most of the Western world, is the second,
//! defined as about 9 billion oscillations of the caesium atom.
//!
//! More information: <https://en.wikipedia.org/wiki/Unit_of_time>

/// Supported time units for conversion
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl TimeUnit {
    /// Returns the value of the time unit in seconds
    fn to_seconds(self) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (to_seconds)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement to_seconds");
}

    /// Parse a string into a TimeUnit (case-insensitive)
    fn from_str(s: &str) -> Result<Self, String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from_str)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from_str");
}
}

/// Convert time from one unit to another
///
/// # Arguments
///
/// * `time_value` - The time value to convert (must be non-negative)
/// * `unit_from` - The source unit (case-insensitive)
/// * `unit_to` - The target unit (case-insensitive)
///
/// # Returns
///
/// Returns the converted time value rounded to 3 decimal places
///
/// # Errors
///
/// Returns an error if:
/// * `time_value` is negative or not a valid number
/// * `unit_from` or `unit_to` is not a valid time unit
pub fn convert_time(time_value: f64, unit_from: &str, unit_to: &str) -> Result<f64, String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (convert_time)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement convert_time");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_to_hours() {
        assert_eq!(convert_time(3600.0, "seconds", "hours").unwrap(), 1.0);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(convert_time(3500.0, "Seconds", "Hours").unwrap(), 0.972);
        assert_eq!(convert_time(1.0, "DaYs", "hours").unwrap(), 24.0);
        assert_eq!(convert_time(120.0, "minutes", "SeCoNdS").unwrap(), 7200.0);
    }

    #[test]
    fn test_weeks_to_days() {
        assert_eq!(convert_time(2.0, "WEEKS", "days").unwrap(), 14.0);
    }

    #[test]
    fn test_hours_to_minutes() {
        assert_eq!(convert_time(0.5, "hours", "MINUTES").unwrap(), 30.0);
    }

    #[test]
    fn test_days_to_months() {
        assert_eq!(convert_time(360.0, "days", "months").unwrap(), 11.828);
    }

    #[test]
    fn test_months_to_years() {
        assert_eq!(convert_time(360.0, "months", "years").unwrap(), 30.0);
    }

    #[test]
    fn test_years_to_seconds() {
        assert_eq!(convert_time(1.0, "years", "seconds").unwrap(), 31_557_600.0);
    }

    #[test]
    fn test_negative_value() {
        let result = convert_time(-3600.0, "seconds", "hours");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "'time_value' must be a non-negative number."
        );
    }

    #[test]
    fn test_invalid_from_unit() {
        let result = convert_time(1.0, "cool", "century");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid unit cool"));
    }

    #[test]
    fn test_invalid_to_unit() {
        let result = convert_time(1.0, "seconds", "hot");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid unit hot"));
    }

    #[test]
    fn test_zero_value() {
        assert_eq!(convert_time(0.0, "hours", "minutes").unwrap(), 0.0);
    }

    #[test]
    fn test_same_unit() {
        assert_eq!(convert_time(100.0, "seconds", "seconds").unwrap(), 100.0);
    }
}
