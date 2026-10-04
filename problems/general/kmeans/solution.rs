// Macro to implement kmeans for both f64 and f32 without writing everything
// twice or importing the `num` crate
macro_rules! impl_kmeans {
    ($kind: ty, $modname: ident) => {
        // Since we can't overload methods in rust, we have to use namespacing
        pub mod $modname {

            /// computes sum of squared deviation between two identically sized vectors
            /// `x`, and `y`.
            fn distance(x: &[$kind], y: &[$kind]) -> $kind {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (distance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement distance");
}

            /// Returns a vector containing the indices z<sub>i</sub> in {0, ..., K-1} of
            /// the centroid nearest to each datum.
            fn nearest_centroids(xs: &[Vec<$kind>], centroids: &[Vec<$kind>]) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (nearest_centroids)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement nearest_centroids");
}

            /// Recompute the centroids given the current clustering
            fn recompute_centroids(
                xs: &[Vec<$kind>],
                clustering: &[usize],
                k: usize,
            ) -> Vec<Vec<$kind>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (recompute_centroids)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement recompute_centroids");
}

            /// Assign the N D-dimensional data, `xs`, to `k` clusters using K-Means clustering
            pub fn kmeans(xs: Vec<Vec<$kind>>, k: usize) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (kmeans)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement kmeans");
}
        }
    };
}

// generate code for kmeans for f32 and f64 data
impl_kmeans!(f64, f64);
impl_kmeans!(f32, f32);


#[cfg(test)]
mod test {
    use self::super::f64::kmeans;

    #[test]
    fn easy_univariate_clustering() {
        let xs: Vec<Vec<f64>> = vec![
            vec![-1.1],
            vec![-1.2],
            vec![-1.3],
            vec![-1.4],
            vec![1.1],
            vec![1.2],
            vec![1.3],
            vec![1.4],
        ];
        let clustering = kmeans(xs, 2);
        assert_eq!(clustering, vec![0, 0, 0, 0, 1, 1, 1, 1]);
    }

    #[test]
    fn easy_univariate_clustering_odd_number_of_data() {
        let xs: Vec<Vec<f64>> = vec![
            vec![-1.1],
            vec![-1.2],
            vec![-1.3],
            vec![-1.4],
            vec![1.1],
            vec![1.2],
            vec![1.3],
            vec![1.4],
            vec![1.5],
        ];
        let clustering = kmeans(xs, 2);
        assert_eq!(clustering, vec![0, 0, 0, 0, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn easy_bivariate_clustering() {
        let xs: Vec<Vec<f64>> = vec![
            vec![-1.1, 0.2],
            vec![-1.2, 0.3],
            vec![-1.3, 0.1],
            vec![-1.4, 0.4],
            vec![1.1, -1.1],
            vec![1.2, -1.0],
            vec![1.3, -1.2],
            vec![1.4, -1.3],
        ];
        let clustering = kmeans(xs, 2);
        assert_eq!(clustering, vec![0, 0, 0, 0, 1, 1, 1, 1]);
    }

    #[test]
    fn high_dims() {
        let xs: Vec<Vec<f64>> = vec![
            vec![-2.7825343, -1.7604825, -5.5550113, -2.9752946, -2.7874138],
            vec![-2.9847919, -3.8209332, -2.1531757, -2.2710119, -2.3582877],
            vec![-3.0109320, -2.2366132, -2.8048492, -1.2632331, -4.5755581],
            vec![-2.8432186, -1.0383805, -2.2022826, -2.7435962, -2.0013399],
            vec![-2.6638082, -3.5520086, -1.3684702, -2.1562444, -1.3186447],
            vec![1.7409171, 1.9687576, 4.7162628, 4.5743537, 3.7905611],
            vec![3.2932369, 2.8508700, 2.5580937, 2.0437325, 4.2192562],
            vec![2.5843321, 2.8329818, 2.1329531, 3.2562319, 2.4878733],
            vec![2.1859638, 3.2880048, 3.7018615, 2.3641232, 1.6281994],
            vec![2.6201773, 0.9006588, 2.6774097, 1.8188620, 1.6076493],
        ];

        let clustering = kmeans(xs, 2);
        assert_eq!(clustering, vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);
    }
}
