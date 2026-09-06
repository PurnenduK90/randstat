//! Sequential ramp byte generator.

use super::traits::ByteGenerator;

/// Generates a sequential ramp of bytes: `(start + i * step) % 256`.
#[derive(Debug, Clone)]
pub struct SequenceGenerator {
    start: u8,
    step: u8,
    current: u8,
}

impl SequenceGenerator {
    /// Creates a new sequential generator with specified start byte and step size.
    pub const fn new(start: u8, step: u8) -> Self {
        Self {
            start,
            step,
            current: start,
        }
    }
}

impl Default for SequenceGenerator {
    fn default() -> Self {
        Self::new(0, 1)
    }
}

impl ByteGenerator for SequenceGenerator {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        let val = self.current;
        self.current = self.current.wrapping_add(self.step);
        val
    }

    fn reset(&mut self) {
        self.current = self.start;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_generator() {
        let mut gen = SequenceGenerator::new(0, 1);
        let mut buf = [0u8; 5];
        gen.fill_bytes(&mut buf);
        assert_eq!(buf, [0, 1, 2, 3, 4]);

        gen.reset();
        assert_eq!(gen.next_byte(), 0);

        let mut gen_step = SequenceGenerator::new(10, 5);
        assert_eq!(gen_step.next_byte(), 10);
        assert_eq!(gen_step.next_byte(), 15);
    }
}
