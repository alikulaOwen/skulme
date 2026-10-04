# IPv4 Conversion

**Category:** `conversions` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Module for converting between IPv4 addresses and their decimal representations
/
/ This module provides functions to convert IPv4 addresses to decimal integers
/ and vice versa.
/
/ Reference: https://www.geeksforgeeks.org/convert-ip-address-to-integer-and-vice-versa/

### Original Rust Signatures
```rust
pub fn ipv4_to_decimal(ipv4_address: &str) -> Result<u32, Ipv4Error>;
pub fn alt_ipv4_to_decimal(ipv4_address: &str) -> Result<u32, Ipv4Error>;
pub fn decimal_to_ipv4(decimal_ipv4: u32) -> Result<String, Ipv4Error>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`ipv4_conversion.rs`](../../../src/conversions/ipv4_conversion.rs).

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
cargo test --lib conversions::ipv4_conversion
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`ipv4_conversion.rs`](../../../src/conversions/ipv4_conversion.rs).
