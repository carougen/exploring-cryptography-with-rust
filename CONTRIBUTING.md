# 🤝 Contributing to Exploring Cryptography with Rust

Thank you for your interest in contributing! Before you begin, please note that this project is licensed under MIT and adheres to the Rust Community Code of Conduct. By participating, you agree to abide by these terms.

---

## 🆘 Getting Help

If you have questions or need assistance, feel free to:
- Open an issue in this repository
- Join our community chat on [Discord](https://discord.gg/HGtAZmvxVT)

---

## ➕ Adding a New Book or Chapter

1. Fork the repository and create a new branch
   ```bash
   git checkout -b feature/<book>-short-description
   ```
2. Under `crates/your_book/your_feature`, add your crate following the existing structure:
   - `Cargo.toml`
   - `src/lib.rs`
   - `examples/`
3. In `book/src/<your_language>/<your_book>/`, add:
   - `index.md`
   - `courses/`
   - `exercices/`
   - `examples/`
   - `images/`
4. Update `book/src/SUMMARY.md` to include your new chapter links

---

## 🛠️ Code Quality Checks

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
  Example: see [`crates/proofs_arguments_zeroknowledge/fingerprint/src/lib.rs`](/crates/proofs_arguments_zeroknowledge/fingerprint/src/lib.rs) for a sample test.

---

## 🚀 Pull Request Process

1. Ensure all CI checks are passing
2. Fill out the PR
3. Submit your pull request
4. Wait for review and address any feedback

---

## 🌿 Branch Naming

- Features: `feature/<book>-short-description`
- Bugfixes: `fix/<crate>-short-description`

---

## 📜 Code of Conduct

Please follow the [Rust Community Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).\
Please follow the [Project Code of Conduct](https://github.com/carougen/exploring-cryptography-with-rust/blob/main/CODE_OF_CONDUCT.md).

---

Thank you for helping improve this project! 🚀
