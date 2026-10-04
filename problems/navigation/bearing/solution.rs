use std::f64::consts::PI;

pub fn bearing(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bearing)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bearing");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(
            format!(
                "{:.0}º",
                bearing(
                    -27.2020447088982,
                    -49.631891179172555,
                    -3.106362,
                    -60.025826,
                )
            ),
            "336º"
        );
    }
}
