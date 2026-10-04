# Weight

**Category:** `conversions` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Conversion of weight units.

This module provides conversion between various weight units including:
- Metric: Gigatonne (Gt), Megatonne (Mt), Metric Ton (t), Kilogram (kg), Gram (g),
Milligram (mg), Microgram (μg), Nanogram (ng), Picogram (pg)
- Imperial/US: Long Ton, Short Ton, Hundredweight (cwt), Quarter (qtr), Stone (st),
Pound (lb), Ounce (oz), Dram (dr), Grain (gr)
- Troy: Troy Pound (lb t), Troy Ounce (oz t), Pennyweight (dwt)
- Other: Carat (ct), Atomic Mass Unit (amu)

# References
- [Kilogram](https://en.wikipedia.org/wiki/Kilogram)
- [Gram](https://en.wikipedia.org/wiki/Gram)
- [Milligram](https://en.wikipedia.org/wiki/Milligram)
- [Microgram](https://en.wikipedia.org/wiki/Microgram)
- [Nanogram](https://en.wikipedia.org/wiki/Orders_of_magnitude_(mass))
- [Picogram](https://en.wikipedia.org/wiki/Orders_of_magnitude_(mass))
- [Tonne](https://en.wikipedia.org/wiki/Tonne)
- [Gigatonne](https://en.wikipedia.org/wiki/Tonne#Derived_units)
- [Megatonne](https://en.wikipedia.org/wiki/Tonne#Derived_units)
- [Long Ton](https://en.wikipedia.org/wiki/Long_ton)
- [Short Ton](https://en.wikipedia.org/wiki/Short_ton)
- [Pound](https://en.wikipedia.org/wiki/Pound_(mass))
- [Ounce](https://en.wikipedia.org/wiki/Ounce)
- [Stone](https://en.wikipedia.org/wiki/Stone_(unit))
- [Quarter](https://en.wikipedia.org/wiki/Quarter_(unit))
- [Hundredweight](https://en.wikipedia.org/wiki/Hundredweight)
- [Grain](https://en.wikipedia.org/wiki/Grain_(unit))
- [Dram](https://en.wikipedia.org/wiki/Dram_(unit))
- [Troy Pound](https://en.wikipedia.org/wiki/Troy_weight)
- [Troy Ounce](https://en.wikipedia.org/wiki/Troy_weight)
- [Pennyweight](https://en.wikipedia.org/wiki/Pennyweight)
- [Carat](https://en.wikipedia.org/wiki/Carat_(mass))
- [Dalton (Atomic Mass Unit)](https://en.wikipedia.org/wiki/Dalton_(unit))

### Original Rust Signatures
```rust
pub fn supported_units() -> Vec<&'static str>;
pub fn convert_weight(value: f64, from_unit: F, to_unit: T) -> Result<f64, String>
where
    F: IntoWeightUnit,
    T: IntoWeightUnit,;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`weight.rs`](../../../src/conversions/weight.rs).

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
cargo test --lib conversions::weight
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`weight.rs`](../../../src/conversions/weight.rs).
