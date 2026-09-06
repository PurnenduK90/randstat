//! Statistical guardrail validation engine.
//!
//! Evaluates an [`EntResult`] against strict statistical thresholds and returns
//! a [`GuardrailEvaluation`] with per-test [`Status`] values and derived metrics.

use crate::math::chi2::{chi2_critical_value, pochisq};
use crate::stats::ent_result::EntResult;

/// Per-test pass/warn/fail status.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass = 0,
    Warn = 1,
    Fail = 2,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Pass => "PASS",
            Status::Warn => "WARN",
            Status::Fail => "FAIL",
        }
    }
}

/// Full guardrail evaluation result.
///
/// `#[repr(C)]` so it can be written to WASM linear memory from `ent_validate`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GuardrailEvaluation {
    pub overall_status: Status,
    pub entropy_status: Status,
    pub chi_square_status: Status,
    pub mean_status: Status,
    pub pi_status: Status,
    pub serial_correlation_status: Status,
    /// Lower critical value for chi-square at the given Î±.
    pub chi_square_lower_bound: f64,
    /// Upper critical value for chi-square at the given Î±.
    pub chi_square_upper_bound: f64,
    /// Probability that a random distribution exceeds the observed chi-square (0..1).
    pub pochisq_exceed_prob: f64,
    /// Absolute percentage error of the Monte Carlo Ï€ estimate.
    pub pi_error_percent: f64,
}

/// Evaluates `res` against statistical guardrails at significance level `alpha` (default 0.05).
pub fn evaluate_guardrails(res: &EntResult, alpha: f64) -> GuardrailEvaluation {
    let chip = pochisq(res.chi_square, 255);
    let pi_err =
        ((res.monte_carlo_pi - core::f64::consts::PI).abs() / core::f64::consts::PI) * 100.0;
    let scc_abs = res.serial_correlation.abs();
    let mean_diff = (res.mean - 127.5).abs();

    let chi_low = chi2_critical_value(1.0 - alpha / 2.0, 255.0);
    let chi_high = chi2_critical_value(alpha / 2.0, 255.0);

    // Shannon Entropy
    let entropy_status = if res.entropy_bits_per_byte >= 7.9 {
        Status::Pass
    } else {
        Status::Warn
    };

    // Chi-Square (two-tailed)
    let chi_square_status = if !(0.0001..=0.9999).contains(&chip) {
        Status::Fail
    } else if !(0.01..=0.99).contains(&chip) {
        Status::Warn
    } else {
        Status::Pass
    };

    // Arithmetic Mean
    let mean_status = if mean_diff >= 5.0 {
        Status::Fail
    } else if mean_diff >= 1.0 {
        Status::Warn
    } else {
        Status::Pass
    };

    // Monte Carlo Pi
    let pi_status = if pi_err >= 3.0 {
        Status::Fail
    } else if pi_err >= 1.0 {
        Status::Warn
    } else {
        Status::Pass
    };

    // Serial Correlation
    let serial_correlation_status = if res.serial_correlation < -90_000.0 || scc_abs >= 0.05 {
        Status::Fail
    } else if scc_abs >= 0.01 {
        Status::Warn
    } else {
        Status::Pass
    };

    let has_fail = chi_square_status == Status::Fail
        || mean_status == Status::Fail
        || pi_status == Status::Fail
        || serial_correlation_status == Status::Fail;

    let has_warn = !has_fail
        && (chi_square_status == Status::Warn
            || mean_status == Status::Warn
            || pi_status == Status::Warn
            || serial_correlation_status == Status::Warn
            || entropy_status == Status::Warn);

    let overall_status = if has_fail {
        Status::Fail
    } else if has_warn {
        Status::Warn
    } else {
        Status::Pass
    };

    GuardrailEvaluation {
        overall_status,
        entropy_status,
        chi_square_status,
        mean_status,
        pi_status,
        serial_correlation_status,
        chi_square_lower_bound: chi_low,
        chi_square_upper_bound: chi_high,
        pochisq_exceed_prob: chip,
        pi_error_percent: pi_err,
    }
}
