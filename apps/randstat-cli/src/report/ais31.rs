//! Report formatters for BSI AIS 20 / AIS 31 Physical TRNG Suite.

use super::common::{fmt_pval, fmt_stat, format_bytes, sha256_hex, status_badge};
use randstat_core::traits::TestStatus;
use randstat_suite_ais31::Ais31Evaluation;

/// Print BSI AIS 20 / AIS 31 Terminal Report.
pub fn print_ais31_terminal(
    eval: &Ais31Evaluation,
    file: &str,
    total_bytes: u64,
    sha256: &[u8; 32],
    alpha: f64,
) {
    let size = format_bytes(total_bytes);
    let sha = sha256_hex(sha256);

    println!("==========================================================================================");
    println!("                    BSI AIS 20 / AIS 31 PHYSICAL TRNG EVALUATION REPORT                   ");
    println!("==========================================================================================");
    println!("File / Input Source : {}", file);
    println!("Evaluated Stream    : {} bytes ({})", total_bytes, size);
    println!("SHA-256 Hash        : {}", sha);
    println!("Significance Alpha  : α = {:.4}\n", alpha);

    println!("+-----+---------------------------------------+-----------+-----------+--------------------+");
    println!("| Test| Test Name                             | Statistic | p-value   | Status             |");
    println!("+-----+---------------------------------------+-----------+-----------+--------------------+");

    for entry in &eval.entries {
        println!(
            "| {:<3} | {:<37} | {:<9} | {:<9} | {:<18} |",
            entry.standard_id,
            entry.name,
            fmt_stat(entry.result.statistic),
            fmt_pval(entry.result.p_value),
            status_badge(entry.result.status)
        );
    }

    println!("+-----+---------------------------------------+-----------+-----------+--------------------+");
    println!();
    println!(
        "SUMMARY: Implemented: {}/{} | Passed: {} | Failed: {} | Not Implemented: {}",
        eval.implemented_count,
        eval.total_tests,
        eval.passed_count,
        eval.failed_count,
        eval.skipped_count
    );
    println!("==========================================================================================");
}

/// Print BSI AIS 20 / AIS 31 Markdown Report.
pub fn print_ais31_markdown(
    eval: &Ais31Evaluation,
    file: &str,
    total_bytes: u64,
    sha256: &[u8; 32],
    alpha: f64,
) {
    let size = format_bytes(total_bytes);
    let sha = sha256_hex(sha256);

    println!("# BSI AIS 20 / AIS 31 Physical TRNG Evaluation Report\n");
    println!("* **File:** `{}` ({} / {} bytes)", file, size, total_bytes);
    println!("* **SHA-256:** `{}`", sha);
    println!("* **Significance Level (α):** `{}`\n", alpha);
    println!("---\n");

    println!("## Test Results\n");
    println!("| ID | Test Name | Statistic | p-value | Status |");
    println!("| :--- | :--- | :--- | :--- | :--- |");

    for entry in &eval.entries {
        let badge = match entry.result.status {
            TestStatus::Passed => "✅ PASS",
            TestStatus::Failed => "❌ FAIL",
            TestStatus::NotImplemented => "🔧 NOT IMPLEMENTED",
            TestStatus::InsufficientData => "⚠️ INSUFFICIENT DATA",
        };
        println!(
            "| `{}` | **{}** | `{}` | `{}` | {} |",
            entry.standard_id,
            entry.name,
            fmt_stat(entry.result.statistic),
            fmt_pval(entry.result.p_value),
            badge
        );
    }

    println!("\n---\n");
    println!(
        "**Summary:** Implemented: `{}/{}` | Passed: `{}` | Failed: `{}` | Stubs: `{}`\n",
        eval.implemented_count,
        eval.total_tests,
        eval.passed_count,
        eval.failed_count,
        eval.skipped_count
    );
}

/// Print BSI AIS 20 / AIS 31 JSON Report.
pub fn print_ais31_json(
    eval: &Ais31Evaluation,
    file: &str,
    total_bytes: u64,
    sha256: &[u8; 32],
    alpha: f64,
) {
    let sha = sha256_hex(sha256);
    let escaped_file = file.replace('\\', "/");

    println!("{{");
    println!("  \"suite\": \"BSI AIS 20 / AIS 31\",");
    println!("  \"file\": \"{}\",", escaped_file);
    println!("  \"total_bytes\": {},", total_bytes);
    println!("  \"sha256\": \"{}\",", sha);
    println!("  \"alpha\": {},", alpha);
    println!("  \"summary\": {{");
    println!("    \"total\": {},", eval.total_tests);
    println!("    \"implemented\": {},", eval.implemented_count);
    println!("    \"passed\": {},", eval.passed_count);
    println!("    \"failed\": {},", eval.failed_count);
    println!("    \"skipped\": {}", eval.skipped_count);
    println!("  }},");
    println!("  \"tests\": [");
    for (i, entry) in eval.entries.iter().enumerate() {
        let comma = if i + 1 < eval.entries.len() { "," } else { "" };
        println!("    {{");
        println!("      \"id\": \"{}\",", entry.standard_id);
        println!("      \"name\": \"{}\",", entry.name);
        println!(
            "      \"statistic\": \"{}\",",
            fmt_stat(entry.result.statistic)
        );
        println!("      \"p_value\": \"{}\",", fmt_pval(entry.result.p_value));
        println!(
            "      \"status\": \"{}\"",
            status_badge(entry.result.status)
        );
        println!("    }}{}", comma);
    }
    println!("  ]");
    println!("}}");
}
