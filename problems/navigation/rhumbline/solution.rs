use std::f64::consts::PI;

const EARTH_RADIUS: f64 = 6371000.0;

pub fn rhumb_dist(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rhumb_dist)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rhumb_dist");
}

pub fn rhumb_bearing(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rhumb_bearing)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rhumb_bearing");
}
pub fn rhumb_destination(lat: f64, long: f64, distance: f64, bearing: f64) -> (f64, f64) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rhumb_destination)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rhumb_destination");
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhumb_distance() {
        let distance = rhumb_dist(28.5416, 77.2006, 28.5457, 77.1928);
        assert!(distance > 700.00 && distance < 1000.0);
    }

    #[test]
    fn test_rhumb_bearing() {
        let bearing = rhumb_bearing(28.5416, 77.2006, 28.5457, 77.1928);
        assert!((bearing - 300.0).abs() < 5.0);
    }

    #[test]
    fn test_rhumb_destination_point() {
        let (lat, lng) = rhumb_destination(28.5457, 77.1928, 1000.00, 305.0);
        assert!((lat - 28.550).abs() < 0.010);
        assert!((lng - 77.1851).abs() < 0.010);
    }
    // edge cases

    #[test]
    fn test_rhumb_distance_cross_antimeridian() {
        // Test when del_lambda > PI (line 12)
        let distance = rhumb_dist(0.0, 170.0, 0.0, -170.0);
        assert!(distance > 0.0);
    }

    #[test]
    fn test_rhumb_distance_cross_antimeridian_negative() {
        // Test when del_lambda < -PI (line 14)
        let distance = rhumb_dist(0.0, -170.0, 0.0, 170.0);
        assert!(distance > 0.0);
    }

    #[test]
    fn test_rhumb_distance_to_equator() {
        // Test when del_psi is near zero (line 21 - the else branch)
        let distance = rhumb_dist(0.0, 0.0, 0.0, 1.0);
        assert!(distance > 0.0);
    }
}
