use std::f64::consts::PI;

const EARTH_RADIUS: f64 = 6371000.00;

pub fn haversine(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (haversine)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement haversine");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(
            format!(
                "{:.2}km",
                haversine(52.375603, 4.903206, 52.366059, 4.926692) / 1000.0
            ),
            "1.92km"
        );
    }
}
