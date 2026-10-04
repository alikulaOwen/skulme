use crate::geometry::Point;
use std::cmp::Ordering;

fn point_min(a: &&Point, b: &&Point) -> Ordering {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (point_min)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement point_min");
}

// Returns a Vec of Points that make up the convex hull of `points`. Returns an empty Vec if there
// is no convex hull.
pub fn graham_scan(mut points: Vec<Point>) -> Vec<Point> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (graham_scan)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement graham_scan");
}


#[cfg(test)]
mod tests {
    use super::graham_scan;
    use super::Point;

    fn test_graham(convex_hull: Vec<Point>, others: Vec<Point>) {
        let mut points = convex_hull.clone();
        points.append(&mut others.clone());
        let graham = graham_scan(points);
        for point in convex_hull {
            assert!(graham.contains(&point));
        }
        for point in others {
            assert!(!graham.contains(&point));
        }
    }

    #[test]
    fn too_few_points() {
        test_graham(vec![], vec![]);
        test_graham(vec![], vec![Point::new(0.0, 0.0)]);
    }

    #[test]
    fn duplicate_point() {
        let p = Point::new(0.0, 0.0);
        test_graham(vec![], vec![p.clone(), p.clone(), p.clone(), p.clone(), p]);
    }

    #[test]
    fn points_same_line() {
        let p1 = Point::new(1.0, 0.0);
        let p2 = Point::new(2.0, 0.0);
        let p3 = Point::new(3.0, 0.0);
        let p4 = Point::new(4.0, 0.0);
        let p5 = Point::new(5.0, 0.0);
        // let p6 = Point::new(1.0, 1.0);
        test_graham(vec![], vec![p1, p2, p3, p4, p5]);
    }

    #[test]
    fn triangle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(1.5, 2.0);
        let points = vec![p1, p2, p3];
        test_graham(points, vec![]);
    }

    #[test]
    fn rectangle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(2.0, 2.0);
        let p4 = Point::new(1.0, 2.0);
        let points = vec![p1, p2, p3, p4];
        test_graham(points, vec![]);
    }

    #[test]
    fn triangle_with_points_in_middle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(1.5, 2.0);
        let p4 = Point::new(1.5, 1.5);
        let p5 = Point::new(1.2, 1.3);
        let p6 = Point::new(1.8, 1.2);
        let p7 = Point::new(1.5, 1.9);
        let hull = vec![p1, p2, p3];
        let others = vec![p4, p5, p6, p7];
        test_graham(hull, others);
    }

    #[test]
    fn rectangle_with_points_in_middle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(2.0, 2.0);
        let p4 = Point::new(1.0, 2.0);
        let p5 = Point::new(1.5, 1.5);
        let p6 = Point::new(1.2, 1.3);
        let p7 = Point::new(1.8, 1.2);
        let p8 = Point::new(1.9, 1.7);
        let p9 = Point::new(1.4, 1.9);
        let hull = vec![p1, p2, p3, p4];
        let others = vec![p5, p6, p7, p8, p9];
        test_graham(hull, others);
    }

    #[test]
    fn star() {
        // A single stroke star shape (kind of). Only the tips(p1-5) are part of the convex hull. The
        // other points would create angles >180 degrees if they were part of the polygon.
        let p1 = Point::new(-5.0, 6.0);
        let p2 = Point::new(-11.0, 0.0);
        let p3 = Point::new(-9.0, -8.0);
        let p4 = Point::new(4.0, 4.0);
        let p5 = Point::new(6.0, -7.0);
        let p6 = Point::new(-7.0, -2.0);
        let p7 = Point::new(-2.0, -4.0);
        let p8 = Point::new(0.0, 1.0);
        let p9 = Point::new(1.0, 0.0);
        let p10 = Point::new(-6.0, 1.0);
        let hull = vec![p1, p2, p3, p4, p5];
        let others = vec![p6, p7, p8, p9, p10];
        test_graham(hull, others);
    }

    #[test]
    fn rectangle_with_points_on_same_line() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(2.0, 2.0);
        let p4 = Point::new(1.0, 2.0);
        let p5 = Point::new(1.5, 1.0);
        let p6 = Point::new(1.0, 1.5);
        let p7 = Point::new(2.0, 1.5);
        let p8 = Point::new(1.5, 2.0);
        let hull = vec![p1, p2, p3, p4];
        let others = vec![p5, p6, p7, p8];
        test_graham(hull, others);
    }
}
