type Ll = i64;
type Pll = (Ll, Ll);

fn cross(x1: Ll, y1: Ll, x2: Ll, y2: Ll) -> Ll {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cross)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cross");
}

pub fn polygon_area(pts: &[Pll]) -> Ll {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (polygon_area)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement polygon_area");
}

fn gcd(mut a: Ll, mut b: Ll) -> Ll {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gcd)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gcd");
}

fn boundary(pts: &[Pll]) -> Ll {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (boundary)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement boundary");
}

pub fn lattice_points(pts: &[Pll]) -> Ll {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (lattice_points)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement lattice_points");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cross() {
        assert_eq!(cross(1, 2, 3, 4), 4 - 3 * 2);
    }

    #[test]
    fn test_polygon_3_coordinates() {
        let pts = vec![(0, 0), (0, 3), (4, 0)];
        assert_eq!(polygon_area(&pts), 6);
    }

    #[test]
    fn test_polygon_4_coordinates() {
        let pts = vec![(0, 0), (0, 2), (2, 2), (2, 0)];
        assert_eq!(polygon_area(&pts), 4);
    }

    #[test]
    fn test_gcd_multiple_of_common_factor() {
        assert_eq!(gcd(14, 28), 14);
    }

    #[test]
    fn test_boundary() {
        let pts = vec![(0, 0), (0, 3), (0, 4), (2, 2)];
        assert_eq!(boundary(&pts), 8);
    }

    #[test]
    fn test_lattice_points() {
        let pts = vec![(1, 1), (5, 1), (5, 4)];
        let result = lattice_points(&pts);
        assert_eq!(result, 3);
    }
}
