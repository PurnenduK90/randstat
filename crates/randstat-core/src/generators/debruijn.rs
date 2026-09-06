//! De Bruijn binary sequence generator of order $n$ (period $2^n$).
//!
//! Produces full $2^n$ de Bruijn cycles containing every $n$-bit subsequence exactly once.
//! Uses $O(1)$ memory by modifying the primitive feedback polynomial at the boundary state.

use super::traits::ByteGenerator;

/// Configuration and state for a De Bruijn sequence generator.
#[derive(Debug, Clone)]
pub struct DeBruijnGenerator {
    order: u32,
    state: u32,
    initial_state: u32,
}

impl DeBruijnGenerator {
    /// Creates a new De Bruijn generator of the specified order (supported: 4, 9, 13, 19, 31).
    pub fn new(order: u32) -> Self {
        let valid_order = match order {
            4 | 9 | 13 | 19 | 31 => order,
            _ => 19,
        };
        Self {
            order: valid_order,
            state: 1,
            initial_state: 1,
        }
    }

    /// Step one single bit through the De Bruijn sequence.
    #[inline]
    pub fn step_bit(&mut self) -> u8 {
        let (next_state, bit) = crate::math::transform::debruijn_step_bit(self.state, self.order);
        self.state = next_state;
        bit
    }
}

impl Default for DeBruijnGenerator {
    fn default() -> Self {
        Self::new(19)
    }
}

impl ByteGenerator for DeBruijnGenerator {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        let mut b = 0u8;
        for shift in (0..8).rev() {
            let bit = self.step_bit();
            b |= bit << shift;
        }
        b
    }

    fn reset(&mut self) {
        self.state = self.initial_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debruijn_order4_period() {
        // Order 4: 2^4 = 16 bits
        let mut db = DeBruijnGenerator::new(4);
        let mut bits = [0u8; 16];
        for b in &mut bits {
            *b = db.step_bit();
        }

        // Count ones and zeros: exactly 8 ones and 8 zeros
        let ones: u32 = bits.iter().map(|&b| b as u32).sum();
        assert_eq!(ones, 8);

        // Sequence must repeat identically
        for &expected in &bits {
            assert_eq!(db.step_bit(), expected);
        }
    }

    #[test]
    fn test_debruijn_fill() {
        let mut db = DeBruijnGenerator::new(9);
        let mut buf = [0u8; 64];
        db.fill_bytes(&mut buf);
        assert_ne!(buf, [0u8; 64]);

        db.reset();
        let mut buf2 = [0u8; 64];
        db.fill_bytes(&mut buf2);
        assert_eq!(buf, buf2);
    }
}
