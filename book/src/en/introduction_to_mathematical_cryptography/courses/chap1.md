# 🏛️ Chapter 1 — An Introduction to Cryptography
_Source: An Introduction to Mathematical Cryptography : Hoffstein, Pipher, Silverman (2nd ed.)_  
_Last updated 2025-04-27_

---
## 🚩 Navigation

| § | Title                                                           | Emoji |
|---|-----------------------------------------------------------------|:--:|
| 1.1 | [Simple Substitution Ciphers](#-11-simple-substitution-ciphers) | 🔤 |
| 1.2 | [Divisibility & GCD](#-12-divisibility--gcd)                    | ➗ |
| 1.3 | [Modular Arithmetic](#-13-modular-arithmetic)                   | 🔄 |
| 1.4 | [Primes & Finite Fields](#-14-primes--finite-fields)            | 🔺 |
| 1.5 | [Powers & Primitive Roots](#-15-powers--primitive-roots)        | ⚡ |
| 1.6 | [Historical Milestones](#-16-historical-milestones)             | 📜 |
| 1.7 | [Symmetric vs Asymmetric](#-17-symmetric-vs-asymmetric)         | 🔐 |

---

## 🔤 1.1 Simple Substitution Ciphers
### 👑 1.1.1 Caesar / shift cipher

Assign letters $a\mapsto0,\;b\mapsto1,\;\dots,\;z\mapsto25$.

To encrypt a letter of value $x\in\{0,\dots,25\}$ with key $k\in\mathbb Z$:

$$
\text{enc}_k(x) =
\begin{cases}
x + k, & \text{if } x + k \le 25,\\
x + k - 26, & \text{if } x + k > 25.
\end{cases}
$$

To decrypt:

$$
\text{dec}_k(x) =
\begin{cases}
x - k, & \text{if } x - k \ge 0,\\
x - k + 26, & \text{if } x - k < 0.
\end{cases}
$$

**Example ($k=2$).** Number the alphabet $A\mapsto0,\;B\mapsto1,\;\dots,\;Z\mapsto25$. With key $k=2$, encryption sends

$$
x \;\mapsto\;
\begin{cases}
x + 2, & \text{if } x + 2 \le 25,\\
x + 2 - 26, & \text{if } x + 2 > 25,
\end{cases}
$$

so in letters:

$$
A\to C,\quad
B\to D,\quad
C\to E,\quad
\dots,\quad
Y\to A,\quad
Z\to B.
$$

---

### 🔀 1.1.2 Mono-alphabetic cipher

A **Mono-alphabetic cipher** fixes a permutation $\sigma$ of the 26 letters. For instance, one choice of $\sigma$ is

$$
A\mapsto Q,\quad
B\mapsto W,\quad
C\mapsto E,\quad
D\mapsto R,\quad
E\mapsto T,\quad
\dots,\quad
Z\mapsto B.
$$

Since there are $26!$ possible permutations, the key-space has size

$$
26! \;\approx\; 4.03\times10^{26}
\quad(\text{about }88\text{ bits}).
$$

---

### 📊 1.1.3 Why statistics leak

English single‑letter frequencies (approx.):

| E | T | A | O | N | I | S | R | H | L |
|---|---|---|---|---|---|---|---|---|---|
|13 %| 9 %| 8 %| 7 %| 7 %| 6 %| 6 %| 6 %| 5 %| 4 %|

Bigrams **th 17 %**, **he 13 %** dominate. A fixed substitution preserves these peaks, so histograms expose the key.

---

### 🛡️ 1.1.4 Defensive evolution

| Era | Technique | Defence idea                             | Ultimate weakness |
|-----|-----------|------------------------------------------|-------------------|
| 16ᵗʰ c. | Vigenère | Changing shift                           | Key length leaks  |
| 1920s | Enigma | Rotor permutation stream                 | Operator mistakes |
| Today | AES / ChaCha | Ciphertext indistinguishable from random | Only brute force  |

---

## ➗ 1.2 Divisibility & GCD
### 🔢 1.2.1 Toolkit

- **Divisibility**  
  We say an integer $a$ **divides** an integer $b$, written  
  $$a \mid b \quad\Longleftrightarrow\quad \exists k \in \mathbb{Z}: b = a k.$$
  Here, $k$ is the **quotient**, and the absence of remainder is crucial.  
  For example, $3 \mid 12$ since $12 = 3 \cdot 4$, but $5 \nmid 12$.

- **Greatest Common Divisor (GCD)**  
  For nonzero integers $a,b$, the **greatest common divisor** $\gcd(a,b)$ is the largest positive integer $d$ satisfying
  $$d \mid a \quad\text{and}\quad d \mid b.$$
  Equivalently,
  $$d = \gcd(a,b)
  \quad\Longleftrightarrow\quad
  d\mid a,\; d\mid b,\;
  \text{and for all } c\in\mathbb{Z},\; c\mid a,\; c\mid b \;\implies\; c \le d.$$

- **Bézout’s Identity**  
  There exist integers $x,y$ (the **Bézout coefficients**) such that
  $$a x + b y = \gcd(a,b).$$
  This identity shows $\gcd(a,b)$ is the smallest positive linear combination of $a$ and $b$.

- **Modular Inverse**  
  If $\gcd(a,m)=1$, Bézout’s identity yields
  $$a x + m y = 1 \;\Longrightarrow\; a x \equiv 1 \pmod{m},$$
  so $x \bmod m$ is the **multiplicative inverse** of $a$ modulo $m$.

---

### ➗ 1.2.2 Euclidean Algorithm (Worked)

The **Euclidean algorithm** computes $\gcd(a,b)$ via repeated division with remainder.

#### Example: Compute $\gcd(2024,748)$

$$
\begin{aligned}
2024 &= 748 \cdot 2 + 528,\\
748  &= 528 \cdot 1 + 220,\\
528  &= 220 \cdot 2 + 88,\\
220  &= 88 \cdot 2 + 44,\\
88   &= 44 \cdot 2 + 0.
\end{aligned}
$$

Once the remainder is $0$, the last nonzero remainder is
$$\gcd(2024,748)=44.$$

> **Complexity**: Lamé’s theorem (1844) shows the number of division steps is $O(\log b)$, in fact fewer than $5$ times the decimal digits of $b$.

---

### 🧮 1.2.3 Extended Euclidean Algorithm

To find Bézout coefficients $s,t$ such that
$$2024\,s + 748\,t = 44,$$
we maintain sequences $(r_i,s_i,t_i)$ with
$$r_i = 2024\,s_i + 748\,t_i.$$

| $r$   | $q$ | $s$  | $t$  |
|:-----:|:---:|:----:|:----:|
| 2024  |     | 1    | 0    |
| 748   | 2   | 0    | 1    |
| 528   | 1   | 1    | −2   |
| 220   | 2   | −1   | 3    |
| 88    | 2   | 3    | −8   |
| 44    |     | −7   | 19   |

Thus,
$$-7 \cdot 2024 + 19 \cdot 748 = 44.$$

---

## 🔄 1.3 Modular Arithmetic
### 📦 1.3.1 Ring Structure of $\mathbb{Z}/m\mathbb{Z}$

- **Residue Classes.**  
  We denote by $[0], [1], \dots, [m-1]$ the equivalence classes of integers modulo $m$:
  $$
  [a] = \{a + k m : k \in \mathbb{Z}\}.
  $$
  **Addition** and **multiplication** are defined with wrap‑around:
  $$
  [a] + [b] = [a + b],
  \quad
  [a]\cdot[b] = [a b],
  $$
  where sums and products are reduced modulo $m$ to lie in $\{0,1,\dots,m-1\}$.

- **Examples ($m=26$).**  
  $$
  [17] + [10] = [27] = [1],
  \quad
  [5]\cdot[5] = [25] = [-1].
  $$
  The set $\{[0],[1],\dots,[m-1]\}$ forms a **ring**, commonly denoted $\mathbb{Z}/m\mathbb{Z}$, with:
  - **Additive identity**: $[0]$, since $[a]+[0]=[a]$.
  - **Multiplicative identity**: $[1]$, since $[a]\cdot[1]=[a]$.
  - **Additive inverses**: $[a] + [m-a] = [0]$.

---

### 🧮 1.3.2 Euler’s Theorem and Fermat’s Little Theorem

- **Euler’s Totient Function** $\varphi(m)$ equals the number of integers in $\{1,2,\dots,m\}$ that are coprime to $m$.

- **Euler’s Theorem.**  
  If $\gcd(a,m)=1$, then
  $$
  a^{\varphi(m)} \equiv 1 \pmod{m}.
  $$

- **Fermat’s Little Theorem.**  
  If $p$ is prime and $p\nmid a$, then $\varphi(p)=p-1$ and
  $$
  a^{p-1} \equiv 1 \pmod{p}.
  $$

---

### ⚡️ 1.3.3 Efficient Exponentiation

- **Binary Expansion of the Exponent.**  
  Any exponent $e$ admits a binary representation
  $$
  e = \sum_{i=0}^{k} e_i \,2^i,
  \quad e_i\in\{0,1\}.
  $$

- **Square‑and‑Multiply Concept.**
  1. Precompute successive squares: $b^{2^0}, b^{2^1}, b^{2^2}, \dots$ modulo $m$.
  2. Multiply together those squares corresponding to the 1‑bits of $e$.
  3. Total modular multiplications: about $2\lfloor \log_2(e)\rfloor$.

- **Illustration.**  
  To compute $b^{13}\bmod m$:
  $$
  13_{10} = 1101_2 = 2^3 + 2^2 + 2^0,
  $$
  so
  $$
  b^{13} = b^{8}\;\cdot\;b^{4}\;\cdot\;b^{1},
  $$
  each obtained via repeated squaring and selected multiplies.
  This reduces the number of multiplications from $13$ (naïve) to roughly $4$ squares + $3$ multiplies = $7$ total.

---

## 🔺 1.4 Primes & Finite Fields
### 🔢 1.4.1 Prime Number
- **Definition.**  
  An integer $p > 1$ is called **prime** if its only positive divisors are $1$ and $p$. Equivalently,  
  $$  
  p\text{ is prime}  
  \quad\Longleftrightarrow\quad  
  \forall\,d\in\mathbb{Z}_{>0},\;d\mid p\;\implies\;(d=1\ \text{or}\ d=p).  
  $$

---

### 🔍 1.4.2 Generating Primes with the Sieve of Eratosthenes
- **Conceptual Overview.**
  1. Start with a list of integers from $2$ up to $n$.
  2. Repeatedly mark (“sieve out”) multiples of each prime found, beginning with $2$.
  3. The unmarked numbers that remain are the primes $\le n$.

- **Example ($n=30$).**
  - Begin: $2,3,4,5,6,7,\dots,30$.
  - Mark multiples of $2$: $4,6,8,\dots,30$.
  - Next unmarked is $3$, mark its multiples: $6,9,12,\dots,30$.
  - Continue up to $\lfloor\sqrt{n}\rfloor$.
  - Primes found: $2,3,5,7,11,13,17,19,23,29$.

- **Efficiency.**
  - Time complexity: $O(n\log\log n)$.
  - Space: $O(n)$.
  - Generates all primes up to $n$ efficiently, and for $n=10^6$, runs in well under a second in optimized implementations.

---

### ⚗️ 1.4.3 The Finite Field $\mathbb{F}_p$

- **Definition.**  
  For a prime $p$, the set $\{0,1,\dots,p-1\}$ with addition and multiplication **modulo $p$** forms a **field**, denoted $\mathbb{F}_p$.

- **Field Axioms.**
  - **Additive group**: $(\mathbb{F}_p,[+])$ is cyclic of order $p$.
  - **Multiplicative group**: $(\mathbb{F}_p^\times,[\cdot])$ is cyclic of order $p-1$.
  - Every nonzero element $a$ has a unique inverse $a^{-1}$ satisfying $a\,a^{-1}\equiv1\pmod p$.

- **Computing Inverses (Fermat’s Little Theorem).**  
  If $p\nmid a$, then
  $$
  a^{p-1}\equiv1\pmod p
  \quad\Longrightarrow\quad
  a^{p-2}\equiv a^{-1}\pmod p.
  $$

- **Cryptographic Choice.**
  - Modern cryptosystems (e.g. Diffie–Hellman, elliptic curves) pick $p\approx2^{256}$ to achieve roughly 128‑bit security against brute‑force attacks.

---

## ⚡ 1.5 Powers & Primitive Roots
### 🔢 1.5.1 Multiplicative Order

- **Definition.**  
  For a prime $p$ and an integer $a$ with $\gcd(a,p)=1$, the **order** of $a$ mod $p$, denoted $\mathrm{ord}_p(a)$, is the smallest positive integer $k$ such that
  $$
  a^k \equiv 1 \pmod{p}.
  $$

- **Properties.**
  1. $\mathrm{ord}_p(a)$ divides $p-1$, since by Fermat’s little theorem $a^{p-1}\equiv1\pmod p$.
  2. The powers $a^1, a^2,\dots,a^{\mathrm{ord}_p(a)}$ cycle through a subgroup of $(\mathbb{Z}/p\mathbb{Z})^\times$ of size $\mathrm{ord}_p(a)$.

---

### 🌱 1.5.2 Primitive Roots (Generators)

- **Primitive Root.**  
  A **primitive root** modulo $p$ is an element $g$ whose order is exactly $p-1$:
  $$
  \mathrm{ord}_p(g) = p-1.
  $$
  Equivalently, the powers $g^1, g^2, \dots, g^{p-1}$ produce **all** nonzero residues $1,2,\dots,p-1$ in some order.

- **Existence.**  
  For every prime $p$, there exists at least one primitive root modulo $p$.
  - To **find** a primitive root $g$, one factors $p-1 = q_1^{e_1}\cdots q_r^{e_r}$ and tests candidates $g=2,3,4,\dots$ until
    $$
    g^{\frac{p-1}{q_i}} \not\equiv 1 \pmod{p}
    \quad
    \text{for all prime divisors }q_i\mid (p-1).
    $$

---

## 📜 1.6 Historical Milestones

A brief overview of landmark ciphers throughout history and the key techniques that led to their cryptanalysis.

| Year   | Cipher                 | Break Highlight               |
|--------|------------------------|-------------------------------|
| 50 BCE | **Caesar Shift**       | Brute‑force search of 26 shifts |
| 830    | **Arab Cryptanalysis** | Frequency analysis of letters |
| 1553   | **Vigenère Cipher**    | Index of coincidence          |
| 1917   | **Zimmermann Telegram**| Exploitation of linguistic patterns |
| 1939   | **Enigma Machine**     | Mechanical Bombe‑based search |
| 1944   | **Lorenz Cipher**      | Colossus computer and statistical methods |

---

### 🏛️ 50 BCE — Caesar Shift

- **Algorithm:** Shift each letter by a fixed offset (typically 3).
- **Cryptanalysis:** A brute‑force attempt of all 26 possible shifts quickly reveals the plaintext.
- **Significance:** One of the earliest recorded uses of substitution ciphers.

---

### 🔍 830 — Arab Cryptanalysis

- **Cryptographer:** Al-Kindi (Arab mathematician).
- **Technique:** Developed **frequency analysis**, observing that certain letters appear more often in a language.
- **Impact:** Demonstrated that simple substitution ciphers are insecure.

---

### 🔒 1553 — Vigenère Cipher

- **Cipher:** Polyalphabetic substitution using a repeating keyword.
- **Cryptanalysis:** The **index of coincidence** method by Babbage and later Kasiski reveals the keyword length, reducing it to multiple Caesar ciphers.
- **Legacy:** Once called “le chiffre indéchiffrable,” it was eventually broken and paved the way for modern cryptanalysis.

---

### 🕵️ 1917 — Zimmermann Telegram

- **Context:** World War I diplomatic communication.
- **Attack Method:** Analysts used **linguistic context** and known-plaintext guesses to reconstruct parts of the message.
- **Historical Outcome:** Its decryption helped bring the United States into the war.

---

### ⚙️ 1939 — Enigma Machine

- **Device:** Electro‑mechanical rotor cipher machine used by Germany.
- **Decryption:** The **Bombe** machine, designed by Alan Turing and colleagues at Bletchley Park, automated key search using cribs and known patterns.
- **Effect:** Allied forces gained critical intelligence during WWII.

---

### 🖥️ 1944 — Lorenz Cipher

- **System:** High‑level teleprinter cipher (called “Tunny” by British codebreakers).
- **Breakthrough:** The **Colossus** computer exploited statistical weaknesses and wheel‑pattern analysis.
- **Importance:** One of the first large‑scale electronic computers, marking the birth of modern computing.

---

## 🔐 1.7 Symmetric vs Asymmetric
### 🔑 1.7.1 Symmetric Ciphers

- **Definition:** Both sender and receiver share the **same secret key** for encryption and decryption.
- **Characteristics:**
  - Very efficient: suitable for encrypting large volumes of data.
  - Key distribution challenge: secure key exchange is required before use.
- **Examples:**
  - **Block ciphers** like AES (Advanced Encryption Standard) operate on fixed-length blocks (e.g. 128 bits).
  - **Stream ciphers** generate a pseudorandom keystream to XOR with the plaintext.

---

### 🌍 1.7.2 Asymmetric Ciphers

- **Definition:** Uses a **public key** for encryption and a **private key** for decryption.
- **Characteristics:**
  - Simplifies key distribution: public key can be openly shared.
  - Computationally heavier: used typically to secure small messages or keys.
- **Examples:**
  - **RSA** (Rivest–Shamir–Adleman).
  - **Diffie–Hellman key exchange** for establishing shared secrets.
  - **Elliptic curve** variants (e.g.\ ECDH, ECDSA).

---

### 🔄 1.7.3 Hybrid Encryption

- **Concept:** Combine strengths of both systems:
  1. Generate a random **session key** for a fast symmetric cipher.
  2. Encrypt the session key using the recipient’s **public key**.
  3. Encrypt the bulk data with the symmetric cipher using the session key.
- **Benefit:** Achieves both performance and secure key distribution.
