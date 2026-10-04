# Order Of Magnitude Conversion

**Category:** `conversions` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Length Unit Conversion

This module provides conversion between metric length units ranging from
meters to yottameters (10^24 meters).

Available units: Meter, Kilometer, Megameter, Gigameter, Terameter,
Petameter, Exameter, Zettameter, Yottameter

## Spelling Convention

This module uses **American spellings** (meter, kilometer, etc.) for all
official API elements including enum variants and documentation, following
standard programming conventions and SI guidelines.

However, the `FromStr` implementation **accepts both American and British spellings**
for maximum compatibility:
- American: "meter", "kilometer", "megameter", etc.
- British: "metre", "kilometre", "megametre", etc.

```
use the_algorithms_rust::conversions::MetricLengthUnit;
use std::str::FromStr;

// Both spellings work!
let american: MetricLengthUnit = "megameter".parse().unwrap();
let british: MetricLengthUnit = "megametre".parse().unwrap();
assert_eq!(american, british); // Same enum variant
```

# References

- [Meter - Wikipedia](https://en.wikipedia.org/wiki/Meter)
- [Kilometer - Wikipedia](https://en.wikipedia.org/wiki/Kilometer)
- [Orders of Magnitude (Length) - Wikipedia](https://en.wikipedia.org/wiki/Orders_of_magnitude_(length))

### Original Rust Signatures
```rust
pub fn exponent(&self) -> i32;
pub fn symbol(&self) -> &'static str;
pub fn convert_metric_length(value: f64, from: MetricLengthUnit, to: MetricLengthUnit) -> f64;
pub fn metric_length_conversion(value: f64, from_type: &str, to_type: &str) -> Result<f64, String>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

### Rust vs. Java Perspective
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`order_of_magnitude_conversion.rs`](../../../src/conversions/order_of_magnitude_conversion.rs).

---

## How to Spin & Run

### 1. Java (Target Interview Language)
Run directly as a single-file application with built-in tests:
```bash
java Solution.java
```

### 2. Rust (Original Ground Truth Answer)
Run the crate unit tests for this module from the repository root:
```bash
cargo test --lib conversions::order_of_magnitude_conversion
```

### 3. Python (Rapid Prototyping)
```bash
python3 solution.py
```

### 4. TypeScript (Industry Standard)
```bash
bun solution.ts
# or using Deno:
deno run solution.ts
```

---

## Self-Evaluation Checklist
- [ ] Understand the problem constraints and edge cases (e.g. empty input, bounds, duplicates).
- [ ] Implement the optimal solution in `Solution.java`.
- [ ] Verify correctness using `java Solution.java`.
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`order_of_magnitude_conversion.rs`](../../../src/conversions/order_of_magnitude_conversion.rs).
