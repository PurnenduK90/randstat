//! Report formatters for Fourmilab ENT Suite.

use super::common::{fmt_chip, fmt_scc, format_bytes, sha256_hex};
use randstat_core::stats::ent_result::EntResult;
use randstat_core::stats::validate::GuardrailEvaluation;

pub fn verdict_label(eval: &GuardrailEvaluation) -> &'static str {
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
    println!("  \"suite\": \"ENT\",");
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
            "[{:.1}, {:.1}]",
            eval.chi_square_lower_bound, eval.chi_square_upper_bound
        ),
        chip_dev,
        eval.chi_square_status.as_str()
    );

    // Arithmetic Mean
    let mean_dev = res.mean - 127.5;
    let mean_dev_str = if mean_dev.abs() < 0.00005 {
        "Diff: ~ 0.0000".to_string()
    } else {
        format!("Diff: {:+.4}", mean_dev)
    };
    println!(
        "| Arithmetic Mean        | {:<19} | ~ 127.5000 (Mean)     | {:<18} | {:<6} |",
        format!("{:.4}", res.mean),
        mean_dev_str,
        eval.mean_status.as_str()
    );

    // Monte Carlo Pi
    let pi_err_str = format!("Err:  {:.2}%", eval.pi_error_percent);
    println!(
        "| Monte Carlo Pi (2D)    | {:<19} | ~ 3.141592654 (MC)    | {:<18} | {:<6} |",
        format!("{:.6}", res.monte_carlo_pi),
        pi_err_str,
        eval.pi_status.as_str()
    );

    // Serial Correlation
    let scc_dev_str = if res.serial_correlation < -90_000.0 {
        "Constant sequence".to_string()
    } else if res.serial_correlation.abs() >= 0.05 {
        "High correlation".to_string()
    } else {
        "Uncorrelated".to_string()
    };
    println!(
        "| Serial Correlation     | {:<19} | 0.000000 (Lag-1)      | {:<18} | {:<6} |",
        fmt_scc(res.serial_correlation),
        scc_dev_str,
        eval.serial_correlation_status.as_str()
    );

    println!("+------------------------+---------------------+-----------------------+--------------------+--------+");
    println!();
    println!("OVERALL VERDICT : {}", verdict);
    println!("==========================================================================================");
}
