# Randstat — Implementation Status, Architecture & Test Roadmap

This document tracks the **implementation status, architectural design, and roadmap** across all statistical randomness testing batteries, test stream generators, and WebAssembly distribution packages in the `randstat` workspace.

---

## 1. Executive Summary & Architecture

| Category | Count | Status | Notes |
|---|:---:|:---:|---|
| **Supported Test Suites** | 8 Suites + Quick Suite | ✅ 100% Integrated | ENT, NIST SP 800-22, BSI AIS 31, Dieharder, NIST SP 800-90B, TestU01, PractRand, gjrand, Quick Screen |
| **Total Test Batteries Tracked** | 82 Tests | 32 Fully Implemented / 50 Scaffolding | All 82 tests have zero-alloc `StreamTest` contracts, WASM C-ABI exports, and UI tables |
| **Test Stream Generators** | 11 Generators / 5 Formats | ✅ 100% Implemented | LFSR, De Bruijn, AES/SHA DRBG, ChaCha20, Box-Muller, Poisson, LCG, Xoshiro |
| **Distribution Packages** | 12 Standalone Folders | ✅ 100% Bundled & Minified | `dist/quick`, `dist/ent`, `dist/nist`, `dist/ais31`, `dist/dieharder`, `dist/sp80090b`, `dist/testu01`, `dist/practrand`, `dist/gjrand`, `dist/generators`, `dist/full`, `dist/math` |
| **Memory Model** | `#![no_std]` Zero Heap Alloc | ✅ Enforced | All suites operate entirely on fixed stack state and static WASM buffers |

---

## 2. Granular Implementation Matrix

### ⚡ Quick Screening & Health Diagnostic Suite (`randstat-suite-quick`)
*Fast, single-pass diagnostic battery covering all 5 core mathematical dimensions of randomness.*

| ID | Metric Name | Dimension | Theoretical Ideal | Status | Implementation Notes |
|:---:|---|---|---|:---:|---|
| **QK01** | **Shannon Entropy ($H$)** | Information Density | 8.000000 bits/byte | ✅ Active | Exact 256-bin log2 entropy calculation |
| **QK02** | **Min-Entropy ($H_\infty$)** | Information Density | $\ge 7.000000$ bits/byte | ✅ Active | Worst-case unpredictability $-\log_2(p_{\max})$ |
| **QK03** | **Optimum Compression** | Information Density | 0.00% reduction | ✅ Active | $(8.0 - H) / 8.0 \times 100\%$ redundancy |
| **QK04** | **Byte Uniformity ($\chi^2$)** | Spectral Uniformity | $df = 255$ ($p = 0.50$) | ✅ Active | Pearson $\chi^2$ goodness-of-fit across 256 bins |
| **QK05** | **pochisq Exceedance** | Spectral Uniformity | 50.00% tail prob | ✅ Active | Incomplete gamma $\chi^2$ survival function |
| **QK06** | **Arithmetic Mean** | Spectral Uniformity | 127.5000 | ✅ Active | Running byte average $\frac{1}{N}\sum x_i$ |
| **QK07** | **Monobit Bitwise Frequency** | Bit Distribution | $p \ge 0.01$ | ✅ Active | Bit balance ratio via Normal error function |
| **QK08** | **Runs Oscillations** | Bit Transitions | $p \ge 0.01$ | ✅ Active | 0-to-1 / 1-to-0 transition oscillation count |
| **QK09** | **Poker Test (4-bit)** | Pattern Uniformity | $df = 15$ ($p \ge 0.01$) | ✅ Active | 16-nibble pattern frequency distribution |
| **QK10** | **Monte Carlo $\pi$ Estimation** | Geometric Convergence | 3.14159265... (err $< 3\%$) | ✅ Active | 6-byte 2D circle coordinate convergence |
| **QK11** | **Serial Correlation (Lag-1)** | Sequential Dependence | 0.000000 | ✅ Active | Consecutive byte Pearson correlation $x_i \cdot x_{i+1}$ |

---

### 📊 Fourmilab ENT Battery (`randstat-suite-ent`)
*The classical 1998 John Walker ENT statistical battery.*

| ID | Test Name | Standard / Reference | Status | Details |
|:---:|---|---|:---:|---|
| **ENT01** | **Shannon Entropy** | Walker (1998) | ✅ Active | Full 256-bin byte entropy (bits per byte) |
| **ENT02** | **Optimum Compression** | Walker (1998) | ✅ Active | Theoretical reduction percentage based on Shannon entropy |
| **ENT03** | **Chi-Square Uniformity** | Walker (1998) | ✅ Active | $\chi^2$ statistic ($df=255$) and `pochisq` exceedance % |
| **ENT04** | **Arithmetic Mean** | Walker (1998) | ✅ Active | Average byte value with $\pm 5.0$ guardrails |
| **ENT05** | **Monte Carlo $\pi$** | Walker (1998) | ✅ Active | Spatial point estimation of $\pi$ with error percentage |
| **ENT06** | **Serial Correlation** | Walker (1998) | ✅ Active | Lag-1 Pearson autocorrelation coefficient |

---

### 🛡️ NIST SP 800-22 Rev. 1a Cryptographic Battery (`randstat-suite-nist`)
*NIST Special Publication 800-22 Rev. 1a: A Statistical Test Suite for Random and Pseudorandom Number Generators for Cryptographic Applications.*

| Section | Test Name | Required Length ($n$) | Status | Implementation Details |
|:---:|---|:---:|:---:|---|
| **§2.1** | **Frequency (Monobit) Test** | $n \ge 100$ | ✅ Active | Computes $S_n = \sum (2\epsilon_i - 1)$ and $p = \text{erfc}(S_{\text{obs}} / \sqrt{2n})$ |
| **§2.2** | **Frequency Test within a Block** | $n \ge 100$ ($M=128$) | ✅ Active | Block-wise proportion $\chi^2$ across $M$-bit blocks |
| **§2.3** | **Runs Test** | $n \ge 100$ | ✅ Active | Pre-test monobit check ($\tau = 2/\sqrt{n}$) + run transitions |
| **§2.4** | **Longest Run of Ones in a Block** | $n \ge 128$ | ✅ Active | Block length $M=8, 128, 10^4$ $\chi^2$ bin distribution |
| **§2.5** | **Binary Matrix Rank Test** | $n \ge 38,400$ | ✅ Active | $32 \times 32$ GF(2) Gaussian elimination rank distribution ($R=32, 31, \le 30$) |
| **§2.6** | **Discrete Fourier Transform (Spectral) Test** | $n \ge 1,000$ | ⏳ Scaffolding | Fixed-size zero-alloc Radix-2 FFT spectral peak evaluator |
| **§2.7** | **Non-overlapping Template Matching Test** | $n \ge 10^6$ | ⏳ Scaffolding | $m$-bit template search ($m=9$) across $N$ blocks |
| **§2.8** | **Overlapping Template Matching Test** | $n \ge 10^6$ | ⏳ Scaffolding | $m$-bit template overlap frequency $\chi^2$ |
| **§2.9** | **Maurer's "Universal Statistical" Test** | $n \ge 387,840$ | ⏳ Scaffolding | Table-based distance entropy calculation ($L=7, Q=1280$) |
| **§2.10** | **Linear Complexity Test** | $n \ge 10^6$ | ⏳ Scaffolding | Berlekamp-Massey algorithm for LFSR connection polynomials |
| **§2.11** | **Serial Test** | $n \ge 10^3$ | ⏳ Scaffolding | Overlapping $m$-bit pattern $\psi^2$ statistics ($\Delta \psi^2, \Delta^2 \psi^2$) |
| **§2.12** | **Approximate Entropy Test** | $n \ge 10^3$ | ⏳ Scaffolding | $m$-bit vs $(m+1)$-bit Pincus approximate entropy ($\text{ApEn}(m)$) |
| **§2.13** | **Cumulative Sums (Cusum) Test** | $n \ge 100$ | ⏳ Scaffolding | Random walk maximum excursion from origin (Forward/Backward) |
| **§2.14** | **Random Excursions Test** | $n \ge 10^6$ | ⏳ Scaffolding | 8 state visits ($x = \pm 1, \pm 2, \pm 3, \pm 4$) in zero-crossing cycles |
| **§2.15** | **Random Excursions Variant Test** | $n \ge 10^6$ | ⏳ Scaffolding | 18 state visits ($x \in [-9, 9] \setminus \{0\}$) via $\text{erfc}$ |

---

### 🇩🇪 BSI AIS 20 / AIS 31 Physical TRNG Suite (`randstat-suite-ais31`)
*German Federal Office for Information Security criteria for physical true random number generators (PTG.1–PTG.3).*

| Criteria | Test Name | Standard Sample Size | Status | Implementation Details |
|:---:|---|:---:|:---:|---|
| **T0** | **Disjointness Test** | 48 sub-blocks | ⏳ Scaffolding | Tests whether $N$ consecutive blocks contain duplicates |
| **T1** | **Monobit Test** | 20,000 bits | ✅ Active | Passes if count of ones $n_1 \in [9654, 10346]$ |
| **T2** | **Poker Test** | 20,000 bits ($k=4$) | ✅ Active | Passes if $\chi^2$ across 16 4-bit nibbles $X \in [1.03, 57.4]$ |
| **T3** | **Runs Test** | 20,000 bits | ✅ Active | 6 run-length bins for zeros and ones within AIS 31 bounds |
| **T4** | **Long Run Test** | 20,000 bits | ✅ Active | Passes if no run of zeros or ones exceeds length $\ge 34$ |
| **T5** | **Autocorrelation Test** | 20,000 bits ($\tau \in [1, 5000]$) | ⏳ Scaffolding | Cross-correlation between bits at distance $\tau$ |
| **T6** | **Uniform Distribution Test** | $10^5$ bytes | ⏳ Scaffolding | Multi-byte spectral uniformity test |
| **T7** | **Comparative Test** | $10^5$ bytes | ⏳ Scaffolding | Comparative distribution across successive samples |
| **T8** | **Entropy Estimation Test** | $10^6$ bits | ✅ Active | Shannon entropy estimation ($H \ge 7.976$ bits/byte) |

---

### 🎲 Dieharder Battery (`randstat-suite-dieharder`)
*Marsaglia & Brown's Dieharder test suite for PRNG benchmarking.*

| ID | Test Name | Test Type | Status | Implementation Details |
|:---:|---|---|:---:|---|
| **DH01** | **Diehard Birthdays Spacings** | Spacings Distribution | ⏳ Scaffolding | Poisson distribution of duplicate intervals on $[0, 2^{32})$ |
| **DH02** | **Diehard OPERM5** | Permutation Frequency | ⏳ Scaffolding | $\chi^2$ test across $5! = 120$ orderings of 5 consecutive floats |
| **DH03** | **Diehard 3D Spheres** | Spatial Geometry | ⏳ Scaffolding | Minimum radius between $N$ random points in 3D unit cube |
| **DH04** | **Diehard Parking Lot** | Spatial Poisson | ⏳ Scaffolding | Parking successes on $100 \times 100$ square with radius $r=1$ |
| **DH05** | **Diehard 2D Minimum Distance** | Spatial Clustering | ⏳ Scaffolding | Square distance distribution between nearest points in 2D |
| **DH06** | **Diehard 3D Minimum Distance** | Spatial Clustering | ⏳ Scaffolding | Sphere distance distribution between nearest points in 3D |
| **DH07** | **Diehard Squeeze** | Compression Count | ⏳ Scaffolding | Iterations to reduce $2^{31}$ to 1 via float multiplications |
| **DH08** | **Diehard Runs** | Run Lengths | ✅ Active | Up-and-down run lengths via covariance matrix |
| **DH09** | **Diehard Craps** | Markov Game | ⏳ Scaffolding | Win/loss ratio and throws-per-game distribution in Craps |
| **DH10** | **Marsaglia DNA** | Pattern Alignment | ⏳ Scaffolding | 10-letter words formed from 4-letter alphabet (2-bit pairs) |
| **DH11** | **Marsaglia Bitstream** | Overlapping Words | ⏳ Scaffolding | Missing 20-bit words in 2M bitstream |
| **DH12** | **Count the 1s (Stream)** | Byte Weight Markov | ✅ Active | Byte Hamming weights modeled as 5-state Markov chain |

---

### 📐 NIST SP 800-90B Min-Entropy Battery (`randstat-suite-sp800-90b`)
*NIST Special Publication 800-90B: Recommendation for the Entropy Sources Used for Random Bit Generation.*

| ID | Estimator (§6.3) | Estimation Strategy | Status | Details |
|:---:|---|---|:---:|---|
| **90B-01** | **Most Common Value (MCV)** | Frequency Peak | ✅ Active | $H_\infty = -\log_2(\hat{p})$, with upper confidence bound $\hat{p}$ |
| **90B-02** | **Collision Estimator** | Collision Distances | ✅ Active | Geometric distribution of first collision occurrences |
| **90B-03** | **Markov Estimator** | Transition Matrix | ✅ Active | First-order Markov chain conditional probability $P(x_i \mid x_{i-1})$ |
| **90B-04** | **Compression Estimator** | Entropy Bounds | ✅ Active | Maurer-style universal code length dictionary |
| **90B-05** | **t-Tuple Estimator** | Multi-tuple Frequencies | ⏳ Scaffolding | Longest tuple $t \in [1, 35]$ frequency maximization |
| **90B-06** | **Longest Repeated Substring (LRS)** | Substring Repetition | ⏳ Scaffolding | Collision search across overlapping substrings |
| **90B-07** | **Multi-Most Common Value (MMCV)** | Sub-stream Peaks | ⏳ Scaffolding | MCV evaluated across decimation sub-sequences |
| **90B-08** | **Lag Prediction Estimator** | Autoregressive Predictor | ⏳ Scaffolding | Predictability score using $D$-lagged predictors |
| **90B-09** | **Multi-Markov (MultiMMC)** | Higher-order Markov | ⏳ Scaffolding | Multi-order Markov model transition predictability |
| **90B-10** | **LZ78Y Estimator** | LZ Dictionary Predictor | ⏳ Scaffolding | Lempel-Ziv dictionary next-symbol prediction score |

---

### 🧪 TestU01 SmallCrush PRNG Benchmark (`randstat-suite-testu01`)
*L'Ecuyer & Simard (2007) SmallCrush battery for rapid PRNG screening.*

| ID | Test Name | Category | Status | Details |
|:---:|---|---|:---:|---|
| **U01-01** | **smarsa_BirthdaySpacings** | Spacings | ⏳ Scaffolding | Birthday collisions in $2^{21}$ day year ($n=10^4$) |
| **U01-02** | **sknuth_Collision** | Collisions | ✅ Active | Hash bucket collisions in $2^{14}$ bins |
| **U01-03** | **sknuth_Gap** | Recurrence Intervals | ⏳ Scaffolding | Gaps between recurring values in intervals $[\alpha, \beta]$ |
| **U01-04** | **sknuth_SimpPoker** | Patterns | ✅ Active | 4-bit / 8-bit poker hand frequency $\chi^2$ |
| **U01-05** | **sknuth_CouponCollector** | Collector Waiting Times | ⏳ Scaffolding | Time steps required to collect full set of $d$ integers |
| **U01-06** | **sknuth_MaxOft** | Extreme Values | ⏳ Scaffolding | Distribution of $M = \max(U_1, \dots, U_t)$ power law |
| **U01-07** | **svar_WeightDistrib** | Hamming Distribution | ✅ Active | Binomial weight distribution of consecutive bit segments |
| **U01-08** | **smarsa_MatrixRank** | Linear Algebra | ✅ Active | Binary matrix rank ($30 \times 30$, $60 \times 60$) |
| **U01-09** | **sstring_HammingIndep** | Autocorrelation | ⏳ Scaffolding | Correlation between Hamming weights at lag $L$ |
| **U01-10** | **sstring_Run** | Run Distribution | ✅ Active | Longest run and run-length counts in bitstreams |

---

### ⚡ PractRand PRNG Battery (`randstat-suite-practrand`)
*Chris Doty-Humphrey's PractRand high-throughput streaming benchmark.*

| ID | Test Name | Stage / Octave | Status | Details |
|:---:|---|---|:---:|---|
| **PR01** | **Gap-16:B** | `[Low1/8]` | ⏳ Scaffolding | 16-bit word gap recurrence test |
| **PR02** | **FPF-16:B** | `[Low1/8]` | ⏳ Scaffolding | Floating-Point Frequency 16-bit fold test |
| **PR03** | **BCFN(2+0,13/64)** | `[Low1/8]` | ⏳ Scaffolding | Block Count Frequency Non-uniformity |
| **PR04** | **BCFN(2+1,13/64)** | `[Low1/8]` | ⏳ Scaffolding | Block Count Frequency Non-uniformity (Lagged) |
| **PR05** | **DC6-9x1Bytes-1** | `[Low4/8]` | ⏳ Scaffolding | 6-bit / 9-byte distance correlation |
| **PR06** | **BRank(12)** | `[Low4/8]` | ✅ Active | 12-bit binary matrix rank test |
| **PR07** | **FPF-8:all64k** | `[Low8/8]` | ⏳ Scaffolding | 8-bit full 64K table frequency folding |
| **PR08** | **Dist-64x2:g** | `[Low8/8]` | ⏳ Scaffolding | 64-bit distance 2D distribution |
| **PR09** | **Gap-8:all64k** | `[Low8/8]` | ⏳ Scaffolding | 8-bit 64K gap recurrence distribution |
| **PR10** | **AutoCor-64** | `[Low8/8]` | ✅ Active | 64-bit multi-lag autocorrelation battery |

---

### 🔬 gjrand PRNG Battery (`randstat-suite-gjrand`)
*David Blackman's gjrand lightweight PRNG testing suite.*

| ID | Test Name | Word Size | Status | Details |
|:---:|---|---|:---:|---|
| **GJ01** | **mcoll16** | 16-bit | ✅ Active | 16-bit word collision test |
| **GJ02** | **mcoll32** | 32-bit | ✅ Active | 32-bit word collision test |
| **GJ03** | **mprob16** | 16-bit | ✅ Active | 16-bit byte probabilities uniformity |
| **GJ04** | **mprob32** | 32-bit | ✅ Active | 32-bit probabilities uniformity |
| **GJ05** | **mdist16** | 16-bit | ⏳ Scaffolding | 16-bit inter-word distance distribution |
| **GJ06** | **mdist32** | 32-bit | ⏳ Scaffolding | 32-bit inter-word distance distribution |
| **GJ07** | **mgap16** | 16-bit | ⏳ Scaffolding | 16-bit gap distribution between matching words |
| **GJ08** | **mgap32** | 32-bit | ⏳ Scaffolding | 32-bit gap distribution between matching words |
| **GJ09** | **mrun16** | 16-bit | ✅ Active | 16-bit ascending/descending run distribution |
| **GJ10** | **mrun32** | 32-bit | ✅ Active | 32-bit ascending/descending run distribution |

---

## 3. Test Stream Generators & Output Formatters

| Family | Algorithm | Parameters / Configuration | Status |
|---|---|---|:---:|
| **Ramp / Sequence** | Linear Counter | Increment step, start offset, modulo limit | ✅ Active |
| **Constant / Fixed** | Zeroes / Ones / Constant Byte | Arbitrary repeating byte `0x00`, `0xFF`, `0xAA`, `0x55` | ✅ Active |
| **Linear Feedback (LFSR)** | Primitive Polynomial LFSR | Polynomial orders 3, 4, 5, 7, 8, 16, 32, 64, 128 | ✅ Active |
| **De Bruijn Cycles** | Maximal Sequence Generators | Complete de Bruijn cycles of orders $n = 2, 3, 4$ | ✅ Active |
| **Linear Congruential (LCG)** | Knuth / Numerical Recipes LCG | $a = 6364136223846793005, c = 1442695040888963407$ | ✅ Active |
| **Xoshiro** | Xoshiro256** | High-performance 256-bit state PRNG | ✅ Active |
| **Gaussian Distribution** | Box-Muller Transformation | Mean $\mu = 127.5$, std dev $\sigma = 32.0$ | ✅ Active |
| **Poisson Distribution** | Knuth Inversion / Transform | Parameter $\lambda = 127.0$ | ✅ Active |
| **NIST SP 800-90A DRBG** | AES-128 CTR_DRBG | Cryptographically secure deterministic random bit generator | ✅ Active |
| **NIST SP 800-90A DRBG** | SHA-256 Hash_DRBG | SHA-256 state-transition deterministic bit generator | ✅ Active |
| **ChaCha Keystream** | ChaCha20 | RFC 8439 256-bit key + 96-bit nonce stream cipher | ✅ Active |

### Export Formatters
- ✅ **Raw Binary (`.bin`)**: Pure unencoded binary stream.
- ✅ **Decimal ASCII (`.txt`)**: Byte-delimited integer sequence `[0, 255]`.
- ✅ **Bitstream (`.bits`)**: ASCII sequence of `0` and `1` characters.
- ✅ **Hexadecimal (`.hex`)**: Lowercase hex pairs `00 1a ff ...`.
- ✅ **Normalized Floats (`.csv`)**: IEEE-754 floats in $[0.0, 1.0]$.

---

## 4. Next Development Milestones

1. **Discrete Fourier Transform (DFT) Port**:
   - Port a fixed $N=1024 / 4096$ zero-allocation Cooley-Tukey Radix-2 FFT kernel into `randstat-core::math::fft` to complete NIST SP 800-22 §2.6.
2. **Berlekamp-Massey Kernel**:
   - Implement bit-level Berlekamp-Massey algorithm for NIST §2.10 Linear Complexity with $M=500$ and $M=1000$ block sizes.
3. **Multi-Lag AutoCorrelation Engine**:
   - Generalize circular buffer autocorrelation across lags $\tau \in [1, 5000]$ for AIS 31 T5 and PractRand AutoCor-64.
4. **NIST SP 800-90B Full t-Tuple and LRS Evaluators**:
   - Implement fixed-depth sliding window suffix array for Longest Repeated Substring and multi-tuple predictors.
