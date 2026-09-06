//! `randstat-core` â€” `#![no_std]` pure math, bitstream accumulators, and statistics primitives.
//!
//! This crate is the foundation of the `randstat` workspace. It has zero heap allocations
//! and is fully compatible with `wasm32-unknown-unknown` without a wasm-bindgen runtime.
//!
//! # Module Layout
//! - [`traits`] â€” `StreamTest` trait and `#[repr(C)]` result structs
//! - [`bitstream`] â€” Streaming byte-level accumulators (frequency, SHA-256, Monte Carlo, serial correlation)
//! - [`math`] â€” Chi-square, Normal, lgamma, pochisq, and distribution plot generators
//! - [`stats`] â€” `EntResult`, `GuardrailEvaluation`, `Status`, and `evaluate_guardrails`

#![no_std]

pub mod algorithms;
pub mod bitstream;
pub mod math;
pub mod stats;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::algorithms::chi_square::chi_square_test;
    use super::algorithms::monobit::nist_monobit;
    use super::algorithms::monte_carlo::monte_carlo_pi_result;
    use super::algorithms::serial_corr::serial_corr_result;
    use super::algorithms::shannon::shannon_score;
    use super::bitstream::byte_freq::ByteFreqTracker;
    use super::bitstream::mont_carlo::MonteCarloAccum;
    use super::bitstream::serial_corr::SerialCorrAccum;
    use super::bitstream::sha256::{format_hex, Sha256};
    use super::math::chi2::{
        chi2_critical_value, chi2_pdf, compute_chi_square, erfc, lgamma, normal_pdf, pochisq, poz,
    };
    use super::math::plot::{generate_chi2_points, generate_normal_points};
    use super::stats::ent_result::EntResult;
    use super::stats::validate::{evaluate_guardrails, Status};

    #[test]
    fn test_algorithms() {
        use super::traits::TestStatus;

        // monobit
        let res = nist_monobit(0, 0);
        assert_eq!(res.status, TestStatus::InsufficientData);
        let res2 = nist_monobit(500, 1000);
        assert!(res2.passed);
        assert_eq!(res2.status, TestStatus::Passed);
        let res3 = nist_monobit(0, 1000);
        assert!(!res3.passed);
        assert_eq!(res3.status, TestStatus::Failed);

        // chi_square
        let counts = [0u64; 256];
        let res = chi_square_test(&counts, 0);
        assert_eq!(res.status, TestStatus::InsufficientData);
        let counts2 = [4u64; 256]; // 1024 total, perfectly uniform -> fails two-tailed
        let res2 = chi_square_test(&counts2, 1024);
        assert!(!res2.passed);
        assert_eq!(res2.status, TestStatus::Failed);
        let mut counts3 = [2u64; 256];
        counts3[..128].fill(6);
        let res3 = chi_square_test(&counts3, 1024);
        assert!(res3.passed);
        assert_eq!(res3.status, TestStatus::Passed);

        // shannon
        let res = shannon_score(&counts, 0);
        assert_eq!(res.status, TestStatus::InsufficientData);
        let res2 = shannon_score(&counts2, 1024);
        assert!(res2.passed);
        assert_eq!(res2.status, TestStatus::Passed);

        // monte_carlo
        let res = monte_carlo_pi_result(0, 0);
        assert_eq!(res.status, TestStatus::InsufficientData);
        let res2 = monte_carlo_pi_result(785, 1000);
        assert!(res2.passed);
        assert_eq!(res2.status, TestStatus::Passed);

        // serial_corr
        let res = serial_corr_result(-99_999.0);
        assert!(!res.passed);
        assert_eq!(res.status, TestStatus::Failed);
        let res2 = serial_corr_result(0.01);
        assert!(res2.passed);
        assert_eq!(res2.status, TestStatus::Passed);
    }

    #[test]
    fn test_math_chi2() {
        // lgamma
        assert_eq!(lgamma(0.0), 0.0);
        assert_eq!(lgamma(-1.0), 0.0);
        assert!(lgamma(5.0) > 0.0); // a < 12
        assert!(lgamma(15.0) > 0.0); // a >= 12

        // chi2_pdf
        assert_eq!(chi2_pdf(-1.0, 1.0), 0.0);
        assert_eq!(chi2_pdf(1.0, -1.0), 0.0);
        assert!(chi2_pdf(2.0, 2.0) > 0.0);
        assert_eq!(chi2_pdf(100000.0, 1.0), 0.0); // log_pdf < -700.0

        // normal_pdf
        assert_eq!(normal_pdf(0.0, -1.0), 0.0);
        assert!(normal_pdf(0.0, 2.0) > 0.0);
        assert_eq!(normal_pdf(10000.0, 1.0), 0.0); // exponent < -700.0

        // poz
        assert_eq!(poz(0.0), 0.5);
        assert!(poz(10.0) > 0.99); // y >= Z_MAX * 0.5 (6.0 * 0.5 = 3.0)
        assert!(poz(-10.0) < 0.01);
        assert!(poz(0.5) > 0.5); // y < 1.0
        assert!(poz(1.5) > 0.5); // y >= 1.0
        assert!(poz(-0.5) < 0.5); // y < 1.0, z < 0.0
        assert!(poz(-1.5) < 0.5); // y >= 1.0, z < 0.0

        // pochisq
        assert_eq!(pochisq(-1.0, 5), 1.0);
        assert_eq!(pochisq(1.0, 0), 1.0);
        assert!(pochisq(1.0, 1) > 0.0); // df = 1 (odd)
        assert!(pochisq(1.0, 2) > 0.0); // df = 2 (even)

        // df > 2, even
        assert!(pochisq(1.0, 4) > 0.0); // small a <= BIGX
        assert_eq!(pochisq(50.0, 4), 0.0); // large a > BIGX (a = 25.0 > 20.0)

        // df > 2, odd
        assert!(pochisq(1.0, 5) > 0.0); // small a <= BIGX
        assert_eq!(pochisq(50.0, 5), 0.0); // large a > BIGX

        // chi2_critical_value
        assert!(chi2_critical_value(0.05, 255.0) > 0.0); // alpha < 0.5
        assert!(chi2_critical_value(0.95, 255.0) > 0.0); // alpha >= 0.5
                                                         // To cover inner <= 0.0
        assert_eq!(chi2_critical_value(0.9999999999, 0.01), 0.0);

        // compute_chi_square
        let counts = [4u64; 256];
        assert!(compute_chi_square(&counts, 1024) >= 0.0);
        assert_eq!(compute_chi_square(&counts, 0), 0.0);

        // erfc
        assert!(erfc(0.0) > 0.0);
    }

    #[test]
    fn test_math_plot() {
        let mut buf = [0.0f64; 10];
        assert_eq!(generate_chi2_points(2.0, 0.0, 1.0, 0, &mut buf), 0);
        assert_eq!(generate_chi2_points(2.0, 0.0, 1.0, 5, &mut buf), 10);
        assert_eq!(generate_chi2_points(2.0, 0.0, 1.0, 20, &mut buf), 10); // cap to 5

        assert_eq!(generate_normal_points(2.0, 0.0, 1.0, 0, &mut buf), 0);
        assert_eq!(generate_normal_points(2.0, 0.0, 1.0, 5, &mut buf), 10);
        assert_eq!(generate_normal_points(2.0, 0.0, 1.0, 20, &mut buf), 10); // cap to 5
    }

    #[test]
    fn test_bitstreams() {
        // ByteFreqTracker
        let mut tracker = ByteFreqTracker::new();
        assert_eq!(tracker.entropy(), 0.0);
        assert_eq!(tracker.arithmetic_mean(), 0.0);
        tracker.update(&[0, 1, 2, 3]);
        assert!(tracker.entropy() > 0.0);
        assert_eq!(tracker.arithmetic_mean(), 1.5);
        tracker.reset();
        assert_eq!(tracker.total_bytes, 0);

        // Default implementation
        let default_tracker = ByteFreqTracker::default();
        assert_eq!(default_tracker.total_bytes, 0);

        // MonteCarloAccum
        let mut mc = MonteCarloAccum::new();
        assert_eq!(mc.pi(), 0.0);
        mc.update(&[0, 0, 0, 0, 0, 0]); // inside
        assert_eq!(mc.pi(), 4.0);
        mc.update(&[255, 255, 255, 255, 255, 255]); // outside
        assert_eq!(mc.pi(), 2.0);
        mc.reset();
        assert_eq!(mc.total, 0);
        let default_mc = MonteCarloAccum::default();
        assert_eq!(default_mc.total, 0);

        // SerialCorrAccum
        let mut sc = SerialCorrAccum::new();
        assert_eq!(sc.correlation(), 0.0);
        sc.update(&[12]);
        assert_eq!(sc.correlation(), -100_000.0); // constant stream
        sc.reset();
        sc.update(&[1, 2, 3, 4]);
        assert!(sc.correlation() != 0.0);
        let default_sc = SerialCorrAccum::default();
        assert_eq!(default_sc.totalc, 0);

        // Sha256
        let mut hasher = Sha256::new();
        hasher.update(&[0; 10]);
        let digest1 = hasher.finalize();
        hasher.reset();
        hasher.update(&[0; 10]);
        let digest2 = hasher.finalize();
        assert_eq!(digest1, digest2);

        // Update with more than 64 bytes
        hasher.reset();
        hasher.update(&[0; 130]);
        let _ = hasher.finalize();

        // Update with non-empty buffer that doesn't fill
        hasher.reset();
        hasher.update(&[0; 10]);
        hasher.update(&[0; 10]);
        let _ = hasher.finalize();

        // Update with non-empty buffer that fills
        hasher.reset();
        hasher.update(&[0; 60]);
        hasher.update(&[0; 10]);
        let _ = hasher.finalize();

        // Finalize with buf_len > 56
        hasher.reset();
        hasher.update(&[0; 58]);
        let _ = hasher.finalize();

        // format_hex
        let mut out = [0u8; 64];
        format_hex(&digest1, &mut out);
        assert_ne!(out, [0u8; 64]);
        let default_sha = Sha256::default();
        assert_eq!(default_sha.total_bytes, 0);
    }

    #[test]
    fn test_stats() {
        assert_eq!(Status::Pass.as_str(), "PASS");
        assert_eq!(Status::Warn.as_str(), "WARN");
        assert_eq!(Status::Fail.as_str(), "FAIL");

        // Pass case
        let res_pass = EntResult {
            total_bytes: 1000,
            entropy_bits_per_byte: 7.95,
            compression_percent: 0.5,
            chi_square: 250.0,
            mean: 127.4,
            monte_carlo_pi: core::f64::consts::PI,
            serial_correlation: 0.005,
            sha256: [0; 32],
        };
        let eval_pass = evaluate_guardrails(&res_pass, 0.05);
        assert_eq!(eval_pass.overall_status, Status::Pass);

        // Warn case (entropy_bits_per_byte < 7.9)
        let res_warn = EntResult {
            entropy_bits_per_byte: 7.85,
            ..res_pass
        };
        let eval_warn = evaluate_guardrails(&res_warn, 0.05);
        assert_eq!(eval_warn.overall_status, Status::Warn);
        assert_eq!(eval_warn.entropy_status, Status::Warn);

        // Fail case (mean_diff >= 5.0)
        let res_fail_mean = EntResult {
            mean: 120.0,
            ..res_pass
        };
        let eval_fail_mean = evaluate_guardrails(&res_fail_mean, 0.05);
        assert_eq!(eval_fail_mean.overall_status, Status::Fail);
        assert_eq!(eval_fail_mean.mean_status, Status::Fail);

        // Fail case (pi_err >= 3.0)
        let res_fail_pi = EntResult {
            monte_carlo_pi: 4.0,
            ..res_pass
        };
        let eval_fail_pi = evaluate_guardrails(&res_fail_pi, 0.05);
        assert_eq!(eval_fail_pi.overall_status, Status::Fail);
        assert_eq!(eval_fail_pi.pi_status, Status::Fail);

        // Fail case (chi_square extreme)
        let res_fail_chi = EntResult {
            chi_square: 1000.0, // extremely high chi-square, tiny pochisq p_value
            ..res_pass
        };
        let eval_fail_chi = evaluate_guardrails(&res_fail_chi, 0.05);
        assert_eq!(eval_fail_chi.overall_status, Status::Fail);
        assert_eq!(eval_fail_chi.chi_square_status, Status::Fail);

        // Warn case (mean_diff >= 1.0 but < 5.0)
        let res_warn_mean = EntResult {
            mean: 126.0,
            ..res_pass
        };
        let eval_warn_mean = evaluate_guardrails(&res_warn_mean, 0.05);
        assert_eq!(eval_warn_mean.mean_status, Status::Warn);

        // Warn case (pi_err >= 1.0 but < 3.0)
        let res_warn_pi = EntResult {
            monte_carlo_pi: 3.2,
            ..res_pass
        };
        let eval_warn_pi = evaluate_guardrails(&res_warn_pi, 0.05);
        assert_eq!(eval_warn_pi.pi_status, Status::Warn);

        // Warn case (serial_correlation >= 0.01 but < 0.05)
        let res_warn_scc = EntResult {
            serial_correlation: 0.02,
            ..res_pass
        };
        let eval_warn_scc = evaluate_guardrails(&res_warn_scc, 0.05);
        assert_eq!(eval_warn_scc.serial_correlation_status, Status::Warn);
        assert_eq!(eval_warn_scc.overall_status, Status::Warn);

        // Fail case (serial correlation extreme / constant stream)
        let res_fail_scc = EntResult {
            serial_correlation: -100_000.0,
            ..res_pass
        };
        let eval_fail_scc = evaluate_guardrails(&res_fail_scc, 0.05);
        assert_eq!(eval_fail_scc.serial_correlation_status, Status::Fail);
        assert_eq!(eval_fail_scc.overall_status, Status::Fail);

        // Test EntResult methods
        assert_eq!(res_pass.compression_reduction_int(), 0);
        let hex_bytes = res_pass.sha256_hex_bytes();
        assert_eq!(hex_bytes[0], b'0');
    }
}
