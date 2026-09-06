//! Poker Test algorithm (BSI AIS 31 Test T2 / DIEHARD / Knuth).
//!
//! Evaluates the uniformity of consecutive 4-bit nibbles (or m-bit words) using
//! Pearson chi-square on 16 buckets with 15 degrees of freedom.

use crate::math::chi2::pochisq;
use crate::traits::{TestResult, TestStatus};

/// Computes the Poker Test result on 4-bit nibbles.
///
/// # Arguments
/// * `nibble_counts` - 16-element histogram of 4-bit nibble frequencies.
/// * `total_nibbles` - Total nibbles evaluated (k).
pub fn poker_test(nibble_counts: &[u64; 16], total_nibbles: u64) -> TestResult {
    if total_nibbles < 100 {
        return TestResult::INSUFFICIENT_DATA;
    }
    let k = total_nibbles as f64;
    let mut sum_sq = 0.0_f64;

    for &count in nibble_counts {
        let c = count as f64;
        sum_sq += c * c;
    }

    // Formula: (16 / k) * sum(f_i^2) - k
    let chi_square = (16.0 / k) * sum_sq - k;
    let p_value = pochisq(chi_square, 15);
    // Two-tailed significance check at alpha = 0.01
    let passed = (0.01..=0.99).contains(&p_value);

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
    fn test_poker_algorithm() {
        assert_eq!(
            poker_test(&[0; 16], 50).status,
            TestStatus::InsufficientData
        );

        // Perfectly uniform 1600 nibbles -> 100 each
        let uniform_counts = [100u64; 16];
        let res_uniform = poker_test(&uniform_counts, 1600);
        // Extreme uniform might trigger low p-value two-tailed, check chi2 is ~0
        assert_eq!(res_uniform.statistic.round(), 0.0);
    }
}
