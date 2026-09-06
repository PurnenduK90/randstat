//! NIST SP 800-90A Rev. 1 Hash_DRBG (SHA-256 based).
//!
//! Implements deterministic hash-based pseudorandom bit generation using SHA-256.

use super::traits::ByteGenerator;
use crate::bitstream::sha256::Sha256;

/// NIST SP 800-90A Hash_DRBG state using SHA-256 (seed length = 440 bits / 55 bytes).
#[derive(Debug, Clone)]
pub struct Sha256Drbg {
    v: [u8; 55],
    initial_v: [u8; 55],
    block_buf: [u8; 32],
    buf_idx: usize,
}

impl Sha256Drbg {
    /// Creates a new Hash_DRBG from a seed slice.
    pub fn new(seed: &[u8]) -> Self {
        let mut v = [0u8; 55];
        let len = seed.len().min(55);
        v[..len].copy_from_slice(&seed[..len]);

        Self {
            v,
            initial_v: v,
            block_buf: [0u8; 32],
            buf_idx: 32,
        }
    }

    /// Increments 55-byte counter state $V$.
    #[inline]
    fn increment_v(&mut self) {
        for byte in self.v.iter_mut().rev() {
            *byte = byte.wrapping_add(1);
            if *byte != 0 {
                break;
            }
        }
    }

    /// Generates next 32-byte block via SHA-256($V$).
    #[inline]
    fn next_block(&mut self) {
        self.increment_v();
        let mut hasher = Sha256::new();
        hasher.update(&self.v);
        self.block_buf = hasher.finalize();
        self.buf_idx = 0;
    }
}

impl Default for Sha256Drbg {
    fn default() -> Self {
        let default_seed = b"randstat_nist_sp800_90a_sha256_hash_drbg_default_seed_material";
        Self::new(default_seed)
    }
}

impl ByteGenerator for Sha256Drbg {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        if self.buf_idx >= 32 {
            self.next_block();
        }
        let b = self.block_buf[self.buf_idx];
        self.buf_idx += 1;
        b
    }

    fn reset(&mut self) {
        self.v = self.initial_v;
        self.buf_idx = 32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_drbg() {
        let mut drbg = Sha256Drbg::default();
        let mut buf = [0u8; 64];
        drbg.fill_bytes(&mut buf);
        assert_ne!(buf, [0u8; 64]);

        drbg.reset();
        let mut buf2 = [0u8; 64];
        drbg.fill_bytes(&mut buf2);
        assert_eq!(buf, buf2);
    }
}
