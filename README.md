# Exploring Cryptography With Rust

<p align="center">
  <img src="assets/logo.png" width="128" alt="Project Logo">
</p>

<p align="center">
  <!-- GitHub Stars -->
  <a href="https://github.com/carougen/exploring-cryptography-with-rust/stargazers">
    <img src="https://img.shields.io/github/stars/carougen/exploring-cryptography-with-rust?style=for-the-badge&logo=github&label=Stars" alt="Stars" />
  </a>

  <a href="https://discord.gg/HGtAZmvxVT">
    <img src="https://img.shields.io/badge/Discord-Join%20Us-5865F2?logo=discord&logoColor=white&style=for-the-badge" alt="Join Discord" />
  </a>

  <!-- GitHub Watchers -->
  <a href="https://github.com/carougen/exploring-cryptography-with-rust/watchers">
    <img src="https://img.shields.io/github/watchers/carougen/exploring-cryptography-with-rust?style=for-the-badge&logo=github&label=Watchers" alt="Watchers" />
  </a>
</p>

<p align="center">
  <!-- Tests -->
  <a href="https://github.com/carougen/exploring-cryptography-with-rust/actions/workflows/tests.yml">
    <img alt="Tests Passing" src="https://github.com/carougen/exploring-cryptography-with-rust/actions/workflows/tests.yml/badge.svg" />
  </a>
  <!-- Format & Lint -->
  <a href="https://github.com/carougen/exploring-cryptography-with-rust/actions/workflows/format.yml">
    <img alt="Format & Lint" src="https://github.com/carougen/exploring-cryptography-with-rust/actions/workflows/format.yml/badge.svg" />
  </a>
  <!-- Website -->
  <a href="https://github.com/carougen/exploring-cryptography-with-rust/actions/workflows/website.yml">
    <img alt="Docs Deployed" src="https://github.com/carougen/exploring-cryptography-with-rust/actions/workflows/website.yml/badge.svg" />
  </a>
  <!-- API Docs -->
  <a href="https://carougen.github.io/exploring-cryptography-docs/">
    <img alt="API Docs" src="https://img.shields.io/badge/docs-rustdoc-blue" />
  </a>
  <!-- License -->
  <a href="https://github.com/carougen/exploring-cryptography-with-rust/blob/main/LICENSE">
    <img alt="MIT License" src="https://img.shields.io/badge/license-MIT-blue.svg" />
  </a>
</p>

> A collection of concise chapter summaries, exercises, and Rust implementations for leading cryptography books.

### 🗂️ Table of Contents

- [Project Structure](#-project-structure)
- [Prerequisites](#-prerequisites)
- [Quick Start](#-quick-start)
- [Build & Tests](#-build--tests--benchmarks)
- [Running Examples](#-running-examples)
- [Documentation & mdBook](#-documentation--mdbook)
- [Contributing](#-contributing)
- [License](#-license)
- [Contact](#-contact)

---

## 📂 Project Structure

```plaintext
exploring-cryptography-with-rust/                         # Repository root
├── README.md                                             # Project overview and usage
├── crates/                                               # Rust crates grouped by textbook
│   ├── docs/                                             # Used for online docs
│   ├── visualizer/                                       # Used to help visualize things (elliptic curves, ...)
│   ├── proofs_arguments_zeroknowledge/
│   │   ├── fingerprint/                                  # Reed–Solomon encoding and interpolation
│   │   └── freivalds/                                    # Freivalds' algorithm crate
│   ├── introduction_to_modern_cryptography/
│   ├── ...
│   └── serious_cryptography/
├── book/                                                 # mdBook documentation
│   └── src/
│       ├── en/
│       │    ├── SUMMARY.md                               # Table of contents (links to each book)
│       │    ├── intro.md                                 # Project introduction and usage
│       │    ├── proofs_arguments_zeroknowledge/
│       │    │   ├── index.md                             # Overview of the book
│       │    │   ├── examples/                            # Code snippets for documentation
│       │    │   ├── courses/                             # Notes of the book
│       │    │   ├── exercices/                           # Exercices of the book
│       │    │   └── images/                              # (Optional) diagrams, visual content
│       │    ├── introduction_to_modern_cryptography/
│       │    ├── ...
│       │    └── serious_cryptography/
│       └── fr/                                           # Other languages
└── .github/                                              # CI pipeline: formatting, lint, test, mdBook + deploy       
```

---

## 🛠️ Prerequisites

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

---

## 🚀 Quick Start

```bash
git clone https://github.com/carougen/exploring-cryptography-with-rust.git
cd exploring-cryptography-with-rust

cargo build --all
mdbook serve book
```

---

## 📦 Build & Tests & Benchmarks

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

---

## ▶️ Running Examples

### From "Proofs, Arguments, and Zero-Knowledge"

#### fingerprint
```bash
cargo run -p fingerprint --example rs_lagrange
cargo run -p fingerprint --example rs_polynomial
```

#### freivalds
```bash
cargo run -p freivalds --example freivalds
```

---

## 📖 Documentation & mdBook

To serve the docs locally with live reload:
```bash
cd book
mdbook serve
```
Then open: [http://localhost:3000](http://localhost:3000)

---

## 🤝 Contributing

Want to add a new book or exercises? See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## 📜 License

This project is released under the [MIT License](./LICENSE).

---

## 📬 Contact

Maintained by [Nolan CAROUGE](https://github.com/carougen).  
For questions, open an issue or reach out on GitHub.
