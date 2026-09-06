//! Byte-frequency histogram accumulator and Shannon entropy calculator.
//!
//! `ByteFreqTracker` counts how many times each byte value (0â€“255) appears
//! in the input stream and computes Shannon entropy in bits per byte.

use libm::log;

/// Tracks byte occurrence counts and computes Shannon entropy.
#[derive(Debug, Clone, Copy)]
pub struct ByteFreqTracker {
    /// Occurrence count for each byte value 0..=255.
    pub byte_counts: [u64; 256],
    /// Total bytes processed.
    pub total_bytes: u64,
}

impl ByteFreqTracker {
    /// Creates a new, zero-initialised `ByteFreqTracker`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            byte_counts: [0u64; 256],
            total_bytes: 0,
        }
    }

    /// Resets all counts to zero.
    #[inline]
    pub fn reset(&mut self) {
        self.byte_counts = [0u64; 256];
        self.total_bytes = 0;
    }

    /// Updates the frequency histogram with `slice`.
    #[inline]
    pub fn update(&mut self, slice: &[u8]) {
        self.total_bytes += slice.len() as u64;
        for &byte in slice {
            self.byte_counts[byte as usize] += 1;
        }
    }

    /// Computes Shannon entropy in bits per byte (0.0 = deterministic, 8.0 = maximum).
    pub fn entropy(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        let total = self.total_bytes as f64;
        let mut entropy = 0.0f64;
        for &count in self.byte_counts.iter() {
            if count > 0 {
                let p = (count as f64) / total;
                entropy -= p * (log(p) / core::f64::consts::LN_2);
            }
        }
        entropy
    }

    /// Computes arithmetic mean of all byte values seen so far.
    pub fn arithmetic_mean(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        let mut sum = 0u64;
        for (val, &count) in self.byte_counts.iter().enumerate() {
            sum += (val as u64) * count;
        }
        sum as f64 / self.total_bytes as f64
    }
}

impl Default for ByteFreqTracker {
    fn default() -> Self {
        Self::new()
    }
}
