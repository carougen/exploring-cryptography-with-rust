# 📖 Introduction

**Exploring Cryptography with Rust** is a hands‑on companion to modern cryptography textbooks, built entirely in Rust. Instead of just reading about algorithms, you can explore self‑contained Rust crates that implement and demonstrate each concept, from error‑correcting codes to probabilistic proofs.

## 🚀 Project Goals

- **Bridge Theory and Practice**  
  Turn textbook pseudocode into real, tested Rust libraries.

- **Learn by Example**  
  Run and modify examples to see how Reed–Solomon encoding, Freivalds’ algorithm, zero‑knowledge proofs, and more really work.

- **Organized by Book**  
  Each crate lives under a folder named for the source text (e.g. _Proofs, Arguments, and Zero‑Knowledge_), letting you browse code chapter by chapter.

## 📚 What’s Inside

- **`fingerprint`**  
  Reed–Solomon encoding & Lagrange interpolation examples (from _Proofs, Arguments, and Zero‑Knowledge_).

- **`freivalds`**  
  Freivalds’ probabilistic matrix multiplication verifier.

- _(…and more crates to come, each tied to a classic crypto book.)_

Each book defines features that includes:
1. A minimal `Cargo.toml`
2. A `src/lib.rs` implementing the core routines
3. An `examples/` directory showing how to generate codewords, test proofs, or verify matrices

## 🛠️ Getting Started

### 🛠️ Prerequisites

- **Rust** (via [rustup](https://rustup.rs/))

This project uses **mdBook v0.4.45** to ensure compatibility with `mdbook-katex`.
- **mdBook** and KaTeX support:
  ```bash
  cargo install mdbook --version 0.4.45
  cargo install mdbook-katex
  
  or

  chmod +x ./setup.sh
  ./setup.sh
  ```

### 🚀 Quick Start

```bash
git clone https://github.com/carougen/exploring-cryptography-with-rust.git
cd exploring-cryptography-with-rust

cargo build --all
mdbook serve book
```

### 📦 Build & Tests

```bash
# Compile all crates
cargo build --all

# Run test for a specific package
cargo test -p freivalds freivalds_test

# Run all tests
cargo test --all

# Generate the HTML for the book
cd book
mdbook build

# Run benchmark for a specific package
cargo bench --package freivalds --bench freivalds_bench

# Run benchmark for all packages
cargo bench --workspace --all-targets
```

### ▶️ Running Examples

#### From "Proofs, Arguments, and Zero-Knowledge"

##### fingerprint
```bash
cargo run -p fingerprint --example rs_lagrange
cargo run -p fingerprint --example rs_polynomial
```

##### freivalds
```bash
cargo run -p freivalds --example freivalds
```

### 📖 Documentation & mdBook

To serve the docs locally with live reload:
```bash
cd book
mdbook serve
```
Then open: [http://localhost:3000](http://localhost:3000)

---

## 🤝 Contributing

Thank you for your interest in contributing! Before you begin, please note that this project is licensed under MIT and adheres to the Rust Community Code of Conduct. By participating, you agree to abide by these terms.

### ➕ Adding a New Book or Chapter

1. Fork the repository and create a new branch
   ```bash
   git checkout -b feature/<book>-short-description
   ```
2. Under `crates/your_book/your_feature`, add your crate following the existing structure:
    - `Cargo.toml`
    - `src/lib.rs`
    - `examples/`
3. In `book/src/<your_language>/your_book>/`, add:
    - `index.md`
    - `courses/`
    - `exercices/`
    - `examples/`
    - `images/`
4. Update `book/src/SUMMARY.md` to include your new chapter links

### 🛠️ Code Quality Checks

- **Formatting**:
  ```bash
  cargo fmt --all -- --check
  ```
- **Linting**:
  ```bash
  cargo clippy --all -- -D warnings
  ```
- **Tests**:  
  Add at least one unit test under `src/`.  
  Example: see [`crates/proofs_arguments_zeroknowledge/fingerprint/src/lib.rs`](https://github.com/carougen/exploring-cryptography-with-rust/blob/main/crates/proofs_arguments_zeroknowledge/fingerprint/src/lib.rs) for a sample test.

### 🚀 Pull Request Process

1. Ensure all CI checks are passing
2. Fill out the PR
3. Submit your pull request
4. Wait for review and address any feedback

### 🌿 Branch Naming

- Features: `feature/<book>-short-description`
- Bugfixes: `fix/<crate>-short-description`

### 📜 Code of Conduct

Please follow the [Rust Community Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). \
Please follow the [Project Code of Conduct](https://github.com/carougen/exploring-cryptography-with-rust/blob/main/CODE_OF_CONDUCT.md).