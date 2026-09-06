//! Poisson distribution byte stream generator.
//!
//! Generates discrete Poisson distributed values (mean $\lambda \approx 127.0$) using
//! Gaussian approximation with continuity correction for efficient streaming.

use super::gaussian::GaussianGenerator;
use super::traits::ByteGenerator;
use libm::sqrt;

/// Poisson distributed byte generator.
#[derive(Debug, Clone)]
pub struct PoissonGenerator {
    gaussian: GaussianGenerator,
}

impl PoissonGenerator {
    /// Creates a new Poisson generator with specified parameter $\lambda$ and seed.
    pub fn new(lambda: f64, seed: u64) -> Self {
        let std_dev = sqrt(if lambda <= 0.0 { 1.0 } else { lambda });
        Self {
            gaussian: GaussianGenerator::new(lambda, std_dev, seed),
        }
    }
}

impl Default for PoissonGenerator {
    fn default() -> Self {
        Self::new(127.0, 0x1A2B3C4D)
    }
}

impl ByteGenerator for PoissonGenerator {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        self.gaussian.next_byte()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.gaussian.fill_bytes(dest);
    }

    fn reset(&mut self) {
        self.gaussian.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poisson_generator() {
        let mut gen = PoissonGenerator::default();
        let mut buf = [0u8; 100];
        gen.fill_bytes(&mut buf);
        assert_ne!(buf, [0u8; 100]);

        gen.reset();
        let mut buf2 = [0u8; 100];
        gen.fill_bytes(&mut buf2);
        assert_eq!(buf, buf2);
    }
}
