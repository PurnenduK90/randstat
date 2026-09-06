//! Linear Feedback Shift Register (LFSR) byte stream generator.
//!
//! Supports orders 3, 4, 8, 9, 11, 13, 19, 27, 31, 32, 64, 96, and 128 using
//! primitive feedback polynomials in `#![no_std]` zero-allocation mode ($O(1)$ memory).

use super::traits::ByteGenerator;

/// Configuration and state for an LFSR generator.
#[derive(Debug, Clone)]
pub struct LfsrGenerator {
    order: u32,
    state: u128,
    initial_state: u128,
}

impl LfsrGenerator {
    /// Creates a new LFSR generator for the specified order and seed state.
    ///
    /// If seed is 0, defaults to 1 (all-zero state is lock-up in classical LFSR).
    pub fn new(order: u32, seed: u128) -> Self {
        let initial_state = if seed == 0 { 1 } else { seed };
        let mask = if order >= 128 {
            !0u128
        } else {
            (1u128 << order) - 1
        };
        let valid_state = initial_state & mask;
        let actual_state = if valid_state == 0 { 1 } else { valid_state };

        Self {
            order,
            state: actual_state,
            initial_state: actual_state,
        }
    }

    /// Step one single bit through the LFSR.
    #[inline]
    pub fn step_bit(&mut self) -> u8 {
        let (next_state, bit) = crate::math::transform::lfsr_step_bit(self.state, self.order);
        self.state = next_state;
        bit
    }
}

impl Default for LfsrGenerator {
    fn default() -> Self {
        Self::new(19, 1)
    }
}

impl ByteGenerator for LfsrGenerator {
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
    fn test_lfsr_orders() {
        for order in [3, 4, 8, 9, 11, 13, 19, 27, 31, 32, 64, 96, 128] {
            let mut lfsr = LfsrGenerator::new(order, 1);
            let mut buf = [0u8; 32];
            lfsr.fill_bytes(&mut buf);
            assert_ne!(buf, [0u8; 32]);

            lfsr.reset();
            let b0 = lfsr.next_byte();
            assert_eq!(b0, buf[0]);
        }
    }

    #[test]
    fn test_lfsr_order3_period() {
        // Order 3 has period 2^3 - 1 = 7 bits
        let mut lfsr = LfsrGenerator::new(3, 1);
        let mut bits = [0u8; 7];
        for b in &mut bits {
            *b = lfsr.step_bit();
        }
        // Next bit should repeat the sequence
        assert_eq!(lfsr.step_bit(), bits[0]);
        assert_eq!(lfsr.step_bit(), bits[1]);
    }
}
