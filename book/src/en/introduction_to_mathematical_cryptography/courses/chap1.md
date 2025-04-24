# 🏛️ 1.1 — Simple Substitution Ciphers

**Section 1.1** of *An Introduction to Mathematical Cryptography* introduces classical cryptography through **simple substitution ciphers**, highlighting their historical context and cryptanalytic vulnerabilities.

---

## 🗝️ Core Concepts

| Concept                        | Explanation                                     |
|--------------------------------|-------------------------------------------------|
| **Caesar (Shift) Cipher**      | Each letter shifted by a fixed offset (e.g., +5) around the alphabet. |
| **General Substitution Cipher**| Every plaintext letter maps uniquely to a ciphertext letter (one-to-one substitution). |
| **Key Space Size**             | 26 letters permuted ⇒ `26! ≈ 4.03 × 10²⁶` keys (massive yet insecure).|
| **Frequency Analysis**         | Breaking the cipher by statistical frequency of letters and pairs (e.g., 'e', 't', 'th', 'he'). |
| **Security Principle**         | Security depends on the strongest available attack, not brute-force attempts.|

---

## 📖 Example Explained: Caesar Cipher

- Julius Caesar encrypted military messages by shifting letters (e.g., shift by 5).
- Example: Ciphertext: JSJRD KFQQNSL... translates to Plaintext: ENEMY FALLING...
- Visualized by a cipher wheel (alphabet circle).

---

## 🔍 Cryptanalysis Walkthrough (298-letter ciphertext)

A real-world example demonstrates frequency analysis:

1. **Letter Frequency**: Guess common letters (`e, t, a, o, n`).
2. **Bigram Analysis**: Identify frequent pairs (`th`, `he`).
3. **Iterative Guessing**: Gradually reconstruct plaintext from fragments.
4. **Result**: Entire plaintext recovered efficiently without brute force:

---

## 🧠 Key Takeaways

- Understand **mapping tables** for encryption/decryption.
- Calculate permutations (`n!`) for alphabet substitutions.
- Practice **manual cryptanalysis** using frequency and bigram analysis.
- Recognize why a large key-space alone doesn't guarantee security.

---

# 🧮 Section 1.2 — Divisibility & Greatest Common Divisors

**Section 1.2** of *An Introduction to Mathematical Cryptography* builds the arithmetic
bedrock—divisibility, Euclid’s algorithm and Bézout identities—on which every
modern public‑key scheme in the book will stand.

---

## 1 Divisibility basics

| Notation          | Meaning                                                                                  |
|-------------------|------------------------------------------------------------------------------------------|
| $a \mid b$        | *a* **divides** *b* ⟺ $\exists\,k\in\mathbb Z : b = a\,k$                                 |
| $a \nmid b$       | *a* does **not** divide *b*                                                               |
| $a \parallel b$   | *a* divides *b* **exactly once** (i.e. $a\mid b$ but $a^{2}\nmid b$)                      |

### Handy rules

* **Transitivity** If $a\mid b$ and $b\mid c$ ⇒ $a\mid c$.
* **Cancellation** If $a\mid b$ and $b\mid a$ ⇒ $a = \pm b$.
* **Linear closure** If $a\mid b$ and $a\mid c$ ⇒ $a\mid(b\pm c)$.
* **Prime trick (Euclid’s lemma)** If a prime $p\mid ab$ then $p\mid a$ *or* $p\mid b$.

---

## 2 Greatest common divisor

For non‑zero integers $a,b$ the **greatest common divisor**

$$d = \gcd(a,b)$$

is the unique $d>0$ satisfying

1. $d\mid a$ and $d\mid b$
2. if $c\mid a$ and $c\mid b$ then $c\mid d$.

*Convention* $\gcd(a,0)=|a|$ (we always return the absolute value).

> **Example** $\gcd(748,2024)=44$.

---

### 2.1 Euclidean algorithm — computing $\gcd$

The algorithm relies on the invariant

$$
\gcd(a,b)=\gcd\bigl(b,\,a\bmod b\bigr)\qquad(a\ge b>0).
$$

Since the remainder is *strictly smaller* than $b$, the pair shrinks at
every turn.

```text
function gcd(a, b):              # returns |gcd(a,b)|
    while b ≠ 0:                 # repeat until remainder is zero
        r  ← a mod b             # 0 ≤ r < b
        (a, b) ← (b, r)          # shift window left
    return |a|                   # a now holds ±gcd, take absolute value
```

*Complexity* Each step reduces the pair by at least the golden‑ratio factor 
$(\varphi \approx 1.618)$ in the *average* case, so the loop count is $O(\log\min\{a,b\})$—blazingly fast even for hundreds of digits.

---

### 2.2 Extended Euclidean algorithm — Bézout coefficients

By tracking two extra registers $(x,y)$ we maintain

$$
x\,a_{\text{cur}} + y\,b_{\text{cur}} = \gcd(a,b)
$$

throughout the loop.  When $b$ hits 0 we obtain integers $x,y$ such that

$$
a\,x + b\,y = \gcd(a,b)\qquad\text{(Bézout identity).}
$$

*Crypto payoff* If $\gcd(a,n)=1$, the coefficient $x$ is the modular inverse
$a^{-1}\pmod n$, crucial for RSA ($d \equiv e^{-1} \pmod{\varphi(n)}$),
Diffie‑Hellman, etc.

---

## 3 Worked example

Compute $\gcd(252,198)$ **and** a Bézout representation.

| Step | $a$ | $b$ | $a\bmod b$ |
|------|-----|-----|-----------|
| 1    | 252 | 198 | 54 |
| 2    | 198 | 54  | 36 |
| 3    | 54  | 36  | 18 |
| 4    | 36  | 18  | 0  |

So $\gcd=18$.

_Rewind_:

$$
\begin{aligned}
18 &= 54 - 36\\
&= 54 - (198-54) = 2\cdot54 - 198\\
&= 2(252 - 198) - 198 = 2\cdot252 - 3\cdot198.
\end{aligned}
$$

Hence Bézout coefficients $(x,y)=(2,-3)$.

---

## 4 Why this matters for crypto

* **RSA** — private exponent $d$ is the inverse of $e$ mod $\varphi(n)$.
* **Diffie‑Hellman / ECC** — modular inverses underpin scalar arithmetic.
* **Lattice & factorisation algorithms** — gcd is everywhere.

---

*Last updated 2025‑04‑24 — PRs & suggestions welcome!*
