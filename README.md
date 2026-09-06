# `randstat` — Modular Randomness Evaluation Workspace

> **A `no_std` Rust Cargo Workspace for streaming statistical randomness testing.**  
> Runs natively as a CLI binary (`randstat`) and compiles to WebAssembly for web dashboards.  
> Evolved from [Stochast/entropy](../entropy) — old project untouched, this is the clean rewrite.

---

## Overview

### Status Summary

| Suite | Tests | Standard | Status | Description |
|---|---|---|---|---|
| **randstat-suite-ent** | 5 tests | Fourmilab ENT | ✅ **Complete** | Fast baseline — entropy, chi-square, arithmetic mean, Monte Carlo π, serial correlation |
| **randstat-suite-nist** | 15 tests | NIST SP 800-22 Rev 1a | 🔧 **In Progress** | Cryptographic randomness battery (§2.1–§2.15) — monobit active, structured stubs |
| **randstat-suite-sp800-90b** | 10 tests | NIST SP 800-90B | 📋 **Planned** | Min-entropy estimation on physical noise sources (TRNG/QRNG) |
| **randstat-suite-ais31** | 9 tests | BSI AIS 20 / AIS 31 | 📋 **Planned** | German BSI physical TRNG evaluation battery (Tests T0–T8) |
| **randstat-suite-dieharder** | 12 tests | DIEHARD / Dieharder | 📋 **Planned** | Classical PRNG verification battery (Birthday spacings, Craps, Squeeze, etc.) |
| **randstat-suite-testu01** | 10+ tests | TestU01 | 📋 **Planned** | Academic benchmark batteries (SmallCrush / Crush / BigCrush) |
| **randstat-suite-practrand** | 6 tests | PractRand | 📋 **Planned** | Dynamic multi-terabyte stream testing filters (BCFN, Gap-16, FPFT, etc.) |
| **randstat-suite-gjrand** | 4 tests | gjrand | 📋 **Planned** | Lightweight generator benchmarking battery |
| **randstat-suite-full** | Master | Unified meta-suite | 🔧 **In Progress** | Comprehensive aggregator combining all test batteries |

> **All suites run in both native (CLI/embedded) and WebAssembly environments**

### Key Features

- **`no_std` compatible** — runs on embedded systems, compiles to WASM
- **Streaming API** — process unlimited data with constant memory
- **Implement once, reuse everywhere** — shared tests defined once in `randstat-tests`, reused across suites
- **Modular suites** — link only the batteries you need (precise WASM binary control)
- **Zero heap allocation** — all tests are `Copy + const`-constructible
- **Multi-format output** — terminal tables, Markdown reports, JSON (CI/CD)
- **Explicit TestStatus** — real evaluated tests vs `NOT IMPLEMENTED` stubs clearly distinguished

---

## Quick Start

### Basic Usage

```bash
# Evaluate with standard ENT suite (default)
randstat data.bin

# Evaluate with NIST SP 800-22 battery
randstat data.bin --suite nist

# Generate Markdown report
randstat data.bin --suite nist --md

# JSON output for scripts / CI
randstat data.bin --suite nist --json

# Read from stdin
cat /dev/urandom | head -c 1M | randstat --suite nist

# ASCII mode (one number per line)
randstat numbers.txt --ascii

# Custom significance level (99% confidence)
randstat data.bin --alpha 0.01
```

---

## Architecture

### Dependency Graph

```
randstat-core          #![no_std]   libm
      │
      │   algorithms/     — pure stateless evaluation functions
      │   bitstream/      — streaming byte accumulators (SHA-256, MC, SCC, freq)
      │   math/           — chi2 PDF/CDF, critical values, plot generators
      │   stats/          — EntResult, GuardrailEvaluation #[repr(C)] structs
      │   traits.rs       — StreamTest trait + TestResult + TestStatus
      │
      ↓
randstat-tests         #![no_std]   thin StreamTest wrappers (one file per test)
      │
      ├──────────────────┬──────────────────┬──────────────────┬──────────────────┐
      ↓                  ↓                  ↓                  ↓                  ↓
randstat-suite-ent  randstat-suite-nist  randstat-suite-sp800-90b  randstat-suite-ais31  (dieharder, testu01, etc.)
      │                  │                  │                  │                  │
      └──────────────────┴──────────────────┴──────────────────┴──────────────────┘
                                           ↓
                                  randstat-suite-full  (meta-suite aggregating all)
                                           │
                                  ┌────────┴────────┐
                                  ↓                 ↓
                            randstat-cli        randstat-wasm
                              (std)            (#![no_std], modular feature flags)
```

### Design Principles

| Principle | Implementation |
|---|---|
| **One test = one file** | Every `StreamTest` struct lives in its own `.rs` file in `randstat-tests` |
| **Implement once** | Common tests (Monobit, Runs, Rank, $\chi^2$) are reused across all suite crates |
| **Algorithms in core** | Pure evaluation functions in `randstat_core::algorithms` — no state, just math |
| **Suites as separate crates** | Each suite is independently linkable → precise WASM binary size control |
| **Zero heap allocation** | All suites are `Copy + const`-constructible; `static mut` in WASM is safe |
| **`no_std` boundary** | `core` + `libm` only; `std` exists only in `randstat-cli` |eparate crates** | Each suite is independently linkable → precise WASM binary size control |
| **Zero heap allocation** | All suites are `Copy + const`-constructible; `static mut` in WASM is safe |
| **`no_std` boundary** | `core` + `libm` only; `std` exists only in `randstat-cli` |

---

## Statistical Test Reference

Comprehensive catalog of all implemented and planned tests, organized by functional category.

### Frequency Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| Shannon Entropy | Fourmilab ENT | ENT, Full, AIS 31 | ✅ Real |
| Frequency (Monobit) | NIST SP800-22 §2.1 | NIST, Full, AIS 31 | ✅ Real |
| Chi-Square Uniformity | ENT / Pearson | ENT, NIST, Full, AIS 31, gjrand | ✅ Real |
| Arithmetic Mean | Fourmilab ENT | ENT, NIST, Full | ✅ Real |
| Block Frequency | NIST SP800-22 §2.2 | NIST, Full | 🔧 Stub |
| Cumulative Sums (CUSUM) | NIST SP800-22 §2.13 | NIST, Full | 🔧 Stub |
| Serial Test (m-bit patterns) | NIST SP800-22 §2.11 | NIST, Full | 🔧 Stub |
| Count Ones in Stream | Dieharder | Dieharder | 🔧 Stub |

### Runs Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| Runs Test | NIST SP800-22 §2.3 | NIST, Full | 🔧 Stub |
| Longest Run of Ones | NIST SP800-22 §2.4 | NIST, Full | 🔧 Stub |
| Runs Up/Down | Dieharder | Dieharder | 🔧 Stub |
| OPERM5 (Overlapping Permutations) | Dieharder | Dieharder | 🔧 Stub |

### Spectral Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| DFT / FFT Spectral | NIST SP800-22 §2.6 | NIST, Full | 🔧 Stub |

### Template / Occupancy Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| Non-overlapping Template | NIST SP800-22 §2.7 | NIST, Full | 🔧 Stub |
| Overlapping Template | NIST SP800-22 §2.8 | NIST, Full | 🔧 Stub |
| OQSO | Dieharder | Dieharder | 🔧 Stub |
| DNA | Dieharder | Dieharder | 🔧 Stub |

### Complexity Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| Maurer's Universal | NIST SP800-22 §2.9 | NIST, Full | 🔧 Stub |
| Berlekamp-Massey (Linear Complexity) | NIST SP800-22 §2.10 | NIST, Full | 🔧 Stub |
| Approximate Entropy | NIST SP800-22 §2.12 | NIST, Full | 🔧 Stub |
| Squeeze | Dieharder | Dieharder | 🔧 Stub |

### Spatial / Geometric Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| Monte Carlo π Estimation | Fourmilab ENT | ENT, Full | ✅ Real |
| Serial Correlation | Fourmilab ENT | ENT, Full | ✅ Real |
| Birthday Spacings | Dieharder | Dieharder | 🔧 Stub |
| Parking Lot | Dieharder | Dieharder | 🔧 Stub |
| Minimum Distance 2D | Dieharder | Dieharder | 🔧 Stub |
| 3D Spheres | Dieharder | Dieharder | 🔧 Stub |

### Matrix Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| Binary Matrix Rank | NIST SP800-22 §2.5 | NIST, Full | 🔧 Stub |

### Random Walk Excursion Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| Random Excursions | NIST SP800-22 §2.14 | NIST, Full | 🔧 Stub |
| Random Excursions Variant | NIST SP800-22 §2.15 | NIST, Full | 🔧 Stub |

### Distribution Tests

| Test | Standard | Suites | Status |
|------|----------|--------|--------|
| Overlapping Sums | Dieharder | Dieharder | 🔧 Stub |
| Craps | Dieharder | Dieharder | 🔧 Stub |

> **Note:** NIST Serial Test (§2.11) is an m-bit pattern frequency test, distinct from the ENT Serial Correlation test (Pearson correlation coefficient).

> **SHA-256 Note:** SHA-256 is used for file identity fingerprinting, not as a statistical test. It's implemented in `randstat-core::bitstream::sha256` and exposed as standalone WASM exports (`sha256_reset`/`update`/`finalize`) available in every build, including math-only.

---

## Development Guide

### Workspace Structure

```
randstat/
├── Cargo.toml                    Workspace root
├── justfile                      Build recipes
├── crates/
│   ├── randstat-core/            Core primitives (#![no_std])
│   │   └── src/
│   │       ├── algorithms/       Pure stateless evaluation functions
│   │       ├── bitstream/        Streaming accumulators (SHA-256, MC, SCC, freq)
│   │       ├── math/             Chi-square PDF/CDF, critical values, plotting
│   │       ├── stats/            Result structs (#[repr(C)] for WASM)
│   │       └── traits.rs         StreamTest trait + TestResult
│   ├── randstat-tests/           Individual test implementations (#![no_std])
│   │   └── src/
│   │       ├── frequency/        Bit/byte frequency tests
│   │       ├── runs/             Run-structure tests
│   │       ├── spectral/         Fourier/spectral tests
│   │       ├── template/         Pattern occupancy tests
│   │       ├── complexity/       Compressibility tests
│   │       ├── spatial/          Geometric tests
│   │       ├── matrix/           Matrix rank tests
│   │       ├── excursions/       Random walk tests
│   │       └── distribution/     Derived-distribution tests
│   ├── randstat-suite-ent/       ENT suite
│   ├── randstat-suite-nist/      NIST SP800-22 suite
│   ├── randstat-suite-quick/     Quick screening suite
│   ├── randstat-suite-full/      Full suite (ENT + NIST)
│   └── randstat-suite-dieharder/ Dieharder suite (planned)
├── apps/
│   ├── randstat-cli/             CLI binary
│   └── randstat-wasm/            WASM library (6 feature-gated builds)
└── ui/                           JavaScript SDK
```

### Adding a New Test

1. **Implement the algorithm** in `randstat-core/src/algorithms/your_test.rs`:
   ```rust
   pub fn your_test(count: u64, total: u64) -> TestResult { ... }
   ```
   Register in `algorithms/mod.rs`.

2. **Create the accumulator** in `randstat-tests/src/category/your_test.rs`:
   ```rust
   pub struct YourTest { /* raw counts */ }
   impl StreamTest for YourTest {
       fn evaluate(&self) -> TestResult {
           randstat_core::algorithms::your_test::your_test(self.count, self.total)
       }
   }
   ```
   Register in `category/mod.rs`.

3. **Add to suite**: Edit the suite crate (e.g., `randstat-suite-nist/src/lib.rs`) to add the field and update/reset calls.

4. **Verify**: `cargo check --workspace && cargo test --workspace`

### Build Recipes

```bash
# Development
just check              # Check workspace
just test               # Run tests
just check-wasm         # Check no_std crates for wasm32 target
just ci                 # Full CI gate (check + check-wasm + test)

# CLI
just build              # Build CLI
just run -- FILE        # Run CLI on file
just install            # Install to ~/.cargo/bin/randstat

# WebAssembly (5 size-optimized variants)
just build-wasm-all     # Build all variants
just build-wasm-math    # ~3–5 KB   — PDF/CDF calculators only
just build-wasm-ent     # ~15–18 KB — ENT suite
just build-wasm-quick   # ~12–15 KB — Quick suite
just build-wasm-nist    # ~30 KB    — NIST suite
just build-wasm-full    # ~35 KB    — Full suite (ENT + NIST)

# Manual WASM build
cargo build -p randstat-wasm \
    --no-default-features --features ent \
    --target wasm32-unknown-unknown --release
```

---

## API Reference

### CLI Usage

**Binary:** `randstat`  
**Modes:** Binary stream (default) | ASCII line mode (`--ascii`)  
**Output formats:** Terminal table | Markdown (`--md`) | JSON (`--json`)

#### Terminal Output Example

```text
==========================================================================================
                         RANDSTAT RANDOMNESS EVALUATION REPORT
==========================================================================================
File / Input Source : data.bin
Input Data Mode     : Binary (Raw Byte Stream)
Evaluated Stream    : 3,563,365 bytes (3.40 MB)
SHA-256 Hash        : 004a5176923d5fa689405c1b573a44313fc1ed7f1f1713c95c76583650d4bdf6
Significance Alpha  : α = 0.0500 (95.0% Confidence)

+------------------------+---------------------+-----------------------+--------------------+--------+
| Test Metric            | Calculated Value    | Ideal Range (α=5%)    | Deviation / Exceed | Status |
+------------------------+---------------------+-----------------------+--------------------+--------+
| Shannon Entropy        | 7.999943 b/B        | ~ 8.000000 bits/byte  | Compress: 0.00%    | PASS   |
| Chi-Square (df=255)    | 281.47              | [212.65 - 301.14]     | Exceed: 12.24%     | PASS   |
| Arithmetic Mean        | 127.5564            | ~ 127.500000          | Diff: +0.0564      | PASS   |
| Monte Carlo Pi         | 3.138647637         | ~ 3.141592654         | Error: 0.09%       | PASS   |
| Serial Correlation     | -0.000100           | [-0.010000, 0.010000] | Uncorrelated       | PASS   |
+------------------------+---------------------+-----------------------+--------------------+--------+

OVERALL VERDICT: [✓ LIKELY RANDOM]
```

#### Verdict Interpretation

| Verdict | Meaning |
|---|---|
| `✓ LIKELY RANDOM` | All metrics within statistical bounds |
| `⚠ NON-UNIFORM` | Chi-square exceedance too low — biased or patterned byte distribution |
| `⚠ TOO UNIFORM` | Chi-square exceedance too high — artificially forced uniformity |

### WebAssembly C-ABI

#### Core Exports (all builds)

| Export | Description |
|---|---|
| `chi2_pdf_export(x, df)` | Chi-square PDF at x |
| `normal_pdf_export(x, df)` | Normal approximation PDF |
| `chi2_critical_value_export(alpha, df)` | Wilson-Hilferty critical value |
| `chi2_pochisq(x, df)` | Upper-tail exceedance probability |
| `get_buffer_ptr()` | Pointer to 1000-element f64 plot buffer |
| `wasm_generate_chi2_points(df, xmin, xmax, steps)` | Fill buffer with Chi-square curve points |
| `wasm_generate_normal_points(df, xmin, xmax, steps)` | Fill buffer with Normal curve points |
| `sha256_reset()` | Reset SHA-256 accumulator |
| `sha256_update(ptr, len)` | Feed bytes into SHA-256 accumulator |
| `sha256_finalize(out_ptr)` | Write 32-byte digest to `out_ptr` |

#### ENT Suite Exports (`--features ent`)

| Export | Description |
|---|---|
| `ent_reset()` | Reset all ENT accumulators |
| `ent_update(ptr, len)` | Feed `len` bytes at `ptr` into the suite |
| `ent_finalize(out_result*)` | Write `EntResult` struct to `out_result` |
| `ent_validate(alpha, out_guardrail*)` | Write `GuardrailEvaluation` to `out_guardrail` |

> **Note:** `ent_finalize` sets `EntResult.sha256` to zero. Call `sha256_finalize` separately for the digest.

#### Additional Suite Exports

| Feature | Exports |
|---|---|
| `nist` | `nist_reset()`, `nist_update(ptr, len)` |
| `quick` | `quick_reset()`, `quick_update(ptr, len)` |
| `full` | `full_reset()`, `full_update(ptr, len)` |

#### JavaScript Usage Example

```javascript
const wasm = await WebAssembly.instantiateStreaming(fetch('randstat_wasm.wasm'));
const exports = wasm.instance.exports;
const { memory, sha256_reset, sha256_update, sha256_finalize,
        ent_reset, ent_update, ent_finalize } = exports;

// SHA-256 and the suite are independent — both receive the same bytes.
const INPUT_OFFSET  = 65536;
const RESULT_OFFSET = INPUT_OFFSET + data.length;
const DIGEST_OFFSET = RESULT_OFFSET + 128; // after EntResult struct

sha256_reset();
ent_reset();

new Uint8Array(memory.buffer, INPUT_OFFSET, data.length).set(data);
sha256_update(INPUT_OFFSET, data.length);
ent_update(INPUT_OFFSET, data.length);

// Two separate outputs:
sha256_finalize(DIGEST_OFFSET);  // 32 bytes — file identity
ent_finalize(RESULT_OFFSET);     // EntResult struct — statistical metrics

// Read the 32-byte digest
const digest = new Uint8Array(memory.buffer, DIGEST_OFFSET, 32);
const hex = Array.from(digest).map(b => b.toString(16).padStart(2,'0')).join('');
```

---

## Contributing

Contributions are welcome! Before submitting a pull request, please read and agree to the [Contributor License Agreement (CLA)](CLA.md).

In brief: Your contributions will be publicly available under EUPL-1.2, but you grant the project maintainer additional rights to use contributions in proprietary projects. See [CLA.md](CLA.md) for full details.

---

## License

Licensed under the [European Union Public Licence v1.2 (EUPL-1.2)](LICENSE).

**Note for Contributors:** By contributing to this project, you agree to the [Contributor License Agreement (CLA)](CLA.md), which grants the project maintainer broader rights including use in proprietary projects.
