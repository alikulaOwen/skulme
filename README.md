<div align="center">
  <h1>💀 skul.me</h1>
  <p><strong>School Me in Algorithms, Systems & Multi-Language DSA</strong></p>
  <p><em>Powered by <a href="https://the-algorithms.com/">The Algorithms</a> &amp; <a href="https://github.com/TheAlgorithms/Rust">TheAlgorithms/Rust</a></em></p>

  <a href="https://skul.me">
    <img src="https://img.shields.io/badge/Platform-skul.me-black?style=for-the-badge&logo=vercel" alt="skul.me">
  </a>
  <a href="https://github.com/TheAlgorithms/Rust">
    <img src="https://img.shields.io/badge/Algorithms-394%20Problems-orange?style=for-the-badge&logo=rust" alt="394 Problems">
  </a>
  <img src="https://img.shields.io/badge/Languages-Rust%20%7C%20Java%20%7C%20Python%20%7C%20TS%20%7C%20SQL-blue?style=for-the-badge" alt="Multi-Language">
</div>

---

## 🌟 What is skul.me?

**skul.me** is an interactive multi-language coding playground and curriculum featuring **394 classic algorithms across 25 categories**. Every problem is crafted with:
- **Real-World Storytelling**: Technical algorithms mapped to practical scenarios (e.g. Courier Delivery Loops, 5G Tower Channels, Chess Inspectors, Security Matrix Vaults).
- **Socratic Prompting Hints**: In-code comments structured as guided prompting questions (`❓ Q1`, `❓ Q2`, `❓ Q3`) that teach **what** to do, **why** it matters, and **how** it scales without giving away the answer.
- **Multi-Language Practice**: Solve any problem in **Rust**, **Java**, **Python**, **TypeScript**, or **SQL**.
- **Interactive Test Harnesses**: Standalone test runners that run instantly from terminal or IDE.
- **Supabase Platform Ready**: Complete database schema and seed datasets ready for the [skul.me](https://skul.me) Lovable web application.

---

## 🗺️ Problem Categories (394 Total)

| Category | Problem Count | Category | Problem Count |
|---|---|---|---|
| **Backtracking** | 10 | **Searching** | 16 |
| **Sorting** | 37 | **Dynamic Programming** | 40 |
| **Graph Theory** | 31 | **Data Structures** | 35 |
| **Math & Number Theory** | 73 | **Bit Manipulation** | 18 |
| **Ciphers & Cryptography** | 27 | **String Algorithms** | 28 |
| **Machine Learning** | 18 | **Compression** | 6 |
| **Conversions** | 15 | **Financial** | 4 |
| **Geometry** | 8 | **Navigation** | 3 |
| **Signal Analysis** | 2 | **Probabilistic** | 2 |

---

## ⚡ Multi-Language Practice Runner

Run tests for any problem across any language with the unified CLI runner:

```bash
# Run Rust tests (via Cargo)
python3 practice.py run backtracking/all_combination_of_size_k rust

# Run Java tests
python3 practice.py run backtracking/all_combination_of_size_k java

# Run Python tests
python3 practice.py run backtracking/all_combination_of_size_k py

# Run TypeScript tests (via Bun or Deno)
python3 practice.py run backtracking/all_combination_of_size_k ts
```

Or test Rust problems directly with standard Cargo commands:
```bash
cargo test --lib backtracking::all_combination_of_size_k
cargo test --lib searching::binary_search
```

---

## 🌙 Moonrepo Monorepo Orchestration

This repository is configured as a multi-language monorepo using **Moonrepo**:
```bash
moon run :test-rust -- backtracking::all_combination_of_size_k
moon run :test-java
moon run :test-py
moon run :test-ts
```

---

## 🗄️ Supabase Platform Database

The repository includes database setup files ready for importing into Supabase:
- `database/schema.sql`: PostgreSQL tables for categories, problems, templates, and user progress.
- `database/seed.sql`: SQL seed statements for all 394 problems.
- `database/seed_problems.json`: Structured JSON metadata for platform frontend ingestion.

---

## 🙏 Attribution

This project is built upon the open-source algorithms repository [TheAlgorithms/Rust](https://github.com/TheAlgorithms/Rust) and [The Algorithms](https://the-algorithms.com/), maintained by the global open-source community.
