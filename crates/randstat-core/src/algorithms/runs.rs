//! NIST SP800-22 §2.3 Runs Test algorithm.
//!
//! Tests whether the total number of runs (consecutive identical bits) is consistent
//! with a random sequence.
//!
//! ## Reference
//! NIST SP800-22 Rev. 1a, §2.3 (p. 2-6)

use crate::math::chi2::erfc;
use crate::traits::{TestResult, TestStatus};
use libm::{fabs, sqrt};

/// Computes the NIST SP800-22 §2.3 Runs test result.
///
/// # Arguments
/// * `ones` - total number of 1-bits observed.
/// * `total_bits` - total bits observed (N).
/// * `transitions` - total bit transitions (where bit[i] != bit[i+1]). Total runs V_n = transitions + 1.
pub fn nist_runs(ones: u64, total_bits: u64, transitions: u64) -> TestResult {
    if total_bits < 100 {
        return TestResult::INSUFFICIENT_DATA;
    }
    let n = total_bits as f64;
    let pi = ones as f64 / n;

    // Step 1: Pre-test frequency check. If pi differs from 0.5 by >= 2 / sqrt(n), test fails.
    let tau = 2.0 / sqrt(n);
    if fabs(pi - 0.5) >= tau {
        return TestResult {
            statistic: 0.0,
            p_value: 0.0,
            passed: false,
            status: TestStatus::Failed,
        };
    }

    // Step 2: Compute test statistic V_n(obs) = 1 + transitions
    let v_n = (transitions + 1) as f64;

    // Step 3: Compute p-value = erfc(|V_n - 2n*pi*(1-pi)| / (2 * sqrt(2n) * pi * (1-pi)))
    let num = fabs(v_n - 2.0 * n * pi * (1.0 - pi));
    let denom = 2.0 * sqrt(2.0 * n) * pi * (1.0 - pi);

    if denom <= 0.0 {
        return TestResult::INSUFFICIENT_DATA;
    }

    let p_value = erfc(num / denom);
    let passed = p_value >= 0.01;

    TestResult {
        statistic: v_n,
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
    fn test_nist_runs() {
        assert_eq!(nist_runs(0, 50, 0).status, TestStatus::InsufficientData);
        // Perfect alternating bits: 01010101... -> pi = 0.5, transitions = 999 (too many runs)
        let res_alt = nist_runs(500, 1000, 999);
        assert!(!res_alt.passed);
        assert!(res_alt.p_value < 0.01);

        // Typical random bitstream: ~500 ones, ~500 transitions in 1000 bits
        let res_rand = nist_runs(500, 1000, 500);
        assert!(res_rand.passed);
        assert!(res_rand.p_value > 0.5);

        // Failed pre-test (biased bitstream)
        let res_biased = nist_runs(800, 1000, 300);
        assert!(!res_biased.passed);
        assert_eq!(res_biased.p_value, 0.0);
    }
}
