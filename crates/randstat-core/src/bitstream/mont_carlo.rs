//! Monte Carlo Pi estimation accumulator.
//!
//! Samples 6-byte coordinate pairs `(x, y)` where each coordinate is a 24-bit
//! integer. Counts how many pairs fall inside the unit circle to estimate Ï€.
//!
//! Algorithm matches the Fourmilab ENT reference implementation.

/// Tracks 2D coordinate samples to estimate Ï€ via Monte Carlo integration.
#[derive(Debug, Clone, Copy)]
pub struct MonteCarloAccum {
    /// Number of 6-byte coordinate pairs falling inside the unit circle.
    pub inside: u64,
    /// Total number of 6-byte coordinate pairs evaluated.
    pub total: u64,
    /// Partial byte buffer (up to 5 bytes carried across `update` calls).
    pub buffer: [u8; 6],
    /// Number of valid bytes currently in `buffer`.
    pub buf_len: usize,
}

impl MonteCarloAccum {
    /// Creates a new, zero-initialised `MonteCarloAccum`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            inside: 0,
            total: 0,
            buffer: [0u8; 6],
            buf_len: 0,
        }
    }

    /// Resets sample counters and byte buffer to zero.
    #[inline]
    pub fn reset(&mut self) {
        self.inside = 0;
        self.total = 0;
        self.buffer = [0u8; 6];
        self.buf_len = 0;
    }

    /// Feeds bytes into the accumulator; processes complete 6-byte groups.
    pub fn update(&mut self, slice: &[u8]) {
        // (256^3 - 1)^2 â€” the squared radius of the 24-bit unit circle
        const INCIRC: f64 = 16_777_215.0 * 16_777_215.0;

        for &byte in slice {
            self.buffer[self.buf_len] = byte;
            self.buf_len += 1;

            if self.buf_len == 6 {
                let x = ((self.buffer[0] as f64) * 256.0 + (self.buffer[1] as f64)) * 256.0
                    + (self.buffer[2] as f64);
                let y = ((self.buffer[3] as f64) * 256.0 + (self.buffer[4] as f64)) * 256.0
                    + (self.buffer[5] as f64);

                if x * x + y * y <= INCIRC {
                    self.inside += 1;
                }
                self.total += 1;
                self.buf_len = 0;
            }
        }
    }

    /// Returns the current Monte Carlo estimate of Ï€.
    /// Returns `0.0` if no complete coordinate pairs have been processed.
    #[inline]
    pub fn pi(&self) -> f64 {
        if self.total > 0 {
            4.0 * (self.inside as f64) / (self.total as f64)
        } else {
            0.0
        }
    }
}

impl Default for MonteCarloAccum {
    fn default() -> Self {
        Self::new()
    }
}
