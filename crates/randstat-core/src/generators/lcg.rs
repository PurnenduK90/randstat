//! Linear Congruential Generator (LCG).
//!
//! Implements classic LCG algorithms ($X_{n+1} = (a X_n + c) \bmod m$)
//! including ANSI C `rand()`, MINSTD, and 64-bit Knuth/MMIX.

use super::traits::ByteGenerator;

/// Preset configurations for LCG.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcgPreset {
    /// ANSI C `rand()`: $a = 1103515245, c = 12345, m = 2^{31}$
    AnsiC,
    /// MINSTD (Park & Miller): $a = 48271, c = 0, m = 2^{31}-1$
    Minstd,
    /// 64-bit Knuth / MMIX: $a = 6364136223846793005, c = 1442695040888963407, m = 2^{64}$
    Knuth64,
}

/// LCG byte generator state.
#[derive(Debug, Clone)]
pub struct LcgGenerator {
    preset: LcgPreset,
    state: u64,
    initial_seed: u64,
}

impl LcgGenerator {
    /// Creates a new LCG generator with specified preset and seed.
    pub fn new(preset: LcgPreset, seed: u64) -> Self {
        let initial_seed = if seed == 0 && preset == LcgPreset::Minstd {
            1
        } else {
            seed
        };
        Self {
            preset,
            state: initial_seed,
            initial_seed,
        }
    }

    /// Creates an ANSI C `rand()` compatible generator.
    pub fn ansi_c(seed: u32) -> Self {
        Self::new(LcgPreset::AnsiC, seed as u64)
    }

    /// Creates a MINSTD generator.
    pub fn minstd(seed: u32) -> Self {
        Self::new(LcgPreset::Minstd, seed as u64)
    }

    /// Steps the LCG and returns a 32-bit integer.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        match self.preset {
            LcgPreset::AnsiC => {
                self.state = (self.state.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7FFFFFFF;
                ((self.state >> 16) & 0x7FFF) as u32
            }
            LcgPreset::Minstd => {
                let temp = (self.state * 48271) % 0x7FFFFFFF;
                self.state = if temp == 0 { 1 } else { temp };
                self.state as u32
            }
            LcgPreset::Knuth64 => {
                self.state = self
                    .state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                (self.state >> 32) as u32
            }
        }
    }
}

impl Default for LcgGenerator {
    fn default() -> Self {
        Self::new(LcgPreset::AnsiC, 12345)
    }
}

impl ByteGenerator for LcgGenerator {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        (self.next_u32() & 0xFF) as u8
    }

    fn reset(&mut self) {
        self.state = self.initial_seed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcg_generators() {
        for preset in [LcgPreset::AnsiC, LcgPreset::Minstd, LcgPreset::Knuth64] {
            let mut lcg = LcgGenerator::new(preset, 42);
            let mut buf = [0u8; 16];
            lcg.fill_bytes(&mut buf);
            assert_ne!(buf, [0u8; 16]);

            lcg.reset();
            assert_eq!(lcg.next_byte(), buf[0]);
        }
    }
}
