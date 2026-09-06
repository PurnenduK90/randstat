//! Pearson serial correlation coefficient accumulator for consecutive byte pairs.
//!
//! Implements the circular pair algorithm from the Fourmilab ENT reference:
//! the last byte is paired with the first byte to close the sequence ring.

/// Accumulates terms for the Pearson serial correlation coefficient.
#[derive(Debug, Clone, Copy)]
pub struct SerialCorrAccum {
    /// Î£ xáµ¢ Â· xáµ¢â‚Šâ‚ (product of consecutive bytes).
    pub scct1: f64,
    /// Î£ xáµ¢ (sum of byte values).
    pub scct2: f64,
    /// Î£ xáµ¢Â² (sum of squared byte values).
    pub scct3: f64,
    /// First byte seen â€” used for the circular end-to-start wrap.
    pub first_byte: Option<u8>,
    /// Last byte seen in the stream.
    pub last_byte: Option<u8>,
    /// Total bytes processed.
    pub totalc: u64,
}

impl SerialCorrAccum {
    /// Creates a new, zero-initialised `SerialCorrAccum`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            scct1: 0.0,
            scct2: 0.0,
            scct3: 0.0,
            first_byte: None,
            last_byte: None,
            totalc: 0,
        }
    }

    /// Resets all accumulators to zero.
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Updates accumulator terms with `slice`.
    pub fn update(&mut self, slice: &[u8]) {
        for &byte in slice {
            let x = byte as f64;
            self.totalc += 1;

            if self.first_byte.is_none() {
                self.first_byte = Some(byte);
            } else if let Some(last) = self.last_byte {
                self.scct1 += (last as f64) * x;
            }

            self.scct2 += x;
            self.scct3 += x * x;
            self.last_byte = Some(byte);
        }
    }

    /// Returns the Pearson serial correlation coefficient.
    ///
    /// Returns `-100_000.0` (sentinel) when the denominator is zero (constant stream).
    /// Returns `0.0` if no data has been processed.
    pub fn correlation(&self) -> f64 {
        if self.totalc == 0 {
            return 0.0;
        }
        let first = match self.first_byte {
            Some(b) => b as f64,
            None => return 0.0,
        };
        let last = match self.last_byte {
            Some(b) => b as f64,
            None => return 0.0,
        };

        let n = self.totalc as f64;
        let scct1 = self.scct1 + (last * first); // circular wrap
        let scct2_sq = self.scct2 * self.scct2;
        let denominator = n * self.scct3 - scct2_sq;

        if denominator == 0.0 {
            -100_000.0
        } else {
            (n * scct1 - scct2_sq) / denominator
        }
    }
}

impl Default for SerialCorrAccum {
    fn default() -> Self {
        Self::new()
    }
}
