use rand::random;

fn get_distance(p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_distance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_distance");
}

fn find_nearest(data_point: &(f64, f64), centroids: &[(f64, f64)]) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find_nearest)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find_nearest");
}

pub fn k_means(data_points: Vec<(f64, f64)>, n_clusters: usize, max_iter: i32) -> Option<Vec<u32>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (k_means)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement k_means");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_k_means() {
        let mut data_points: Vec<(f64, f64)> = vec![];
        let n_points: usize = 1000;

        for _ in 0..n_points {
            let x: f64 = random::<f64>() * 100.0;
            let y: f64 = random::<f64>() * 100.0;

            data_points.push((x, y));
        }

        println!("{:?}", k_means(data_points, 10, 100).unwrap_or_default());
    }
}
