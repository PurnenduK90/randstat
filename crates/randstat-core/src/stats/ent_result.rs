//! `EntResult` â€” the aggregate result struct for a full ENT-style evaluation.
//!
//! This is a flat `#[repr(C)]` struct that can be written directly into WASM
//! linear memory via pointer from JavaScript without any serialisation overhead.

use crate::bitstream::sha256::format_hex;

/// Aggregate statistical result from a complete ENT evaluation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EntResult {
    /// Total bytes evaluated.
    pub total_bytes: u64,
    /// Shannon entropy in bits per byte (0.0 = constant, 8.0 = maximum randomness).
    pub entropy_bits_per_byte: f64,
    /// Estimated compression reduction percentage (0.0%â€“100.0%).
    pub compression_percent: f64,
    /// Chi-square statistic across 256 byte bins.
    pub chi_square: f64,
    /// Arithmetic mean of all byte values (ideal random = 127.5).
    pub mean: f64,
    /// Monte Carlo estimate of Ï€.
    pub monte_carlo_pi: f64,
    /// Pearson serial correlation coefficient (0.0 = uncorrelated, sentinel -100_000.0 = constant).
    pub serial_correlation: f64,
    /// SHA-256 digest of the evaluated byte stream.
    pub sha256: [u8; 32],
}

impl EntResult {
    /// Returns the compression reduction as an integer percentage (0â€“100).
    #[inline]
    pub fn compression_reduction_int(&self) -> i16 {
        self.compression_percent as i16
    }

    /// Formats the SHA-256 digest as a 64-byte lowercase hex ASCII array.
    /// Usable in `no_std` contexts without `String`.
    pub fn sha256_hex_bytes(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        format_hex(&self.sha256, &mut out);
        out
    }
}
