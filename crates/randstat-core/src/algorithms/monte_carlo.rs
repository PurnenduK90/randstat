//! Monte Carlo Ãâ‚¬ estimation algorithm.
//!
//! Converts raw point-in-circle counts into a Ãâ‚¬ estimate and `TestResult`.
//! This is a **pure function** Ã¢â‚¬â€ it holds no streaming state.

use crate::traits::TestResult;
use libm::fabs;

/// Evaluates a Monte Carlo Ãâ‚¬ estimate from raw circle-test counts.
///
/// # Arguments
/// * `inside` Ã¢â‚¬â€ points that fell inside the unit circle.
/// * `total`  Ã¢â‚¬â€ total points tested.
///
/// # Returns
/// `TestResult` where:
/// - `statistic` = estimated Ãâ‚¬ value
/// - `p_value`   = `(1 Ã¢Ë†â€™ error_percent / 3.0).clamp(0, 1)` (pseudo p-value)
/// - `passed`    = error < 3.0%
pub fn monte_carlo_pi_result(inside: u64, total: u64) -> TestResult {
    if total == 0 {
        return TestResult::INSUFFICIENT_DATA;
    }
    let pi_est = 4.0 * (inside as f64) / (total as f64);
    let pi_err = fabs(pi_est - core::f64::consts::PI) / core::f64::consts::PI * 100.0;
    let p_value = (1.0 - pi_err / 3.0).clamp(0.0, 1.0);
    let passed = pi_err < 3.0;
    TestResult {
        statistic: pi_est,
        p_value,
        passed,
        status: if passed {
            crate::traits::TestStatus::Passed
        } else {
            crate::traits::TestStatus::Failed
        },
    }
}
