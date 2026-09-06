//! Constant byte generator (all zeros, all ones, or arbitrary constant byte).

use super::traits::ByteGenerator;

/// Generates a constant stream of a single repeated byte.
#[derive(Debug, Clone)]
pub struct FixedGenerator {
    value: u8,
}

impl FixedGenerator {
    /// Creates a new generator yielding constant byte `value`.
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    /// Creates a generator yielding constant zeros (`0x00`).
    pub const fn zeros() -> Self {
        Self::new(0x00)
    }

    /// Creates a generator yielding constant ones (`0xFF`).
    pub const fn ones() -> Self {
        Self::new(0xFF)
    }
}

impl Default for FixedGenerator {
    fn default() -> Self {
        Self::zeros()
    }
}

impl ByteGenerator for FixedGenerator {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        self.value
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.fill(self.value);
    }

    fn reset(&mut self) {
        // Stateless
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_generator() {
        let mut gen_zeros = FixedGenerator::zeros();
        let mut buf = [1u8; 10];
        gen_zeros.fill_bytes(&mut buf);
        assert_eq!(buf, [0u8; 10]);

        let mut gen_custom = FixedGenerator::new(0x42);
        assert_eq!(gen_custom.next_byte(), 0x42);
        gen_custom.reset();
        assert_eq!(gen_custom.next_byte(), 0x42);
    }
}
