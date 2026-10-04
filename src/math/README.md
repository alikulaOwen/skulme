# Math - Algorithm Practice & Interview Guide

Mathematical algorithms including number theory, combinatorics, modular arithmetic, and algebra.

## Key Java Interview Takeaways
- Watch out for 32-bit integer overflow: `(a + b)` and `(a * b)` can easily exceed `Integer.MAX_VALUE` (2^31 - 1). Cast to `long` before multiplication: `(long) a * b % MOD`.
- For arbitrary precision arithmetic, use `java.math.BigInteger` and `java.math.BigDecimal`.

## Comparison: Rust vs Java
- Rust panics on debug integer overflow and wraps in release mode (or offers `checked_add`, `saturating_mul`).
- Java silently overflows integer operations without exception unless `Math.addExact()` is explicitly used.

---

## Problems & Practice Workspaces (83 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Abs](../../problems/math/abs/README.md) | [`abs.rs`](./abs.rs) | `java Solution.java` | `cargo test --lib math::abs` |
| [Aliquot Sum](../../problems/math/aliquot_sum/README.md) | [`aliquot_sum.rs`](./aliquot_sum.rs) | `java Solution.java` | `cargo test --lib math::aliquot_sum` |
| [Amicable Numbers](../../problems/math/amicable_numbers/README.md) | [`amicable_numbers.rs`](./amicable_numbers.rs) | `java Solution.java` | `cargo test --lib math::amicable_numbers` |
| [Area Of Polygon](../../problems/math/area_of_polygon/README.md) | [`area_of_polygon.rs`](./area_of_polygon.rs) | `java Solution.java` | `cargo test --lib math::area_of_polygon` |
| [Area Under Curve](../../problems/math/area_under_curve/README.md) | [`area_under_curve.rs`](./area_under_curve.rs) | `java Solution.java` | `cargo test --lib math::area_under_curve` |
| [Armstrong Number](../../problems/math/armstrong_number/README.md) | [`armstrong_number.rs`](./armstrong_number.rs) | `java Solution.java` | `cargo test --lib math::armstrong_number` |
| [Average](../../problems/math/average/README.md) | [`average.rs`](./average.rs) | `java Solution.java` | `cargo test --lib math::average` |
| [Baby Step Giant Step](../../problems/math/baby_step_giant_step/README.md) | [`baby_step_giant_step.rs`](./baby_step_giant_step.rs) | `java Solution.java` | `cargo test --lib math::baby_step_giant_step` |
| [Bell Numbers](../../problems/math/bell_numbers/README.md) | [`bell_numbers.rs`](./bell_numbers.rs) | `java Solution.java` | `cargo test --lib math::bell_numbers` |
| [Binary Exponentiation](../../problems/math/binary_exponentiation/README.md) | [`binary_exponentiation.rs`](./binary_exponentiation.rs) | `java Solution.java` | `cargo test --lib math::binary_exponentiation` |
| [Binomial Coefficient](../../problems/math/binomial_coefficient/README.md) | [`binomial_coefficient.rs`](./binomial_coefficient.rs) | `java Solution.java` | `cargo test --lib math::binomial_coefficient` |
| [Catalan Numbers](../../problems/math/catalan_numbers/README.md) | [`catalan_numbers.rs`](./catalan_numbers.rs) | `java Solution.java` | `cargo test --lib math::catalan_numbers` |
| [Ceil](../../problems/math/ceil/README.md) | [`ceil.rs`](./ceil.rs) | `java Solution.java` | `cargo test --lib math::ceil` |
| [Chinese Remainder Theorem](../../problems/math/chinese_remainder_theorem/README.md) | [`chinese_remainder_theorem.rs`](./chinese_remainder_theorem.rs) | `java Solution.java` | `cargo test --lib math::chinese_remainder_theorem` |
| [Collatz Sequence](../../problems/math/collatz_sequence/README.md) | [`collatz_sequence.rs`](./collatz_sequence.rs) | `java Solution.java` | `cargo test --lib math::collatz_sequence` |
| [Combinations](../../problems/math/combinations/README.md) | [`combinations.rs`](./combinations.rs) | `java Solution.java` | `cargo test --lib math::combinations` |
| [Cross Entropy Loss](../../problems/math/cross_entropy_loss/README.md) | [`cross_entropy_loss.rs`](./cross_entropy_loss.rs) | `java Solution.java` | `cargo test --lib math::cross_entropy_loss` |
| [Decimal To Fraction](../../problems/math/decimal_to_fraction/README.md) | [`decimal_to_fraction.rs`](./decimal_to_fraction.rs) | `java Solution.java` | `cargo test --lib math::decimal_to_fraction` |
| [Doomsday](../../problems/math/doomsday/README.md) | [`doomsday.rs`](./doomsday.rs) | `java Solution.java` | `cargo test --lib math::doomsday` |
| [Elliptic Curve](../../problems/math/elliptic_curve/README.md) | [`elliptic_curve.rs`](./elliptic_curve.rs) | `java Solution.java` | `cargo test --lib math::elliptic_curve` |
| [Euclidean Distance](../../problems/math/euclidean_distance/README.md) | [`euclidean_distance.rs`](./euclidean_distance.rs) | `java Solution.java` | `cargo test --lib math::euclidean_distance` |
| [Exponential Linear Unit](../../problems/math/exponential_linear_unit/README.md) | [`exponential_linear_unit.rs`](./exponential_linear_unit.rs) | `java Solution.java` | `cargo test --lib math::exponential_linear_unit` |
| [Extended Euclidean Algorithm](../../problems/math/extended_euclidean_algorithm/README.md) | [`extended_euclidean_algorithm.rs`](./extended_euclidean_algorithm.rs) | `java Solution.java` | `cargo test --lib math::extended_euclidean_algorithm` |
| [Factorial](../../problems/math/factorial/README.md) | [`factorial.rs`](./factorial.rs) | `java Solution.java` | `cargo test --lib math::factorial` |
| [Factors](../../problems/math/factors/README.md) | [`factors.rs`](./factors.rs) | `java Solution.java` | `cargo test --lib math::factors` |
| [Fast Fourier Transform](../../problems/math/fast_fourier_transform/README.md) | [`fast_fourier_transform.rs`](./fast_fourier_transform.rs) | `java Solution.java` | `cargo test --lib math::fast_fourier_transform` |
| [Fast Power](../../problems/math/fast_power/README.md) | [`fast_power.rs`](./fast_power.rs) | `java Solution.java` | `cargo test --lib math::fast_power` |
| [Faster Perfect Numbers](../../problems/math/faster_perfect_numbers/README.md) | [`faster_perfect_numbers.rs`](./faster_perfect_numbers.rs) | `java Solution.java` | `cargo test --lib math::faster_perfect_numbers` |
| [Field](../../problems/math/field/README.md) | [`field.rs`](./field.rs) | `java Solution.java` | `cargo test --lib math::field` |
| [Frizzy Number](../../problems/math/frizzy_number/README.md) | [`frizzy_number.rs`](./frizzy_number.rs) | `java Solution.java` | `cargo test --lib math::frizzy_number` |
| [Gaussian Elimination](../../problems/math/gaussian_elimination/README.md) | [`gaussian_elimination.rs`](./gaussian_elimination.rs) | `java Solution.java` | `cargo test --lib math::gaussian_elimination` |
| [Gaussian Error Linear Unit](../../problems/math/gaussian_error_linear_unit/README.md) | [`gaussian_error_linear_unit.rs`](./gaussian_error_linear_unit.rs) | `java Solution.java` | `cargo test --lib math::gaussian_error_linear_unit` |
| [GCD Of N Numbers](../../problems/math/gcd_of_n_numbers/README.md) | [`gcd_of_n_numbers.rs`](./gcd_of_n_numbers.rs) | `java Solution.java` | `cargo test --lib math::gcd_of_n_numbers` |
| [Geometric Series](../../problems/math/geometric_series/README.md) | [`geometric_series.rs`](./geometric_series.rs) | `java Solution.java` | `cargo test --lib math::geometric_series` |
| [Greatest Common Divisor](../../problems/math/greatest_common_divisor/README.md) | [`greatest_common_divisor.rs`](./greatest_common_divisor.rs) | `java Solution.java` | `cargo test --lib math::greatest_common_divisor` |
| [Huber Loss](../../problems/math/huber_loss/README.md) | [`huber_loss.rs`](./huber_loss.rs) | `java Solution.java` | `cargo test --lib math::huber_loss` |
| [Infix To Postfix](../../problems/math/infix_to_postfix/README.md) | [`infix_to_postfix.rs`](./infix_to_postfix.rs) | `java Solution.java` | `cargo test --lib math::infix_to_postfix` |
| [Interest](../../problems/math/interest/README.md) | [`interest.rs`](./interest.rs) | `java Solution.java` | `cargo test --lib math::interest` |
| [Interpolation](../../problems/math/interpolation/README.md) | [`interpolation.rs`](./interpolation.rs) | `java Solution.java` | `cargo test --lib math::interpolation` |
| [Interquartile Range](../../problems/math/interquartile_range/README.md) | [`interquartile_range.rs`](./interquartile_range.rs) | `java Solution.java` | `cargo test --lib math::interquartile_range` |
| [Karatsuba Multiplication](../../problems/math/karatsuba_multiplication/README.md) | [`karatsuba_multiplication.rs`](./karatsuba_multiplication.rs) | `java Solution.java` | `cargo test --lib math::karatsuba_multiplication` |
| [LCM Of N Numbers](../../problems/math/lcm_of_n_numbers/README.md) | [`lcm_of_n_numbers.rs`](./lcm_of_n_numbers.rs) | `java Solution.java` | `cargo test --lib math::lcm_of_n_numbers` |
| [Leaky Relu](../../problems/math/leaky_relu/README.md) | [`leaky_relu.rs`](./leaky_relu.rs) | `java Solution.java` | `cargo test --lib math::leaky_relu` |
| [Least Square Approx](../../problems/math/least_square_approx/README.md) | [`least_square_approx.rs`](./least_square_approx.rs) | `java Solution.java` | `cargo test --lib math::least_square_approx` |
| [Linear Sieve](../../problems/math/linear_sieve/README.md) | [`linear_sieve.rs`](./linear_sieve.rs) | `java Solution.java` | `cargo test --lib math::linear_sieve` |
| [Logarithm](../../problems/math/logarithm/README.md) | [`logarithm.rs`](./logarithm.rs) | `java Solution.java` | `cargo test --lib math::logarithm` |
| [Lucas Series](../../problems/math/lucas_series/README.md) | [`lucas_series.rs`](./lucas_series.rs) | `java Solution.java` | `cargo test --lib math::lucas_series` |
| [Matrix Ops](../../problems/math/matrix_ops/README.md) | [`matrix_ops.rs`](./matrix_ops.rs) | `java Solution.java` | `cargo test --lib math::matrix_ops` |
| [Mersenne Primes](../../problems/math/mersenne_primes/README.md) | [`mersenne_primes.rs`](./mersenne_primes.rs) | `java Solution.java` | `cargo test --lib math::mersenne_primes` |
| [Miller Rabin](../../problems/math/miller_rabin/README.md) | [`miller_rabin.rs`](./miller_rabin.rs) | `java Solution.java` | `cargo test --lib math::miller_rabin` |
| [Modular Exponential](../../problems/math/modular_exponential/README.md) | [`modular_exponential.rs`](./modular_exponential.rs) | `java Solution.java` | `cargo test --lib math::modular_exponential` |
| [Newton Raphson](../../problems/math/newton_raphson/README.md) | [`newton_raphson.rs`](./newton_raphson.rs) | `java Solution.java` | `cargo test --lib math::newton_raphson` |
| [Nthprime](../../problems/math/nthprime/README.md) | [`nthprime.rs`](./nthprime.rs) | `java Solution.java` | `cargo test --lib math::nthprime` |
| [Pascal Triangle](../../problems/math/pascal_triangle/README.md) | [`pascal_triangle.rs`](./pascal_triangle.rs) | `java Solution.java` | `cargo test --lib math::pascal_triangle` |
| [Perfect Cube](../../problems/math/perfect_cube/README.md) | [`perfect_cube.rs`](./perfect_cube.rs) | `java Solution.java` | `cargo test --lib math::perfect_cube` |
| [Perfect Numbers](../../problems/math/perfect_numbers/README.md) | [`perfect_numbers.rs`](./perfect_numbers.rs) | `java Solution.java` | `cargo test --lib math::perfect_numbers` |
| [Perfect Square](../../problems/math/perfect_square/README.md) | [`perfect_square.rs`](./perfect_square.rs) | `java Solution.java` | `cargo test --lib math::perfect_square` |
| [Pollard Rho](../../problems/math/pollard_rho/README.md) | [`pollard_rho.rs`](./pollard_rho.rs) | `java Solution.java` | `cargo test --lib math::pollard_rho` |
| [Postfix Evaluation](../../problems/math/postfix_evaluation/README.md) | [`postfix_evaluation.rs`](./postfix_evaluation.rs) | `java Solution.java` | `cargo test --lib math::postfix_evaluation` |
| [Prime Check](../../problems/math/prime_check/README.md) | [`prime_check.rs`](./prime_check.rs) | `java Solution.java` | `cargo test --lib math::prime_check` |
| [Prime Factors](../../problems/math/prime_factors/README.md) | [`prime_factors.rs`](./prime_factors.rs) | `java Solution.java` | `cargo test --lib math::prime_factors` |
| [Prime Numbers](../../problems/math/prime_numbers/README.md) | [`prime_numbers.rs`](./prime_numbers.rs) | `java Solution.java` | `cargo test --lib math::prime_numbers` |
| [Quadratic Residue](../../problems/math/quadratic_residue/README.md) | [`quadratic_residue.rs`](./quadratic_residue.rs) | `java Solution.java` | `cargo test --lib math::quadratic_residue` |
| [Random](../../problems/math/random/README.md) | [`random.rs`](./random.rs) | `java Solution.java` | `cargo test --lib math::random` |
| [Relu](../../problems/math/relu/README.md) | [`relu.rs`](./relu.rs) | `java Solution.java` | `cargo test --lib math::relu` |
| [Sieve Of Eratosthenes](../../problems/math/sieve_of_eratosthenes/README.md) | [`sieve_of_eratosthenes.rs`](./sieve_of_eratosthenes.rs) | `java Solution.java` | `cargo test --lib math::sieve_of_eratosthenes` |
| [Sigmoid](../../problems/math/sigmoid/README.md) | [`sigmoid.rs`](./sigmoid.rs) | `java Solution.java` | `cargo test --lib math::sigmoid` |
| [Signum](../../problems/math/signum/README.md) | [`signum.rs`](./signum.rs) | `java Solution.java` | `cargo test --lib math::signum` |
| [Simpsons Integration](../../problems/math/simpsons_integration/README.md) | [`simpsons_integration.rs`](./simpsons_integration.rs) | `java Solution.java` | `cargo test --lib math::simpsons_integration` |
| [Softmax](../../problems/math/softmax/README.md) | [`softmax.rs`](./softmax.rs) | `java Solution.java` | `cargo test --lib math::softmax` |
| [Sprague Grundy Theorem](../../problems/math/sprague_grundy_theorem/README.md) | [`sprague_grundy_theorem.rs`](./sprague_grundy_theorem.rs) | `java Solution.java` | `cargo test --lib math::sprague_grundy_theorem` |
| [Square Pyramidal Numbers](../../problems/math/square_pyramidal_numbers/README.md) | [`square_pyramidal_numbers.rs`](./square_pyramidal_numbers.rs) | `java Solution.java` | `cargo test --lib math::square_pyramidal_numbers` |
| [Square Root](../../problems/math/square_root/README.md) | [`square_root.rs`](./square_root.rs) | `java Solution.java` | `cargo test --lib math::square_root` |
| [Sum Of Digits](../../problems/math/sum_of_digits/README.md) | [`sum_of_digits.rs`](./sum_of_digits.rs) | `java Solution.java` | `cargo test --lib math::sum_of_digits` |
| [Sum Of Geometric Progression](../../problems/math/sum_of_geometric_progression/README.md) | [`sum_of_geometric_progression.rs`](./sum_of_geometric_progression.rs) | `java Solution.java` | `cargo test --lib math::sum_of_geometric_progression` |
| [Sum Of Harmonic Series](../../problems/math/sum_of_harmonic_series/README.md) | [`sum_of_harmonic_series.rs`](./sum_of_harmonic_series.rs) | `java Solution.java` | `cargo test --lib math::sum_of_harmonic_series` |
| [Sylvester Sequence](../../problems/math/sylvester_sequence/README.md) | [`sylvester_sequence.rs`](./sylvester_sequence.rs) | `java Solution.java` | `cargo test --lib math::sylvester_sequence` |
| [Tanh](../../problems/math/tanh/README.md) | [`tanh.rs`](./tanh.rs) | `java Solution.java` | `cargo test --lib math::tanh` |
| [Trapezoidal Integration](../../problems/math/trapezoidal_integration/README.md) | [`trapezoidal_integration.rs`](./trapezoidal_integration.rs) | `java Solution.java` | `cargo test --lib math::trapezoidal_integration` |
| [Trial Division](../../problems/math/trial_division/README.md) | [`trial_division.rs`](./trial_division.rs) | `java Solution.java` | `cargo test --lib math::trial_division` |
| [Trig Functions](../../problems/math/trig_functions/README.md) | [`trig_functions.rs`](./trig_functions.rs) | `java Solution.java` | `cargo test --lib math::trig_functions` |
| [Vector Cross Product](../../problems/math/vector_cross_product/README.md) | [`vector_cross_product.rs`](./vector_cross_product.rs) | `java Solution.java` | `cargo test --lib math::vector_cross_product` |
| [Zellers Congruence Algorithm](../../problems/math/zellers_congruence_algorithm/README.md) | [`zellers_congruence_algorithm.rs`](./zellers_congruence_algorithm.rs) | `java Solution.java` | `cargo test --lib math::zellers_congruence_algorithm` |

---
*Generated for Java Software Engineering Interview Preparation.*
