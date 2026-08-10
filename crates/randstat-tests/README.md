# `randstat-tests`

`#![no_std]` individual statistical test accumulators for the `randstat` workspace.

**One test = one file.** Each struct implements `randstat_core::traits::StreamTest` and is a thin wrapper — `evaluate()` delegates to a pure function in `randstat_core::algorithms`.

| Category | Tests |
|---|---|
| `frequency/` | Monobit ✅, Shannon Entropy ✅, Chi-Square ✅, Arithmetic Mean, Block Frequency (stub), CUSUM (stub) |
| `runs/` | Runs Test (stub), Longest Run (stub) |
| `spectral/` | DFT/FFT (stub) |
| `complexity/` | Berlekamp-Massey (stub), Approx Entropy (stub) |
| `spatial/` | Monte Carlo π ✅, Serial Correlation ✅ |
| `matrix/` | Binary Matrix Rank (stub) |

Suite aggregators live in separate crates (`randstat-suite-*`), not here.
