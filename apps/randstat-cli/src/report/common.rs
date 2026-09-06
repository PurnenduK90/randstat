//! Common formatting helpers for CLI reports.

use randstat_core::traits::TestStatus;

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
pub fn fmt_chip(prob: f64) -> String {
    if prob < 0.0001 {
        "< 0.01%".to_string()
    } else if prob > 0.9999 {
        "> 99.99%".to_string()
    } else {
        format!("{:.2}%", prob * 100.0)
    }
}

/// Formats the serial correlation coefficient (handles the constant-stream sentinel).
pub fn fmt_scc(scc: f64) -> String {
    if scc < -90_000.0 {
        "Undefined".to_string()
    } else {
        format!("{:.6}", scc)
    }
}

/// Format test statistic or "N/A" if NaN.
pub fn fmt_stat(stat: f64) -> String {
    if stat.is_nan() {
        "N/A".to_string()
    } else {
        format!("{:.4}", stat)
    }
}

/// Format p-value or "N/A" if NaN.
pub fn fmt_pval(p_val: f64) -> String {
    if p_val.is_nan() {
        "N/A".to_string()
    } else {
        format!("{:.4}", p_val)
    }
}

/// Format status badge for terminal display.
pub fn status_badge(status: TestStatus) -> &'static str {
    match status {
        TestStatus::Passed => "PASS",
        TestStatus::Failed => "FAIL",
        TestStatus::NotImplemented => "NOT IMPLEMENTED",
        TestStatus::InsufficientData => "INSUFFICIENT DATA",
    }
}
