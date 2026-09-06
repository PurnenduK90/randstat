//! Report formatters for `randstat-cli`.
//!
//! Organized modularly per test suite:
//! - [`common`] — size/hash formatting and table helpers
//! - [`ent`] — Fourmilab ENT suite reports
//! - [`nist`] — NIST SP 800-22 Rev 1a reports
//! - [`ais31`] — BSI AIS 20 / AIS 31 reports

pub mod ais31;
pub mod common;
pub mod ent;
pub mod nist;

// Re-exports for CLI
pub use ais31::{print_ais31_json, print_ais31_markdown, print_ais31_terminal};
pub use ent::{print_json, print_markdown, print_terminal};
pub use nist::{print_nist_json, print_nist_markdown, print_nist_terminal};

#[cfg(test)]
mod tests {
    use super::common::{format_bytes, sha256_hex};
    use super::*;
    use randstat_core::stats::ent_result::EntResult;
    use randstat_core::stats::validate::evaluate_guardrails;
    use randstat_suite_ais31::Ais31Suite;
    use randstat_suite_nist::NistSuite;

    #[test]
    fn test_report_formatters() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");

        let digest = [0u8; 32];
        assert_eq!(
            sha256_hex(&digest),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );

        let res = EntResult {
            total_bytes: 1000,
            entropy_bits_per_byte: 7.95,
            compression_percent: 0.5,
            chi_square: 250.0,
            mean: 127.4,
            monte_carlo_pi: std::f64::consts::PI,
            serial_correlation: 0.005,
            sha256: [0; 32],
        };
        let eval = evaluate_guardrails(&res, 0.05);

        // ENT
        print_markdown(&res, &eval, "test_file", 0.05);
        print_json(&res, &eval, "test_file", 0.05);
        print_terminal(&res, &eval, "test_file", 0.05, false);
        print_terminal(&res, &eval, "test_file", 0.05, true);

        // NIST
        let mut nist_suite = NistSuite::new();
        nist_suite.update(&[0xAA; 100]);
        let nist_eval = nist_suite.evaluate();
        print_nist_terminal(&nist_eval, "test_file", 100, &digest, 0.05);
        print_nist_markdown(&nist_eval, "test_file", 100, &digest, 0.05);
        print_nist_json(&nist_eval, "test_file", 100, &digest, 0.05);

        // AIS 31
        let mut ais31_suite = Ais31Suite::new();
        ais31_suite.update(&[0xAA; 100]);
        let ais31_eval = ais31_suite.evaluate();
        print_ais31_terminal(&ais31_eval, "test_file", 100, &digest, 0.05);
        print_ais31_markdown(&ais31_eval, "test_file", 100, &digest, 0.05);
        print_ais31_json(&ais31_eval, "test_file", 100, &digest, 0.05);
    }
}
