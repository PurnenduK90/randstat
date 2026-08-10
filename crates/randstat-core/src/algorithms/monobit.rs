//! NIST SP800-22 §2.1 Frequency (Monobit) algorithm.
//!
//! Converts raw bit counts into a p-value using the NIST SP800-22 §2.1 formula.
//! This is a **pure function** — it holds no streaming state.
//!
//! ## Reference
//! NIST SP800-22 Rev. 1a, §2.1.4 (p. 2-2)

use crate::math::chi2::erfc;
use crate::traits::TestResult;
use libm::sqrt;

/// Computes the NIST SP800-22 §2.1 Frequency (Monobit) test result.
///
/// # Arguments
/// * `ones` — total number of 1-bits observed in the stream.
/// * `total_bits` — total bits observed (= bytes × 8).
///
/// # Algorithm
/// 1. `S_n = ones − zeros`
/// 2. `S_obs = |S_n| / sqrt(N)`
/// 3. `p_value = erfc(S_obs / sqrt(2))`
/// 4. Pass if `p_value ≥ 0.01` (NIST minimum).
pub fn nist_monobit(ones: u64, total_bits: u64) -> TestResult {
    if total_bits == 0 {
        return TestResult {
            statistic: 0.0,
            p_value: 1.0,
            passed: true,
        };
    }
    let n = total_bits as f64;
    let zeros = total_bits - ones;
    let s_n = (ones as f64) - (zeros as f64);
    let s_obs = s_n.abs() / sqrt(n);
    let p_value = erfc(s_obs / core::f64::consts::SQRT_2);
    TestResult {
        statistic: s_obs,
        p_value,
        passed: p_value >= 0.01,
    }
}
