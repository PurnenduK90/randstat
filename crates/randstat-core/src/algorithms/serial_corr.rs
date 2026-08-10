//! Serial correlation evaluation algorithm.
//!
//! Converts a precomputed Pearson SCC value into a `TestResult`.
//! This is a **pure function** — it holds no streaming state.

use crate::traits::TestResult;
use libm::fabs;

/// Evaluates a Pearson serial correlation coefficient into a `TestResult`.
///
/// # Arguments
/// * `scc` — Pearson SCC; use sentinel value `< −90_000.0` for a constant stream.
///
/// # Returns
/// `TestResult` where:
/// - `statistic` = the SCC value (or sentinel)
/// - `p_value`   = `(1 − |scc| / 0.05).clamp(0, 1)` (pseudo p-value)
/// - `passed`    = `|scc| < 0.05` (not constant stream, and low correlation)
pub fn serial_corr_result(scc: f64) -> TestResult {
    if scc < -90_000.0 {
        // Constant byte stream — denominator was zero in the accumulator
        return TestResult {
            statistic: scc,
            p_value: 0.0,
            passed: false,
        };
    }
    let abs_scc = fabs(scc);
    let p_value = (1.0 - abs_scc / 0.05).clamp(0.0, 1.0);
    TestResult {
        statistic: scc,
        p_value,
        passed: abs_scc < 0.05,
    }
}
