//! Module for converting between RGB and HSV color representations
//!
//! The RGB color model is an additive color model in which red, green, and blue light
//! are added together in various ways to reproduce a broad array of colors. The name
//! of the model comes from the initials of the three additive primary colors, red,
//! green, and blue. Meanwhile, the HSV representation models how colors appear under
//! light. In it, colors are represented using three components: hue, saturation and
//! (brightness-)value.
//!
//! References:
//! - https://en.wikipedia.org/wiki/RGB_color_model
//! - https://en.wikipedia.org/wiki/HSL_and_HSV
//! - https://www.rapidtables.com/convert/color/hsv-to-rgb.html

/// Errors that can occur during color conversion
#[derive(Debug, PartialEq)]
pub enum ColorError {
    /// Hue value is out of valid range [0, 360]
    InvalidHue(f64),
    /// Saturation value is out of valid range [0, 1]
    InvalidSaturation(f64),
    /// Value component is out of valid range [0, 1]
    InvalidValue(f64),
}

impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fmt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fmt");
}
}

impl std::error::Error for ColorError {}

/// RGB color representation with red, green, and blue components (0-255)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    /// Create a new RGB color
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

/// HSV color representation with hue (0-360), saturation (0-1), and value (0-1)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

impl Hsv {
    /// Create a new HSV color with validation
    pub fn new(hue: f64, saturation: f64, value: f64) -> Result<Self, ColorError> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    /// Check if two HSV colors are approximately equal
    ///
    /// Uses tolerance values:
    /// - Hue: 0.2 degrees
    /// - Saturation: 0.002
    /// - Value: 0.002
    pub fn approximately_equal(&self, other: &Hsv) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (approximately_equal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement approximately_equal");
}
}

/// Convert HSV color representation to RGB
///
/// Converts from HSV (Hue, Saturation, Value) to RGB (Red, Green, Blue).
///
/// # Arguments
///
/// * `hue` - Hue value in degrees (0-360)
/// * `saturation` - Saturation value (0-1)
/// * `value` - Value/brightness (0-1)
///
/// # Returns
///
/// * `Ok(Rgb)` - RGB color with components in range 0-255
/// * `Err(ColorError)` - If any input is out of valid range
pub fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> Result<Rgb, ColorError> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (hsv_to_rgb)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement hsv_to_rgb");
}

/// Convert RGB color representation to HSV
///
/// Converts from RGB (Red, Green, Blue) to HSV (Hue, Saturation, Value).
///
/// # Arguments
///
/// * `red` - Red component (0-255)
/// * `green` - Green component (0-255)
/// * `blue` - Blue component (0-255)
///
/// # Returns
///
/// * `Ok(Hsv)` - HSV color with hue in [0, 360] and saturation/value in [0, 1]
pub fn rgb_to_hsv(red: u8, green: u8, blue: u8) -> Result<Hsv, ColorError> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rgb_to_hsv)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rgb_to_hsv");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_to_rgb_basic_colors() {
        // Black
        assert_eq!(hsv_to_rgb(0.0, 0.0, 0.0).unwrap(), Rgb::new(0, 0, 0));

        // White
        assert_eq!(hsv_to_rgb(0.0, 0.0, 1.0).unwrap(), Rgb::new(255, 255, 255));

        // Red
        assert_eq!(hsv_to_rgb(0.0, 1.0, 1.0).unwrap(), Rgb::new(255, 0, 0));

        // Yellow
        assert_eq!(hsv_to_rgb(60.0, 1.0, 1.0).unwrap(), Rgb::new(255, 255, 0));

        // Green
        assert_eq!(hsv_to_rgb(120.0, 1.0, 1.0).unwrap(), Rgb::new(0, 255, 0));

        // Blue
        assert_eq!(hsv_to_rgb(240.0, 1.0, 1.0).unwrap(), Rgb::new(0, 0, 255));

        // Magenta
        assert_eq!(hsv_to_rgb(300.0, 1.0, 1.0).unwrap(), Rgb::new(255, 0, 255));
    }

    #[test]
    fn test_hsv_to_rgb_intermediate_colors() {
        assert_eq!(hsv_to_rgb(180.0, 0.5, 0.5).unwrap(), Rgb::new(64, 128, 128));
        assert_eq!(
            hsv_to_rgb(234.0, 0.14, 0.88).unwrap(),
            Rgb::new(193, 196, 224)
        );
        assert_eq!(hsv_to_rgb(330.0, 0.75, 0.5).unwrap(), Rgb::new(128, 32, 80));
    }

    #[test]
    fn test_hsv_to_rgb_invalid_hue() {
        assert_eq!(
            hsv_to_rgb(-1.0, 0.5, 0.5),
            Err(ColorError::InvalidHue(-1.0))
        );
        assert_eq!(
            hsv_to_rgb(361.0, 0.5, 0.5),
            Err(ColorError::InvalidHue(361.0))
        );
    }

    #[test]
    fn test_hsv_to_rgb_invalid_saturation() {
        assert_eq!(
            hsv_to_rgb(180.0, -0.1, 0.5),
            Err(ColorError::InvalidSaturation(-0.1))
        );
        assert_eq!(
            hsv_to_rgb(180.0, 1.1, 0.5),
            Err(ColorError::InvalidSaturation(1.1))
        );
    }

    #[test]
    fn test_hsv_to_rgb_invalid_value() {
        assert_eq!(
            hsv_to_rgb(180.0, 0.5, -0.1),
            Err(ColorError::InvalidValue(-0.1))
        );
        assert_eq!(
            hsv_to_rgb(180.0, 0.5, 1.1),
            Err(ColorError::InvalidValue(1.1))
        );
    }

    #[test]
    fn test_rgb_to_hsv_basic_colors() {
        // Black
        let hsv = rgb_to_hsv(0, 0, 0).unwrap();
        assert!(Hsv::new(0.0, 0.0, 0.0).unwrap().approximately_equal(&hsv));

        // White
        let hsv = rgb_to_hsv(255, 255, 255).unwrap();
        assert!(Hsv::new(0.0, 0.0, 1.0).unwrap().approximately_equal(&hsv));

        // Red
        let hsv = rgb_to_hsv(255, 0, 0).unwrap();
        assert!(Hsv::new(0.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));

        // Yellow
        let hsv = rgb_to_hsv(255, 255, 0).unwrap();
        assert!(Hsv::new(60.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));

        // Green
        let hsv = rgb_to_hsv(0, 255, 0).unwrap();
        assert!(Hsv::new(120.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));

        // Blue
        let hsv = rgb_to_hsv(0, 0, 255).unwrap();
        assert!(Hsv::new(240.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));

        // Magenta
        let hsv = rgb_to_hsv(255, 0, 255).unwrap();
        assert!(Hsv::new(300.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));
    }

    #[test]
    fn test_rgb_to_hsv_intermediate_colors() {
        let hsv = rgb_to_hsv(64, 128, 128).unwrap();
        assert!(Hsv::new(180.0, 0.5, 0.5).unwrap().approximately_equal(&hsv));

        let hsv = rgb_to_hsv(193, 196, 224).unwrap();
        assert!(Hsv::new(234.0, 0.14, 0.88)
            .unwrap()
            .approximately_equal(&hsv));

        let hsv = rgb_to_hsv(128, 32, 80).unwrap();
        assert!(Hsv::new(330.0, 0.75, 0.5)
            .unwrap()
            .approximately_equal(&hsv));
    }

    #[test]
    fn test_round_trip_conversion() {
        let test_cases = vec![
            (0.0, 0.0, 0.0),
            (0.0, 0.0, 1.0),
            (0.0, 1.0, 1.0),
            (60.0, 1.0, 1.0),
            (120.0, 1.0, 1.0),
            (240.0, 1.0, 1.0),
            (300.0, 1.0, 1.0),
            (180.0, 0.5, 0.5),
            (234.0, 0.14, 0.88),
            (330.0, 0.75, 0.5),
        ];

        for (hue, sat, val) in test_cases {
            let original_hsv = Hsv::new(hue, sat, val).unwrap();
            let rgb = hsv_to_rgb(hue, sat, val).unwrap();
            let converted_hsv = rgb_to_hsv(rgb.red, rgb.green, rgb.blue).unwrap();
            assert!(
                original_hsv.approximately_equal(&converted_hsv),
                "Round trip failed for HSV({hue}, {sat}, {val})"
            );
        }
    }

    #[test]
    fn test_approximately_equal_hsv() {
        let hsv1 = Hsv::new(0.0, 0.0, 0.0).unwrap();
        let hsv2 = Hsv::new(0.0, 0.0, 0.0).unwrap();
        assert!(hsv1.approximately_equal(&hsv2));

        let hsv1 = Hsv::new(180.0, 0.5, 0.3).unwrap();
        let hsv2 = Hsv::new(179.9999, 0.500001, 0.30001).unwrap();
        assert!(hsv1.approximately_equal(&hsv2));

        let hsv1 = Hsv::new(0.0, 0.0, 0.0).unwrap();
        let hsv2 = Hsv::new(1.0, 0.0, 0.0).unwrap();
        assert!(!hsv1.approximately_equal(&hsv2));

        let hsv1 = Hsv::new(180.0, 0.5, 0.3).unwrap();
        let hsv2 = Hsv::new(179.9999, 0.6, 0.30001).unwrap();
        assert!(!hsv1.approximately_equal(&hsv2));
    }

    #[test]
    fn test_hsv_new_validation() {
        assert!(Hsv::new(0.0, 0.0, 0.0).is_ok());
        assert!(Hsv::new(360.0, 1.0, 1.0).is_ok());
        assert_eq!(Hsv::new(-1.0, 0.5, 0.5), Err(ColorError::InvalidHue(-1.0)));
        assert_eq!(
            Hsv::new(361.0, 0.5, 0.5),
            Err(ColorError::InvalidHue(361.0))
        );
        assert_eq!(
            Hsv::new(180.0, -0.1, 0.5),
            Err(ColorError::InvalidSaturation(-0.1))
        );
        assert_eq!(
            Hsv::new(180.0, 1.1, 0.5),
            Err(ColorError::InvalidSaturation(1.1))
        );
        assert_eq!(
            Hsv::new(180.0, 0.5, -0.1),
            Err(ColorError::InvalidValue(-0.1))
        );
        assert_eq!(
            Hsv::new(180.0, 0.5, 1.1),
            Err(ColorError::InvalidValue(1.1))
        );
    }

    #[test]
    fn test_edge_cases() {
        // Hue = 360 should work (edge of valid range)
        assert!(hsv_to_rgb(360.0, 1.0, 1.0).is_ok());

        // Saturation and value at boundaries
        assert!(hsv_to_rgb(180.0, 0.0, 0.0).is_ok());
        assert!(hsv_to_rgb(180.0, 1.0, 1.0).is_ok());

        // All RGB values at max
        assert!(rgb_to_hsv(255, 255, 255).is_ok());

        // All RGB values at min
        assert!(rgb_to_hsv(0, 0, 0).is_ok());
    }

    #[test]
    fn test_rgb_struct() {
        let rgb = Rgb::new(100, 150, 200);
        assert_eq!(rgb.red, 100);
        assert_eq!(rgb.green, 150);
        assert_eq!(rgb.blue, 200);
    }
}
