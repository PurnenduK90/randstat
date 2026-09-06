//! Xoshiro256** and Xoroshiro128+ fast pseudo-random number generators.
//!
//! Modern, high-speed, general-purpose simulation PRNGs by David Blackman and Sebastiano Vigna.

use super::traits::ByteGenerator;
use crate::math::transform::{splitmix64, xoshiro256_next};

/// Xoshiro256** generator state ($2^{256}-1$ period).
#[derive(Debug, Clone)]
pub struct Xoshiro256StarStar {
    s: [u64; 4],
    initial_s: [u64; 4],
    buf: [u8; 8],
    buf_idx: usize,
}

impl Xoshiro256StarStar {
    /// Creates a new Xoshiro256** generator from a 64-bit seed using SplitMix64 initialization.
    pub fn from_seed(seed: u64) -> Self {
        let mut sm_state = seed;
        let s = [
            splitmix64(&mut sm_state),
            splitmix64(&mut sm_state),
            splitmix64(&mut sm_state),
            splitmix64(&mut sm_state),
        ];
        Self {
            s,
            initial_s: s,
            buf: [0u8; 8],
            buf_idx: 8,
        }
    }

    /// Generates next 64-bit random word.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        xoshiro256_next(&mut self.s)
    }

    /// Generates uniform float in `[0.0, 1.0)`.
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}

impl Default for Xoshiro256StarStar {
    fn default() -> Self {
        Self::from_seed(0x853C49E6748FEA9B)
    }
}

impl ByteGenerator for Xoshiro256StarStar {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        if self.buf_idx >= 8 {
            let val = self.next_u64();
            self.buf = val.to_le_bytes();
            self.buf_idx = 0;
        }
        let b = self.buf[self.buf_idx];
        self.buf_idx += 1;
        b
    }

    fn reset(&mut self) {
        self.s = self.initial_s;
        self.buf_idx = 8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xoshiro() {
        let mut rng = Xoshiro256StarStar::from_seed(12345);
        let mut buf = [0u8; 64];
        rng.fill_bytes(&mut buf);
        assert_ne!(buf, [0u8; 64]);

        rng.reset();
        let mut buf2 = [0u8; 64];
        rng.fill_bytes(&mut buf2);
        assert_eq!(buf, buf2);
    }
}
