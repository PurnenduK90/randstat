//! Serial correlation evaluation algorithm.
//!
//! Converts a precomputed Pearson SCC value into a `TestResult`.
//! This is a **pure function** Ã¢â‚¬â€ it holds no streaming state.

use crate::traits::TestResult;
use libm::fabs;

/// Evaluates a Pearson serial correlation coefficient into a `TestResult`.
///
/// # Arguments
/// * `scc` Ã¢â‚¬â€ Pearson SCC; use sentinel value `< Ã¢Ë†â€™90_000.0` for a constant stream.
///
/// # Returns
/// `TestResult` where:
/// - `statistic` = the SCC value (or sentinel)
/// - `p_value`   = `(1 Ã¢Ë†â€™ |scc| / 0.05).clamp(0, 1)` (pseudo p-value)
/// - `passed`    = `|scc| < 0.05` (not constant stream, and low correlation)
pub fn serial_corr_result(scc: f64) -> TestResult {
    if scc < -90_000.0 {
        // Constant byte stream Ã¢â‚¬â€ denominator was zero in the accumulator
        return TestResult {
            statistic: scc,
            p_value: 0.0,
            passed: false,
            status: crate::traits::TestStatus::Failed,
        };
    }
    let abs_scc = fabs(scc);
    let p_value = (1.0 - abs_scc / 0.05).clamp(0.0, 1.0);
    let passed = abs_scc < 0.05;
    TestResult {
        statistic: scc,
        p_value,
        passed,
        status: if passed {
            crate::traits::TestStatus::Passed
        } else {
            crate::traits::TestStatus::Failed
        },
    }
}
