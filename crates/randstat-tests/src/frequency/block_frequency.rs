//! NIST SP800-22 §2.2 — Block Frequency test.
//!
//! Divides the bit stream into non-overlapping blocks of M=128 bits (16 bytes) and tests
//! whether the proportion of 1s in each block is approximately M/2 = 64.

use randstat_core::algorithms::block_frequency::nist_block_frequency;
use randstat_core::traits::{StreamTest, TestResult};

pub const BLOCK_BITS: usize = 128;
pub const BLOCK_BYTES: usize = 16;
pub const BLOCK_HALF_BITS: i32 = (BLOCK_BITS / 2) as i32;

/// Block frequency test streaming accumulator (M = 128 bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockFrequencyTest {
    pub num_blocks: u64,
    pub sum_sq_diff: u64,
    pub partial_bytes: [u8; BLOCK_BYTES],
    pub partial_len: usize,
    pub total_bits: u64,
}

impl BlockFrequencyTest {
    pub const ZERO: Self = Self {
        num_blocks: 0,
        sum_sq_diff: 0,
        partial_bytes: [0u8; BLOCK_BYTES],
        partial_len: 0,
        total_bits: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }
}

impl Default for BlockFrequencyTest {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamTest for BlockFrequencyTest {
    fn update(&mut self, chunk: &[u8]) {
        if chunk.is_empty() {
            return;
        }
        self.total_bits += (chunk.len() as u64) * 8;
        let mut offset = 0;

        // Complete partial block if any
        if self.partial_len > 0 {
            let needed = BLOCK_BYTES - self.partial_len;
            let available = chunk.len().min(needed);
            self.partial_bytes[self.partial_len..self.partial_len + available]
                .copy_from_slice(&chunk[..available]);
            self.partial_len += available;
            offset += available;

            if self.partial_len == BLOCK_BYTES {
                let mut ones = 0i32;
                for &b in &self.partial_bytes {
                    ones += b.count_ones() as i32;
                }
                let diff = ones - BLOCK_HALF_BITS;
                self.sum_sq_diff += (diff * diff) as u64;
                self.num_blocks += 1;
                self.partial_len = 0;
            }
        }

        // Process full 16-byte blocks
        let remaining = &chunk[offset..];
        let num_full_blocks = remaining.len() / BLOCK_BYTES;
        let full_bytes = num_full_blocks * BLOCK_BYTES;

        for block in remaining[..full_bytes].chunks_exact(BLOCK_BYTES) {
            let mut ones = 0i32;
            for &b in block {
                ones += b.count_ones() as i32;
            }
            let diff = ones - BLOCK_HALF_BITS;
            self.sum_sq_diff += (diff * diff) as u64;
            self.num_blocks += 1;
        }

        // Store remainder in partial_bytes
        let rem = &remaining[full_bytes..];
        if !rem.is_empty() {
            self.partial_bytes[..rem.len()].copy_from_slice(rem);
            self.partial_len = rem.len();
        }
    }

    fn reset(&mut self) {
        *self = Self::ZERO;
    }

    fn evaluate(&self) -> TestResult {
        nist_block_frequency(self.num_blocks, self.sum_sq_diff, BLOCK_BITS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use randstat_core::traits::TestStatus;

    #[test]
    fn test_block_frequency_streaming() {
        let mut test = BlockFrequencyTest::new();
        // Less than 1 block (10 bytes) -> insufficient data
        test.update(&[0xAA; 10]);
        assert_eq!(test.evaluate().status, TestStatus::InsufficientData);

        // Feed remaining 6 bytes to complete 1st block of alternating bits (0xAA has 4 ones/byte -> 64 ones/block)
        test.update(&[0xAA; 6]);
        assert_eq!(test.num_blocks, 1);
        assert_eq!(test.sum_sq_diff, 0);
        let eval = test.evaluate();
        assert_eq!(eval.status, TestStatus::Passed);
        assert_eq!(eval.statistic, 0.0);

        // Feed 10 more blocks of 0xAA in various chunk sizes
        test.update(&[0xAA; 160]);
        assert_eq!(test.num_blocks, 11);
        assert_eq!(test.sum_sq_diff, 0);
        assert_eq!(test.evaluate().status, TestStatus::Passed);

        // Reset
        test.reset();
        assert_eq!(test.num_blocks, 0);
        assert_eq!(test.total_bits, 0);
    }
}
