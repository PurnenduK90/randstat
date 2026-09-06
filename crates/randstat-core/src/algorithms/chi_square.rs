//! Chi-square byte-uniformity evaluation algorithm.
//!
//! Tests whether the 256-bucket byte frequency histogram is consistent with
//! a uniform distribution, using the standard Pearson chi-square statistic.
//! This is a **pure function** Ã¢â‚¬â€ it holds no streaming state.

use crate::math::chi2::{compute_chi_square, pochisq};
use crate::traits::TestResult;

/// Evaluates byte-uniformity via the chi-square test.
///
/// # Arguments
/// * `byte_counts` Ã¢â‚¬â€ 256-bucket histogram of byte values.
/// * `total_bytes` Ã¢â‚¬â€ total bytes counted (sum of `byte_counts`).
///
/// # Returns
/// `TestResult` where:
/// - `statistic` = Pearson chi-square statistic (df=255)
/// - `p_value`   = `pochisq(statistic, 255)` Ã¢â‚¬â€ upper-tail exceedance probability
/// - `passed`    = `0.01 Ã¢â€°Â¤ p_value Ã¢â€°Â¤ 0.99` (two-tailed, 1% significance)
pub fn chi_square_test(byte_counts: &[u64; 256], total_bytes: u64) -> TestResult {
    if total_bytes == 0 {
        return TestResult::INSUFFICIENT_DATA;
    }
    let chi = compute_chi_square(byte_counts, total_bytes);
    let p = pochisq(chi, 255);
    // Two-tailed: fail if exceedance probability is extreme in either direction
    let passed = (0.01..=0.99).contains(&p);
    TestResult {
        statistic: chi,
        p_value: p,
        passed,
        status: if passed {
            crate::traits::TestStatus::Passed
        } else {
            crate::traits::TestStatus::Failed
        },
    }
}
