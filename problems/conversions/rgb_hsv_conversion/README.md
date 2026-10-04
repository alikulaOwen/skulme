# RGB HSV Conversion

**Category:** `conversions` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Module for converting between RGB and HSV color representations

The RGB color model is an additive color model in which red, green, and blue light
are added together in various ways to reproduce a broad array of colors. The name
of the model comes from the initials of the three additive primary colors, red,
green, and blue. Meanwhile, the HSV representation models how colors appear under
light. In it, colors are represented using three components: hue, saturation and
(brightness-)value.

References:
- https://en.wikipedia.org/wiki/RGB_color_model
- https://en.wikipedia.org/wiki/HSL_and_HSV
- https://www.rapidtables.com/convert/color/hsv-to-rgb.html

### Original Rust Signatures
```rust
pub fn new(red: u8, green: u8, blue: u8) -> Self;
pub fn new(hue: f64, saturation: f64, value: f64) -> Result<Self, ColorError>;
pub fn approximately_equal(&self, other: &Hsv) -> bool;
pub fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> Result<Rgb, ColorError>;
pub fn rgb_to_hsv(red: u8, green: u8, blue: u8) -> Result<Hsv, ColorError>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`rgb_hsv_conversion.rs`](../../../src/conversions/rgb_hsv_conversion.rs).

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
cargo test --lib conversions::rgb_hsv_conversion
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`rgb_hsv_conversion.rs`](../../../src/conversions/rgb_hsv_conversion.rs).
