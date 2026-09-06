//! NIST SP800-22 §2.2 Block Frequency Test algorithm.
//!
//! Tests whether the frequency of ones in M-bit blocks is approximately M/2.
//!
//! ## Reference
//! NIST SP800-22 Rev. 1a, §2.2 (p. 2-4)

use crate::math::chi2::pochisq;
use crate::traits::{TestResult, TestStatus};

/// Computes the NIST SP800-22 §2.2 Block Frequency test result.
///
/// # Arguments
/// * `num_blocks` - Total number of M-bit blocks processed (N = floor(n / M)).
/// * `sum_sq_diff` - Sum of squared deviations: sum_{i=1}^N (ones_i - M/2)^2.
/// * `block_size` - Size of each block in bits (M, typically 128).
pub fn nist_block_frequency(num_blocks: u64, sum_sq_diff: u64, block_size: usize) -> TestResult {
    if num_blocks < 1 || block_size == 0 {
        return TestResult::INSUFFICIENT_DATA;
    }

    let m = block_size as f64;

    // chi^2(obs) = 4 * M * sum_{i=1}^N (pi_i - 0.5)^2 = (4 / M) * sum_{i=1}^N (ones_i - M/2)^2
    let chi_square = (4.0 / m) * (sum_sq_diff as f64);

    // p-value = pochisq(chi^2, N) where degrees of freedom df = N
    let p_value = pochisq(chi_square, num_blocks as usize);
    let passed = p_value >= 0.01;

    TestResult {
        statistic: chi_square,
        p_value,
        passed,
        status: if passed {
            TestStatus::Passed
        } else {
            TestStatus::Failed
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nist_block_frequency() {
        assert_eq!(
            nist_block_frequency(0, 0, 128).status,
            TestStatus::InsufficientData
        );
        assert_eq!(
            nist_block_frequency(10, 0, 0).status,
            TestStatus::InsufficientData
        );

        // Perfectly balanced blocks: each 128-bit block has exactly 64 ones -> sum_sq_diff = 0
        // chi^2 = 0.0, df = 10 -> p_value = 1.0 (pass)
        let res_bal = nist_block_frequency(10, 0, 128);
        assert!(res_bal.passed);
        assert_eq!(res_bal.statistic, 0.0);
        assert!(res_bal.p_value >= 0.99);

        // Highly biased blocks: each 128-bit block has 128 ones -> diff = 64, diff^2 = 4096
        // For 10 blocks: sum_sq_diff = 40960 -> chi^2 = (4/128) * 40960 = 1280.0
        // p-value -> ~0.0 (fail)
        let res_biased = nist_block_frequency(10, 40960, 128);
        assert!(!res_biased.passed);
        assert!(res_biased.p_value < 0.0001);
    }
}
