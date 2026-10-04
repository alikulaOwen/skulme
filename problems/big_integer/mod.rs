#![cfg(feature = "big-math")]

// Automatically generated category module

#[path = "fast_factorial/solution.rs"]
pub mod fast_factorial;

#[path = "multiply/solution.rs"]
pub mod multiply;

#[path = "poly1305/solution.rs"]
pub mod poly1305;


pub use self::fast_factorial::fast_factorial;
pub use self::multiply::multiply;
pub use self::poly1305::Poly1305;
