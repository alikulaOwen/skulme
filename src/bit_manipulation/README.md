# Bit Manipulation - Algorithm Practice & Interview Guide

Bitwise operations perform fast, low-level manipulation of individual bits in integral numbers.

## Key Java Interview Takeaways
- Note the bit shift difference:
  - `>>` is Arithmetic Right Shift (preserves sign bit).
  - `>>>` is Logical (Unsigned) Right Shift (fills left with zeros).
- Useful bit hacks:
  - `n & (n - 1)` removes the lowest set bit.
  - `n & (-n)` isolates the lowest set bit.
  - `n ^ n = 0` (used for single number / unique element finding).
- Built-ins: `Integer.bitCount(n)`, `Integer.highestOneBit(n)`, `Integer.numberOfLeadingZeros(n)`.

## Comparison: Rust vs Java
- Rust has explicit unsigned integer types (`u32`, `u64`), so `>>` is automatically unsigned for unsigned types.
- Java only has signed integers (`int`, `long`), which is why the `>>>` operator exists.

---

## Problems & Practice Workspaces (16 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Binary Coded Decimal](../../problems/bit_manipulation/binary_coded_decimal/README.md) | [`binary_coded_decimal.rs`](./binary_coded_decimal.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::binary_coded_decimal` |
| [Binary Count Trailing Zeros](../../problems/bit_manipulation/binary_count_trailing_zeros/README.md) | [`binary_count_trailing_zeros.rs`](./binary_count_trailing_zeros.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::binary_count_trailing_zeros` |
| [Binary Shifts](../../problems/bit_manipulation/binary_shifts/README.md) | [`binary_shifts.rs`](./binary_shifts.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::binary_shifts` |
| [Counting Bits](../../problems/bit_manipulation/counting_bits/README.md) | [`counting_bits.rs`](./counting_bits.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::counting_bits` |
| [Find Missing Number](../../problems/bit_manipulation/find_missing_number/README.md) | [`find_missing_number.rs`](./find_missing_number.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::find_missing_number` |
| [Find Previous Power Of Two](../../problems/bit_manipulation/find_previous_power_of_two/README.md) | [`find_previous_power_of_two.rs`](./find_previous_power_of_two.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::find_previous_power_of_two` |
| [Find Unique Number](../../problems/bit_manipulation/find_unique_number/README.md) | [`find_unique_number.rs`](./find_unique_number.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::find_unique_number` |
| [Hamming Distance](../../problems/bit_manipulation/hamming_distance/README.md) | [`hamming_distance.rs`](./hamming_distance.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::hamming_distance` |
| [Highest Set Bit](../../problems/bit_manipulation/highest_set_bit/README.md) | [`highest_set_bit.rs`](./highest_set_bit.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::highest_set_bit` |
| [Is Power Of Two](../../problems/bit_manipulation/is_power_of_two/README.md) | [`is_power_of_two.rs`](./is_power_of_two.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::is_power_of_two` |
| [N Bits Gray Code](../../problems/bit_manipulation/n_bits_gray_code/README.md) | [`n_bits_gray_code.rs`](./n_bits_gray_code.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::n_bits_gray_code` |
| [Reverse Bits](../../problems/bit_manipulation/reverse_bits/README.md) | [`reverse_bits.rs`](./reverse_bits.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::reverse_bits` |
| [Rightmost Set Bit](../../problems/bit_manipulation/rightmost_set_bit/README.md) | [`rightmost_set_bit.rs`](./rightmost_set_bit.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::rightmost_set_bit` |
| [Sum Of Two Integers](../../problems/bit_manipulation/sum_of_two_integers/README.md) | [`sum_of_two_integers.rs`](./sum_of_two_integers.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::sum_of_two_integers` |
| [Swap Odd Even Bits](../../problems/bit_manipulation/swap_odd_even_bits/README.md) | [`swap_odd_even_bits.rs`](./swap_odd_even_bits.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::swap_odd_even_bits` |
| [Twos Complement](../../problems/bit_manipulation/twos_complement/README.md) | [`twos_complement.rs`](./twos_complement.rs) | `java Solution.java` | `cargo test --lib bit_manipulation::twos_complement` |

---
*Generated for Java Software Engineering Interview Preparation.*
