use crate::geometry::Point;
use std::cmp::Ordering;

fn cmp_x(p1: &Point, p2: &Point) -> Ordering {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cmp_x)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cmp_x");
}

fn f64_cmp(a: &f64, b: &f64) -> Ordering {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (f64_cmp)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement f64_cmp");
}

/// returns the two closest points
/// or None if there are zero or one point
pub fn closest_points(points: &[Point]) -> Option<(Point, Point)> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (closest_points)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement closest_points");
}

// We maintain two vectors with the same points, one sort by x coordinates and one sorted by y
// coordinates.
fn closest_points_aux(
    points_x: &[Point],
    points_y: Vec<Point>,
    mut start: usize,
    mut end: usize,
) -> Option<(Point, Point)> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (closest_points_aux)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement closest_points_aux");
}


#[cfg(test)]
mod tests {
    use super::closest_points;
    use super::Point;

    fn eq(p1: Option<(Point, Point)>, p2: Option<(Point, Point)>) -> bool {
        match (p1, p2) {
            (None, None) => true,
            (Some((p1, p2)), Some((p3, p4))) => (p1 == p3 && p2 == p4) || (p1 == p4 && p2 == p3),
            _ => false,
        }
    }

    macro_rules! assert_display {
        ($left: expr, $right: expr) => {
            assert!(
                eq($left, $right),
                "assertion failed: `(left == right)`\nleft: `{:?}`,\nright: `{:?}`",
                $left,
                $right
            )
        };
    }

    #[test]
    fn zero_points() {
        let vals: [Point; 0] = [];
        assert_display!(closest_points(&vals), None::<(Point, Point)>);
    }

    #[test]
    fn one_points() {
        let vals = [Point::new(0., 0.)];
        assert_display!(closest_points(&vals), None::<(Point, Point)>);
    }

    #[test]
    fn two_points() {
        let vals = [Point::new(0., 0.), Point::new(1., 1.)];
        assert_display!(
            closest_points(&vals),
            Some((vals[0].clone(), vals[1].clone()))
        );
    }

    #[test]
    fn three_points() {
        let vals = [Point::new(0., 0.), Point::new(1., 1.), Point::new(3., 3.)];
        assert_display!(
            closest_points(&vals),
            Some((vals[0].clone(), vals[1].clone()))
        );
    }

    #[test]
    fn list_1() {
        let vals = [
            Point::new(0., 0.),
            Point::new(2., 1.),
            Point::new(5., 2.),
            Point::new(2., 3.),
            Point::new(4., 0.),
            Point::new(0., 4.),
            Point::new(5., 6.),
            Point::new(4., 4.),
            Point::new(7., 3.),
            Point::new(-1., 2.),
            Point::new(2., 6.),
        ];
        assert_display!(
            closest_points(&vals),
            Some((Point::new(2., 1.), Point::new(2., 3.)))
        );
    }

    #[test]
    fn list_2() {
        let vals = [
            Point::new(1., 3.),
            Point::new(4., 6.),
            Point::new(8., 8.),
            Point::new(7., 5.),
            Point::new(5., 3.),
            Point::new(10., 3.),
            Point::new(7., 1.),
            Point::new(8., 3.),
            Point::new(4., 9.),
            Point::new(4., 12.),
            Point::new(4., 15.),
            Point::new(7., 14.),
            Point::new(8., 12.),
            Point::new(6., 10.),
            Point::new(4., 14.),
            Point::new(2., 7.),
            Point::new(3., 8.),
            Point::new(5., 8.),
            Point::new(6., 7.),
            Point::new(8., 10.),
            Point::new(6., 12.),
        ];
        assert_display!(
            closest_points(&vals),
            Some((Point::new(4., 14.), Point::new(4., 15.)))
        );
    }

    #[test]
    fn vertical_points() {
        let vals = [
            Point::new(0., 0.),
            Point::new(0., 50.),
            Point::new(0., -25.),
            Point::new(0., 40.),
            Point::new(0., 42.),
            Point::new(0., 100.),
            Point::new(0., 17.),
            Point::new(0., 29.),
            Point::new(0., -50.),
            Point::new(0., 37.),
            Point::new(0., 34.),
            Point::new(0., 8.),
            Point::new(0., 3.),
            Point::new(0., 46.),
        ];
        assert_display!(
            closest_points(&vals),
            Some((Point::new(0., 40.), Point::new(0., 42.)))
        );
    }
}
