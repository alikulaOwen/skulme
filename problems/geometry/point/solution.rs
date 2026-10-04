use std::ops::Sub;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    // Returns the orientation of consecutive segments ab and bc.
    pub fn consecutive_orientation(&self, b: &Point, c: &Point) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (consecutive_orientation)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement consecutive_orientation");
}

    pub fn cross_prod(&self, other: &Point) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cross_prod)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cross_prod");
}

    pub fn euclidean_distance(&self, other: &Point) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (euclidean_distance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement euclidean_distance");
}
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (sub)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement sub");
}
}

