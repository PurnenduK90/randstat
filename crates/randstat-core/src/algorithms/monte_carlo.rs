//! Monte Carlo π estimation algorithm.
//!
//! Converts raw point-in-circle counts into a π estimate and `TestResult`.
//! This is a **pure function** — it holds no streaming state.

use crate::traits::TestResult;
use libm::fabs;

/// Evaluates a Monte Carlo π estimate from raw circle-test counts.
///
/// # Arguments
/// * `inside` — points that fell inside the unit circle.
/// * `total`  — total points tested.
///
/// # Returns
/// `TestResult` where:
/// - `statistic` = estimated π value
/// - `p_value`   = `(1 − error_percent / 3.0).clamp(0, 1)` (pseudo p-value)
/// - `passed`    = error < 3.0%
pub fn monte_carlo_pi_result(inside: u64, total: u64) -> TestResult {
    if total == 0 {
        return TestResult {
            statistic: 0.0,
            p_value: 0.0,
            passed: false,
        };
    }
    let pi_est = 4.0 * (inside as f64) / (total as f64);
    let pi_err = fabs(pi_est - core::f64::consts::PI) / core::f64::consts::PI * 100.0;
    let p_value = (1.0 - pi_err / 3.0).clamp(0.0, 1.0);
    TestResult {
        statistic: pi_est,
        p_value,
        passed: pi_err < 3.0,
    }
}
