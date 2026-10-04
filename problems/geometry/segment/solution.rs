use super::Point;

const TOLERANCE: f64 = 0.0001;

pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Segment {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Segment {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn from_points(a: Point, b: Point) -> Segment {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from_points)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from_points");
}

    pub fn direction(&self, p: &Point) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (direction)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement direction");
}

    pub fn is_vertical(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_vertical)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_vertical");
}

    // returns (slope, y-intercept)
    pub fn get_line_equation(&self) -> (f64, f64) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_line_equation)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_line_equation");
}

    // Compute the value of y at x. Uses the line equation, and assumes the segment
    // has infinite length.
    pub fn compute_y_at_x(&self, x: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (compute_y_at_x)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement compute_y_at_x");
}

    pub fn is_colinear(&self, p: &Point) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_colinear)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_colinear");
}

    // p must be colinear with the segment
    pub fn colinear_point_on_segment(&self, p: &Point) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (colinear_point_on_segment)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement colinear_point_on_segment");
}

    pub fn on_segment(&self, p: &Point) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (on_segment)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement on_segment");
}

    pub fn intersects(&self, other: &Segment) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (intersects)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement intersects");
}
}


#[cfg(test)]
mod tests {
    use super::Point;
    use super::Segment;

    #[test]
    fn colinear() {
        let segment = Segment::new(2.0, 3.0, 6.0, 5.0);
        assert_eq!((0.5, 2.0), segment.get_line_equation());

        assert!(segment.is_colinear(&Point::new(2.0, 3.0)));
        assert!(segment.is_colinear(&Point::new(6.0, 5.0)));
        assert!(segment.is_colinear(&Point::new(0.0, 2.0)));
        assert!(segment.is_colinear(&Point::new(-5.0, -0.5)));
        assert!(segment.is_colinear(&Point::new(10.0, 7.0)));

        assert!(!segment.is_colinear(&Point::new(0.0, 0.0)));
        assert!(!segment.is_colinear(&Point::new(1.9, 3.0)));
        assert!(!segment.is_colinear(&Point::new(2.1, 3.0)));
        assert!(!segment.is_colinear(&Point::new(2.0, 2.9)));
        assert!(!segment.is_colinear(&Point::new(2.0, 3.1)));
        assert!(!segment.is_colinear(&Point::new(5.9, 5.0)));
        assert!(!segment.is_colinear(&Point::new(6.1, 5.0)));
        assert!(!segment.is_colinear(&Point::new(6.0, 4.9)));
        assert!(!segment.is_colinear(&Point::new(6.0, 5.1)));
    }

    #[test]
    fn colinear_vertical() {
        let segment = Segment::new(2.0, 3.0, 2.0, 5.0);
        assert!(segment.is_colinear(&Point::new(2.0, 1.0)));
        assert!(segment.is_colinear(&Point::new(2.0, 3.0)));
        assert!(segment.is_colinear(&Point::new(2.0, 4.0)));
        assert!(segment.is_colinear(&Point::new(2.0, 5.0)));
        assert!(segment.is_colinear(&Point::new(2.0, 6.0)));

        assert!(!segment.is_colinear(&Point::new(1.0, 3.0)));
        assert!(!segment.is_colinear(&Point::new(3.0, 3.0)));
    }

    fn test_intersect(s1: &Segment, s2: &Segment, result: bool) {
        assert_eq!(s1.intersects(s2), result);
        assert_eq!(s2.intersects(s1), result);
    }

    #[test]
    fn intersects() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(-1.0, 9.0, 10.0, -3.0);
        let s3 = Segment::new(-0.0, 10.0, 11.0, -2.0);
        let s4 = Segment::new(100.0, 200.0, 40.0, 50.0);
        test_intersect(&s1, &s2, true);
        test_intersect(&s1, &s3, true);
        test_intersect(&s2, &s3, false);
        test_intersect(&s1, &s4, false);
        test_intersect(&s2, &s4, false);
        test_intersect(&s3, &s4, false);
    }

    #[test]
    fn intersects_endpoint_on_segment() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(4.0, 4.0, -11.0, 20.0);
        let s3 = Segment::new(4.0, 4.0, 14.0, -19.0);
        test_intersect(&s1, &s2, true);
        test_intersect(&s1, &s3, true);
    }

    #[test]
    fn intersects_self() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(2.0, 3.0, 6.0, 5.0);
        test_intersect(&s1, &s2, true);
    }

    #[test]
    fn too_short_to_intersect() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(-1.0, 10.0, 3.0, 5.0);
        let s3 = Segment::new(5.0, 3.0, 10.0, -11.0);
        test_intersect(&s1, &s2, false);
        test_intersect(&s1, &s3, false);
        test_intersect(&s2, &s3, false);
    }

    #[test]
    fn parallel_segments() {
        let s1 = Segment::new(-5.0, 0.0, 5.0, 0.0);
        let s2 = Segment::new(-5.0, 1.0, 5.0, 1.0);
        let s3 = Segment::new(-5.0, -1.0, 5.0, -1.0);
        test_intersect(&s1, &s2, false);
        test_intersect(&s1, &s3, false);
        test_intersect(&s2, &s3, false);
    }
}
