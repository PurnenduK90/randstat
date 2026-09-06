//! NIST SP800-22 §2.4 Longest Run of Ones in a Block algorithm.
//!
//! Tests whether the distribution of the longest runs of ones across blocks is consistent
//! with the theoretical distribution of a random sequence.
//!
//! ## Reference
//! NIST SP800-22 Rev. 1a, §2.4 (p. 2-9)

use crate::math::chi2::pochisq;
use crate::traits::{TestResult, TestStatus};

/// Precomputed theoretical probabilities for M=128, K=5 (6 bins: <=4, 5, 6, 7, 8, >=9)
pub const PI_M128: [f64; 6] = [0.1174, 0.2430, 0.2493, 0.1752, 0.1027, 0.1124];

/// Precomputed theoretical probabilities for M=8, K=3 (4 bins: <=1, 2, 3, >=4)
pub const PI_M8: [f64; 4] = [0.2148, 0.3672, 0.2305, 0.1875];

/// Evaluates the NIST §2.4 Longest Run of Ones test for M=128 blocks.
///
/// # Arguments
/// * `bin_counts` - Counts of blocks whose longest run fell into each of the 6 bins: `[<=4, 5, 6, 7, 8, >=9]`.
/// * `num_blocks` - Total blocks evaluated (sum of `bin_counts`).
pub fn nist_longest_run_m128(bin_counts: &[u64; 6], num_blocks: u64) -> TestResult {
    if num_blocks < 16 {
        // Fallback or insufficient data
        return TestResult::INSUFFICIENT_DATA;
    }
    let n = num_blocks as f64;
    let mut chi_square = 0.0_f64;

    for i in 0..6 {
        let expected = n * PI_M128[i];
        let diff = (bin_counts[i] as f64) - expected;
        chi_square += (diff * diff) / expected;
    }

    let p_value = pochisq(chi_square, 5);
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

/// Evaluates the NIST §2.4 Longest Run of Ones test for M=8 blocks (small stream mode).
pub fn nist_longest_run_m8(bin_counts: &[u64; 4], num_blocks: u64) -> TestResult {
    if num_blocks < 16 {
        return TestResult::INSUFFICIENT_DATA;
    }
    let n = num_blocks as f64;
    let mut chi_square = 0.0_f64;

    for i in 0..4 {
        let expected = n * PI_M8[i];
        let diff = (bin_counts[i] as f64) - expected;
        chi_square += (diff * diff) / expected;
    }

    let p_value = pochisq(chi_square, 3);
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
    fn test_longest_run_algorithms() {
        assert_eq!(
            nist_longest_run_m128(&[0; 6], 10).status,
            TestStatus::InsufficientData
        );
        assert_eq!(
            nist_longest_run_m8(&[0; 4], 10).status,
            TestStatus::InsufficientData
        );

        // Perfectly matching expected distribution for 100 blocks
        let mut counts_m128 = [0u64; 6];
        for i in 0..6 {
            counts_m128[i] = (100.0 * PI_M128[i]).round() as u64;
        }
        let res = nist_longest_run_m128(&counts_m128, 100);
        assert!(res.passed);
        assert!(res.p_value > 0.5);
    }
}
