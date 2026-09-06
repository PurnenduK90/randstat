//! NIST SP 800-90A Rev. 1 CTR_DRBG (AES-128 / AES-256 Counter Mode DRBG).
//!
//! Provides deterministic cryptographically secure random bit generation using AES in Counter Mode.

use super::aes::AesKey;
use super::traits::ByteGenerator;

/// NIST SP 800-90A CTR_DRBG state.
#[derive(Debug, Clone)]
pub struct AesCtrDrbg {
    key: AesKey,
    v: [u8; 16],
    initial_v: [u8; 16],
    block_buf: [u8; 16],
    buf_idx: usize,
}

impl AesCtrDrbg {
    /// Creates a new CTR_DRBG instance from a 128-bit key and 128-bit initial counter $V$.
    pub fn new_128(key: &[u8; 16], v: &[u8; 16]) -> Self {
        let aes_key = AesKey::expand_128(key);
        Self {
            key: aes_key,
            v: *v,
            initial_v: *v,
            block_buf: [0u8; 16],
            buf_idx: 16,
        }
    }

    /// Creates a new CTR_DRBG instance from a 256-bit key and 128-bit initial counter $V$.
    pub fn new_256(key: &[u8; 32], v: &[u8; 16]) -> Self {
        let aes_key = AesKey::expand_256(key);
        Self {
            key: aes_key,
            v: *v,
            initial_v: *v,
            block_buf: [0u8; 16],
            buf_idx: 16,
        }
    }

    /// Increments the 128-bit counter $V$ modulo $2^{128}$.
    #[inline]
    fn increment_v(&mut self) {
        for byte in self.v.iter_mut().rev() {
            *byte = byte.wrapping_add(1);
            if *byte != 0 {
                break;
            }
        }
    }

    /// Generates the next 16-byte block.
    #[inline]
    fn next_block(&mut self) {
        self.increment_v();
        let mut block = self.v;
        self.key.encrypt_block(&mut block);
        self.block_buf = block;
        self.buf_idx = 0;
    }
}

impl Default for AesCtrDrbg {
    fn default() -> Self {
        let default_key: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let default_v: [u8; 16] = [0x00; 16];
        Self::new_256(&default_key, &default_v)
    }
}

impl ByteGenerator for AesCtrDrbg {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        if self.buf_idx >= 16 {
            self.next_block();
        }
        let b = self.block_buf[self.buf_idx];
        self.buf_idx += 1;
        b
    }

    fn reset(&mut self) {
        self.v = self.initial_v;
        self.buf_idx = 16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_ctr_drbg() {
        let mut drbg = AesCtrDrbg::default();
        let mut buf = [0u8; 64];
        drbg.fill_bytes(&mut buf);
        assert_ne!(buf, [0u8; 64]);

        drbg.reset();
        let mut buf2 = [0u8; 64];
        drbg.fill_bytes(&mut buf2);
        assert_eq!(buf, buf2);
    }
}
