//! Shannon entropy evaluation algorithm.
//!
//! Converts raw byte-frequency counts into an entropy score and `TestResult`.
//! This is a **pure function** — it holds no streaming state.

use crate::traits::TestResult;
use libm::log2;

/// Computes the Shannon entropy in bits per byte from raw frequency counts.
///
/// # Arguments
/// * `byte_counts` — 256-bucket histogram of byte values.
/// * `total_bytes` — total bytes counted (sum of `byte_counts`).
///
/// # Returns
/// `TestResult` where:
/// - `statistic` = entropy in bits/byte (0.0–8.0)
/// - `p_value`   = `entropy / 8.0` (normalised; 1.0 = perfect random)
/// - `passed`    = `entropy >= 7.9`
pub fn shannon_score(byte_counts: &[u64; 256], total_bytes: u64) -> TestResult {
    if total_bytes == 0 {
        return TestResult {
            statistic: 0.0,
            p_value: 0.0,
            passed: false,
        };
    }
    let n = total_bytes as f64;
    let mut entropy = 0.0_f64;
    for &count in byte_counts.iter() {
        if count > 0 {
            let p = count as f64 / n;
            entropy -= p * log2(p);
        }
    }
    let p_value = (entropy / 8.0).clamp(0.0, 1.0);
    TestResult {
        statistic: entropy,
        p_value,
        passed: entropy >= 7.9,
    }
}
