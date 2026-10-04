use std::ops::{Add, Mul, MulAssign, Sub};

// f64 complex
#[derive(Clone, Copy, Debug)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    #[inline]
    pub fn new(re: f64, im: f64) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    #[inline]
    pub fn square_norm(&self) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (square_norm)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement square_norm");
}

    #[inline]
    pub fn norm(&self) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (norm)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement norm");
}

    #[inline]
    pub fn inverse(&self) -> Complex64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (inverse)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement inverse");
}
}

impl Default for Complex64 {
    #[inline]
    fn default() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement default");
}
}

impl Add<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn add(self, other: Complex64) -> Complex64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add");
}
}

impl Sub<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn sub(self, other: Complex64) -> Complex64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (sub)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement sub");
}
}

impl Mul<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn mul(self, other: Complex64) -> Complex64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mul)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mul");
}
}

impl MulAssign<Complex64> for Complex64 {
    #[inline]
    fn mul_assign(&mut self, other: Complex64) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mul_assign)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mul_assign");
}
}

pub fn fast_fourier_transform_input_permutation(length: usize) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fast_fourier_transform_input_permutation)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fast_fourier_transform_input_permutation");
}

pub fn fast_fourier_transform(input: &[f64], input_permutation: &[usize]) -> Vec<Complex64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fast_fourier_transform)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fast_fourier_transform");
}

pub fn inverse_fast_fourier_transform(
    input: &[Complex64],
    input_permutation: &[usize],
) -> Vec<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (inverse_fast_fourier_transform)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement inverse_fast_fourier_transform");
}


#[cfg(test)]
mod tests {
    use super::*;
    fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    const EPSILON: f64 = 1e-6;

    #[test]
    fn small_polynomial_returns_self() {
        let polynomial = vec![1.0f64, 1.0, 0.0, 2.5];
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let fft = fast_fourier_transform(&polynomial, &permutation);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        for (x, y) in ifft.iter().zip(polynomial.iter()) {
            assert!(almost_equal(*x, *y, EPSILON));
        }
    }

    #[test]
    fn square_small_polynomial() {
        let mut polynomial = vec![1.0f64, 1.0, 0.0, 2.0];
        polynomial.append(&mut vec![0.0; 4]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        for num in fft.iter_mut() {
            *num *= *num;
        }
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = [1.0, 2.0, 1.0, 4.0, 4.0, 0.0, 4.0, 0.0, 0.0];
        for (x, y) in ifft.iter().zip(expected.iter()) {
            assert!(almost_equal(*x, *y, EPSILON));
        }
    }

    #[test]
    #[ignore]
    fn square_big_polynomial() {
        // This test case takes ~1050ms on my machine in unoptimized mode,
        // but it takes ~70ms in release mode.
        let n = 1 << 17; // ~100_000
        let mut polynomial = vec![1.0f64; n];
        polynomial.append(&mut vec![0.0f64; n]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        for num in fft.iter_mut() {
            *num *= *num;
        }
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = (0..((n << 1) - 1)).map(|i| std::cmp::min(i + 1, (n << 1) - 1 - i) as f64);
        for (&x, y) in ifft.iter().zip(expected) {
            assert!(almost_equal(x, y, EPSILON));
        }
    }
}
