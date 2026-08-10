# `randstat-core`

`#![no_std]` shared primitives for the `randstat` workspace.

Provides:
- **`algorithms/`** — pure stateless evaluation functions (`nist_monobit`, `shannon_score`, `monte_carlo_pi_result`, `serial_corr_result`, `chi_square_test`)
- **`bitstream/`** — streaming byte accumulators (`ByteFreqTracker`, `MonteCarloAccum`, `SerialCorrAccum`, `Sha256`)
- **`math/`** — chi-square PDF/CDF, critical values, plot coordinate generators
- **`stats/`** — `EntResult` and `GuardrailEvaluation` (`#[repr(C)]`, WASM-safe)
- **`traits`** — `StreamTest` trait + `TestResult`

Zero heap allocation. Compiles to `wasm32-unknown-unknown` with only `libm` as a dependency.
