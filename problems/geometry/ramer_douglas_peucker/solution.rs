use crate::geometry::Point;

pub fn ramer_douglas_peucker(points: &[Point], epsilon: f64) -> Vec<Point> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (ramer_douglas_peucker)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement ramer_douglas_peucker");
}

fn perpendicular_distance(p: &Point, a: &Point, b: &Point) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (perpendicular_distance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement perpendicular_distance");
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_perpendicular_distance {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p, a, b, expected) = $test_case;
                    assert_eq!(perpendicular_distance(&p, &a, &b), expected);
                    assert_eq!(perpendicular_distance(&p, &b, &a), expected);
                }
            )*
        };
    }

    test_perpendicular_distance! {
        basic: (Point::new(4.0, 0.0), Point::new(0.0, 0.0), Point::new(0.0, 3.0), 4.0),
        basic_shifted_1: (Point::new(4.0, 1.0), Point::new(0.0, 1.0), Point::new(0.0, 4.0), 4.0),
        basic_shifted_2: (Point::new(2.0, 1.0), Point::new(-2.0, 1.0), Point::new(-2.0, 4.0), 4.0),
    }

    #[test]
    fn test_ramer_douglas_peucker_polygon() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(2.0, 0.0);
        let d = Point::new(2.0, 1.0);
        let e = Point::new(2.0, 2.0);
        let f = Point::new(1.0, 2.0);
        let g = Point::new(0.0, 2.0);
        let h = Point::new(0.0, 1.0);
        let polygon = vec![
            a.clone(),
            b,
            c.clone(),
            d,
            e.clone(),
            f,
            g.clone(),
            h.clone(),
        ];
        let epsilon = 0.7;
        let result = ramer_douglas_peucker(&polygon, epsilon);
        assert_eq!(result, vec![a, c, e, g, h]);
    }

    #[test]
    fn test_ramer_douglas_peucker_polygonal_chain() {
        let a = Point::new(0., 0.);
        let b = Point::new(2., 0.5);
        let c = Point::new(3., 3.);
        let d = Point::new(6., 3.);
        let e = Point::new(8., 4.);

        let points = vec![a.clone(), b, c, d, e.clone()];

        let epsilon = 3.; // The epsilon is quite large, so the result will be a single line
        let result = ramer_douglas_peucker(&points, epsilon);
        assert_eq!(result, vec![a, e]);
    }

    #[test]
    fn test_less_than_three_points() {
        let a = Point::new(0., 0.);
        let b = Point::new(1., 1.);

        let epsilon = 0.1;

        assert_eq!(ramer_douglas_peucker(&[], epsilon), vec![]);
        assert_eq!(
            ramer_douglas_peucker(std::slice::from_ref(&a), epsilon),
            vec![a.clone()]
        );
        assert_eq!(
            ramer_douglas_peucker(&[a.clone(), b.clone()], epsilon),
            vec![a, b]
        );
    }
}
