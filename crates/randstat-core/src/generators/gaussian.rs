//! Gaussian (Normal) distribution byte stream generator.
//!
//! Uses the Box-Muller transform to map uniform random numbers into Gaussian values
//! scaled into the byte range `[0, 255]`.

use super::traits::ByteGenerator;
use super::xoshiro::Xoshiro256StarStar;
use crate::math::transform::box_muller;

/// Gaussian distributed byte generator.
#[derive(Debug, Clone)]
pub struct GaussianGenerator {
    mean: f64,
    std_dev: f64,
    rng: Xoshiro256StarStar,
    has_spare: bool,
    spare: f64,
}

impl GaussianGenerator {
    /// Creates a new Gaussian generator with specified mean, standard deviation, and RNG seed.
    pub fn new(mean: f64, std_dev: f64, seed: u64) -> Self {
        Self {
            mean,
            std_dev,
            rng: Xoshiro256StarStar::from_seed(seed),
            has_spare: false,
            spare: 0.0,
        }
    }

    /// Generates the next floating point sample from $\mathcal{N}(\mu, \sigma^2)$.
    pub fn next_f64(&mut self) -> f64 {
        if self.has_spare {
            self.has_spare = false;
            return self.spare * self.std_dev + self.mean;
        }

        let u1 = self.rng.next_f64();
        let u2 = self.rng.next_f64();

        let (z0, z1) = box_muller(u1, u2);

        self.spare = z1;
        self.has_spare = true;

        z0 * self.std_dev + self.mean
    }
}

impl Default for GaussianGenerator {
    fn default() -> Self {
        Self::new(127.5, 30.0, 0x4D3C2B1A)
    }
}

impl ByteGenerator for GaussianGenerator {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        let val = self.next_f64();
        if val <= 0.0 {
            0
        } else if val >= 255.0 {
            255
        } else {
            val as u8
        }
    }

    fn reset(&mut self) {
        self.rng.reset();
        self.has_spare = false;
        self.spare = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_generator() {
        let mut gen = GaussianGenerator::default();
        let mut buf = [0u8; 100];
        gen.fill_bytes(&mut buf);
        assert_ne!(buf, [0u8; 100]);

        gen.reset();
        let mut buf2 = [0u8; 100];
        gen.fill_bytes(&mut buf2);
        assert_eq!(buf, buf2);
    }
}
