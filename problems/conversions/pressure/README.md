# Pressure

**Category:** `conversions` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Conversion of pressure units.

This module provides conversion between various pressure units including:
Pascal (Pa, kPa, MPa, GPa), Bar (bar, mbar), Atmosphere (atm, at, ata),
Torr (Torr, mTorr), PSI (psi, ksi), Barad (Ba), Pièze (pz),
and manometric units (mmHg, cmHg, inHg, mmH2O, cmH2O, inH2O, msw, fsw).

# References
- [Units of Pressure](https://msestudent.com/what-are-the-units-of-pressure/)

### Original Rust Signatures
```rust
pub fn supported_units() -> Vec<&'static str>;
pub fn convert_pressure(value: f64, from_unit: F, to_unit: T) -> Result<f64, String>
where
    F: IntoPressureUnit,
    T: IntoPressureUnit,;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`pressure.rs`](../../../src/conversions/pressure.rs).

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
cargo test --lib conversions::pressure
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`pressure.rs`](../../../src/conversions/pressure.rs).
