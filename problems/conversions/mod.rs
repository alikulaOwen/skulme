// Automatically generated category module

#[path = "binary_to_decimal/solution.rs"]
pub mod binary_to_decimal;

#[path = "binary_to_hexadecimal/solution.rs"]
pub mod binary_to_hexadecimal;

#[path = "binary_to_octal/solution.rs"]
pub mod binary_to_octal;

#[path = "decimal_to_binary/solution.rs"]
pub mod decimal_to_binary;

#[path = "decimal_to_hexadecimal/solution.rs"]
pub mod decimal_to_hexadecimal;

#[path = "decimal_to_octal/solution.rs"]
pub mod decimal_to_octal;

#[path = "energy/solution.rs"]
pub mod energy;

#[path = "hexadecimal_to_binary/solution.rs"]
pub mod hexadecimal_to_binary;

#[path = "hexadecimal_to_decimal/solution.rs"]
pub mod hexadecimal_to_decimal;

#[path = "hexadecimal_to_octal/solution.rs"]
pub mod hexadecimal_to_octal;

#[path = "ipv4_conversion/solution.rs"]
pub mod ipv4_conversion;

#[path = "length_conversion/solution.rs"]
pub mod length_conversion;

#[path = "octal_to_binary/solution.rs"]
pub mod octal_to_binary;

#[path = "octal_to_decimal/solution.rs"]
pub mod octal_to_decimal;

#[path = "octal_to_hexadecimal/solution.rs"]
pub mod octal_to_hexadecimal;

#[path = "order_of_magnitude_conversion/solution.rs"]
pub mod order_of_magnitude_conversion;

#[path = "pressure/solution.rs"]
pub mod pressure;

#[path = "rectangular_to_polar/solution.rs"]
pub mod rectangular_to_polar;

#[path = "rgb_cmyk_conversion/solution.rs"]
pub mod rgb_cmyk_conversion;

#[path = "rgb_hsv_conversion/solution.rs"]
pub mod rgb_hsv_conversion;

#[path = "roman_numerals/solution.rs"]
pub mod roman_numerals;

#[path = "speed/solution.rs"]
pub mod speed;

#[path = "temperature/solution.rs"]
pub mod temperature;

#[path = "time/solution.rs"]
pub mod time;

#[path = "volume/solution.rs"]
pub mod volume;

#[path = "weight/solution.rs"]
pub mod weight;


pub use self::binary_to_decimal::binary_to_decimal;
pub use self::binary_to_hexadecimal::binary_to_hexadecimal;
pub use self::binary_to_octal::binary_to_octal;
pub use self::decimal_to_binary::decimal_to_binary;
pub use self::decimal_to_hexadecimal::decimal_to_hexadecimal;
pub use self::decimal_to_octal::decimal_to_octal;
pub use self::energy::{convert_energy, EnergyUnit};
pub use self::hexadecimal_to_binary::hexadecimal_to_binary;
pub use self::hexadecimal_to_decimal::hexadecimal_to_decimal;
pub use self::hexadecimal_to_octal::hexadecimal_to_octal;
pub use self::ipv4_conversion::{alt_ipv4_to_decimal, decimal_to_ipv4, ipv4_to_decimal, Ipv4Error};
pub use self::length_conversion::length_conversion;
pub use self::octal_to_binary::octal_to_binary;
pub use self::octal_to_decimal::octal_to_decimal;
pub use self::octal_to_hexadecimal::octal_to_hexadecimal;
pub use self::order_of_magnitude_conversion::{
    convert_metric_length, metric_length_conversion, MetricLengthUnit,
};
pub use self::pressure::{convert_pressure, PressureUnit};
pub use self::rectangular_to_polar::rectangular_to_polar;
pub use self::rgb_cmyk_conversion::rgb_to_cmyk;
pub use self::rgb_hsv_conversion::{hsv_to_rgb, rgb_to_hsv, ColorError, Hsv, Rgb};
pub use self::roman_numerals::{int_to_roman, roman_to_int};
pub use self::speed::{convert_speed, SpeedUnit};
pub use self::temperature::{convert_temperature, TemperatureUnit};
pub use self::time::convert_time;
pub use self::volume::{convert_volume, VolumeUnit};
pub use self::weight::{convert_weight, WeightUnit};
