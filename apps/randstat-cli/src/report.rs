//! Report formatters for `randstat-cli`.
//!
//! All string formatting and output logic lives here, keeping `main.rs` clean.
//! Three output modes are supported:
//! - [`print_terminal`] — padded ASCII box (default)
//! - [`print_markdown`] — GitHub-Flavoured Markdown table
//! - [`print_json`] — pretty-printed JSON

use randstat_core::stats::ent_result::EntResult;
use randstat_core::stats::validate::GuardrailEvaluation;

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Formats a byte count as a human-readable size string.
pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1 << 30 {
        format!("{:.2} GB", bytes as f64 / (1 << 30) as f64)
    } else if bytes >= 1 << 20 {
        format!("{:.2} MB", bytes as f64 / (1 << 20) as f64)
    } else if bytes >= 1 << 10 {
        format!("{:.2} KB", bytes as f64 / (1 << 10) as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Formats a 32-byte SHA-256 digest as a 64-character lowercase hex `String`.
pub fn sha256_hex(digest: &[u8; 32]) -> String {
    let hex_bytes = {
        let mut buf = [0u8; 64];
        randstat_core::bitstream::sha256::format_hex(digest, &mut buf);
        buf
    };
    String::from_utf8_lossy(&hex_bytes).into_owned()
}

/// Formats the chi-square exceedance probability as a percentage string.
fn fmt_chip(prob: f64) -> String {
    if prob < 0.0001 {
        "< 0.01%".to_string()
    } else if prob > 0.9999 {
        "> 99.99%".to_string()
    } else {
        format!("{:.2}%", prob * 100.0)
    }
}

/// Formats the serial correlation coefficient (handles the constant-stream sentinel).
fn fmt_scc(scc: f64) -> String {
    if scc < -90_000.0 {
        "Undefined".to_string()
    } else {
        format!("{:.6}", scc)
    }
}

// ─── Output Modes ─────────────────────────────────────────────────────────────

/// Outputs a GitHub-Flavoured Markdown randomness report.
pub fn print_markdown(res: &EntResult, eval: &GuardrailEvaluation, file: &str, alpha: f64) {
    let size = format_bytes(res.total_bytes);
    let sha = sha256_hex(&res.sha256);
    let chip_str = fmt_chip(eval.pochisq_exceed_prob);
    let scc_str = fmt_scc(res.serial_correlation);
    let scc_margin = if res.serial_correlation < -90_000.0 || res.serial_correlation.abs() >= 0.05 {
        "⚠ High Correlation"
    } else {
        "Uncorrelated"
    };

    let verdict = verdict_label(eval);

    println!("# randstat Randomness Evaluation Report\n");
    println!(
        "* **File:** `{}` ({} / {} bytes)",
        file, size, res.total_bytes
    );
    println!("* **SHA-256:** `{}`", sha);
    println!("* **Overall Verdict:** `{}`\n", verdict);
    println!("---\n");

    println!("## Metric Breakdown\n");
    println!("| Test Metric | Calculated Value | Ideal Random Val | Deviation / Margin | Status |");
    println!("| :--- | :--- | :--- | :--- | :--- |");
    println!(
        "| **Shannon Entropy** | `{:.6}` b/B | 8.000000 bits/byte | Redundancy: {:.2}% | `{}` |",
        res.entropy_bits_per_byte,
        res.compression_percent,
        eval.entropy_status.as_str()
    );
    println!(
        "| **Chi-Square (\\(\\chi^2\\))** | `{:.2}` (df=255) | 255.000000 | Exceedance: {:<7} | `{}` |",
        res.chi_square, chip_str, eval.chi_square_status.as_str()
    );
    println!(
        "| **Arithmetic Mean** | `{:.4}` | 127.500000 | Diff: {:+.4} | `{}` |",
        res.mean,
        res.mean - 127.5,
        eval.mean_status.as_str()
    );
    println!(
        "| **Monte Carlo \\(\\pi\\)** | `{:.6}` | 3.141592654 | Error: {:.2}% | `{}` |",
        res.monte_carlo_pi,
        eval.pi_error_percent,
        eval.pi_status.as_str()
    );
    println!(
        "| **Serial Correlation** | `{}` | 0.000000 | {:<18} | `{}` |",
        scc_str,
        scc_margin,
        eval.serial_correlation_status.as_str()
    );

    println!("\n---\n");
    println!(
        "## Test Interpretation & Guardrails (\\(\\alpha = {:.2}\\))\n",
        alpha
    );
    println!("> **Note on Chi-Square Interpretation:**  ");
    println!(
        "> A random sequence passes if {:.2} \\(\\le \\chi^2 \\le\\) {:.2} ({:.1}% \\(\\le \\text{{Exceedance}} \\le\\) {:.1}%).  ",
        eval.chi_square_lower_bound,
        eval.chi_square_upper_bound,
        (alpha / 2.0) * 100.0,
        (1.0 - alpha / 2.0) * 100.0
    );
    println!(
        "> * \\(\\chi^2 > {:.2}\\): Non-uniform / biased sequence.",
        eval.chi_square_upper_bound
    );
    println!(
        "> * \\(\\chi^2 < {:.2}\\): Artificially forced / overly uniform sequence.\n",
        eval.chi_square_lower_bound
    );
    println!("* **Shannon Entropy:** Measures information density. Higher is more random (\\(8.0\\) = completely uncompressible).");
    println!("* **Arithmetic Mean:** Average byte value. Pure random byte streams centre on \\(127.5000\\).");
    println!("* **Monte Carlo \\(\\pi\\):** Evaluates 2D spatial clustering using 6-byte coordinate pairs. Error \\(< 3.0\\%\\) passes.");
    println!("* **Serial Correlation:** Measures sequence memory (\\(x_i\\) vs \\(x_{{i+1}}\\)). Values near \\(0.0\\) indicate independent bytes.");
}

/// Outputs a pretty-printed JSON report.
pub fn print_json(res: &EntResult, eval: &GuardrailEvaluation, file: &str, alpha: f64) {
    let sha = sha256_hex(&res.sha256);
    let chip_str = fmt_chip(eval.pochisq_exceed_prob);
    let scc_str = fmt_scc(res.serial_correlation);
    let verdict = verdict_label(eval);

    let escaped_file = file.replace('\\', "/");
    println!("{{");
    println!("  \"file\": \"{}\",", escaped_file);
    println!("  \"total_bytes\": {},", res.total_bytes);
    println!("  \"sha256\": \"{}\",", sha);
    println!("  \"verdict\": \"{}\",", verdict);
    println!("  \"alpha\": {},", alpha);
    println!("  \"metrics\": {{");
    println!(
        "    \"shannon_entropy_bits_per_byte\": {:.6},",
        res.entropy_bits_per_byte
    );
    println!(
        "    \"optimum_compression_percent\": {:.2},",
        res.compression_percent
    );
    println!("    \"chi_square_statistic\": {:.2},", res.chi_square);
    println!("    \"chi_square_degrees_of_freedom\": 255,");
    println!("    \"chi_square_exceed_probability\": \"{}\",", chip_str);
    println!("    \"arithmetic_mean\": {:.4},", res.mean);
    println!("    \"monte_carlo_pi\": {:.9},", res.monte_carlo_pi);
    println!(
        "    \"monte_carlo_pi_error_percent\": {:.2},",
        eval.pi_error_percent
    );
    println!("    \"serial_correlation\": \"{}\"", scc_str);
    println!("  }},");
    println!("  \"status\": {{");
    println!("    \"entropy\": \"{}\",", eval.entropy_status.as_str());
    println!(
        "    \"chi_square\": \"{}\",",
        eval.chi_square_status.as_str()
    );
    println!("    \"mean\": \"{}\",", eval.mean_status.as_str());
    println!("    \"pi\": \"{}\",", eval.pi_status.as_str());
    println!(
        "    \"serial_correlation\": \"{}\"",
        eval.serial_correlation_status.as_str()
    );
    println!("  }}");
    println!("}}");
}

/// Outputs the default padded terminal box report.
pub fn print_terminal(
    res: &EntResult,
    eval: &GuardrailEvaluation,
    file: &str,
    alpha: f64,
    ascii_mode: bool,
) {
    let size = format_bytes(res.total_bytes);
    let sha = sha256_hex(&res.sha256);
    let chip_pct = eval.pochisq_exceed_prob * 100.0;
    let alpha_pct = format!("{:.0}%", alpha * 100.0);
    let verdict = verdict_label(eval);
    let input_mode = if ascii_mode {
        "ASCII Text (One value / line)"
    } else {
        "Binary (Raw Byte Stream)"
    };

    println!("==========================================================================================");
    println!(
        "                         RANDSTAT RANDOMNESS EVALUATION REPORT                           "
    );
    println!("==========================================================================================");
    println!("File / Input Source : {}", file);
    println!("Input Data Mode     : {}", input_mode);
    println!("Evaluated Stream    : {} bytes ({})", res.total_bytes, size);
    println!("SHA-256 Hash        : {}", sha);
    println!(
        "Significance Alpha  : α = {:.4} ({}% Confidence)\n",
        alpha,
        (1.0 - alpha) * 100.0
    );

    println!("+------------------------+---------------------+-----------------------+--------------------+--------+");
    println!(
        "| Test Metric            | Calculated Value    | Ideal Range (α={:<5}) | Deviation / Exceed | Status |",
        alpha_pct
    );
    println!("+------------------------+---------------------+-----------------------+--------------------+--------+");

    // Shannon Entropy
    println!(
        "| Shannon Entropy        | {:<19} | ~ 8.000000 bits/byte  | {:<18} | {:<6} |",
        format!("{:.6} b/B", res.entropy_bits_per_byte),
        format!("Compress: {:.2}%", res.compression_percent),
        eval.entropy_status.as_str()
    );

    // Chi-Square
    let chip_dev = if eval.pochisq_exceed_prob < 0.0001 {
        "Exceed: < 0.01%".to_string()
    } else if eval.pochisq_exceed_prob > 0.9999 {
        "Exceed: > 99.99%".to_string()
    } else {
        format!("Exceed: {:.2}%", chip_pct)
    };
    println!(
        "| Chi-Square (df=255)    | {:<19} | {:<21} | {:<18} | {:<6} |",
        format!("{:.2}", res.chi_square),
        format!(
            "[{:.2} - {:.2}]",
            eval.chi_square_lower_bound, eval.chi_square_upper_bound
        ),
        chip_dev,
        eval.chi_square_status.as_str()
    );

    // Arithmetic Mean
    println!(
        "| Arithmetic Mean        | {:<19} | ~ 127.500000          | {:<18} | {:<6} |",
        format!("{:.4}", res.mean),
        format!("Diff: {:+.4}", res.mean - 127.5),
        eval.mean_status.as_str()
    );

    // Monte Carlo Pi
    println!(
        "| Monte Carlo Pi         | {:<19} | ~ 3.141592654         | {:<18} | {:<6} |",
        format!("{:.9}", res.monte_carlo_pi),
        format!("Error: {:.2}%", eval.pi_error_percent),
        eval.pi_status.as_str()
    );

    // Serial Correlation
    let scc_str = fmt_scc(res.serial_correlation);
    let scc_dev = if res.serial_correlation < -90_000.0 || res.serial_correlation.abs() >= 0.05 {
        "⚠ High Correlation"
    } else {
        "Uncorrelated"
    };
    println!(
        "| Serial Correlation     | {:<19} | [-0.010000, 0.010000] | {:<18} | {:<6} |",
        scc_str,
        scc_dev,
        eval.serial_correlation_status.as_str()
    );

    println!("+------------------------+---------------------+-----------------------+--------------------+--------+");
    println!();
    println!("OVERALL VERDICT: [{}]", verdict);
    println!("\nTEST INTERPRETATION & GUIDE:");
    println!(" - Shannon Entropy    : Higher is more random. Ideal = 8.0 bits/byte.");
    println!(
        " - Chi-Square Test    : At α={:.4} (df=255), valid range [{:.2}, {:.2}].",
        alpha, eval.chi_square_lower_bound, eval.chi_square_upper_bound
    );
    println!(" - Arithmetic Mean    : Random byte streams average ~127.5000.");
    println!(" - Monte Carlo Pi     : Error < 3.0% indicates randomness.");
    println!(" - Serial Correlation : Values near 0.0 indicate no correlation.");
    println!("==========================================================================================");
}

// ─── Internal ────────────────────────────────────────────────────────────────

fn verdict_label(eval: &GuardrailEvaluation) -> &'static str {
    use randstat_core::stats::validate::Status;
    match eval.overall_status {
        Status::Pass => "✓ LIKELY RANDOM",
        Status::Warn => "⚠ ARTIFICIAL UNIFORMITY/STRUCTURED",
        Status::Fail => {
            if eval.chi_square_status == Status::Fail && eval.pochisq_exceed_prob > 0.9999 {
                "⚠ ARTIFICIAL UNIFORMITY/STRUCTURED"
            } else {
                "⚠ LIKELY BIASED/ANOMALY"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use randstat_core::stats::ent_result::EntResult;
    use randstat_core::stats::validate::evaluate_guardrails;

    #[test]
    fn test_report_formatters() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");

        let digest = [0u8; 32];
        assert_eq!(sha256_hex(&digest), "0000000000000000000000000000000000000000000000000000000000000000");

        assert_eq!(fmt_chip(0.00001), "< 0.01%");
        assert_eq!(fmt_chip(0.99999), "> 99.99%");
        assert_eq!(fmt_chip(0.5), "50.00%");

        assert_eq!(fmt_scc(-90001.0), "Undefined");
        assert_eq!(fmt_scc(0.001), "0.001000");

        let res = EntResult {
            total_bytes: 1000,
            entropy_bits_per_byte: 7.95,
            compression_percent: 0.5,
            chi_square: 250.0,
            mean: 127.4,
            monte_carlo_pi: 3.1415,
            serial_correlation: 0.005,
            sha256: [0; 32],
        };
        let eval = evaluate_guardrails(&res, 0.05);

        // Call print functions to ensure they run and don't panic
        print_markdown(&res, &eval, "test_file", 0.05);
        print_json(&res, &eval, "test_file", 0.05);
        print_terminal(&res, &eval, "test_file", 0.05, false);
        print_terminal(&res, &eval, "test_file", 0.05, true);

        // Test other status paths in verdict_label
        let res_fail_chi_high = EntResult {
            chi_square: 0.0, // perfect uniform -> pochisq exceed prob = 1.0 > 0.9999
            ..res
        };
        let eval_fail_chi_high = evaluate_guardrails(&res_fail_chi_high, 0.05);
        // This triggers the other branches of verdict_label
        print_terminal(&res_fail_chi_high, &eval_fail_chi_high, "test_file", 0.05, false);

        let res_warn_chi = EntResult {
            chi_square: 10.0,
            ..res
        };
        let eval_warn_chi = evaluate_guardrails(&res_warn_chi, 0.05);
        print_terminal(&res_warn_chi, &eval_warn_chi, "test_file", 0.05, false);
    }
}
