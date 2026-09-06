//! ChaCha20 stream cipher generator (RFC 8439).
//!
//! Generates 64-byte keystream blocks using 20 rounds of the ChaCha quarter-round function.

use super::traits::ByteGenerator;

/// ChaCha20 stream generator state.
#[derive(Debug, Clone)]
pub struct ChaCha20Generator {
    key: [u32; 8],
    nonce: [u32; 3],
    counter: u32,
    initial_counter: u32,
    block_buf: [u8; 64],
    buf_idx: usize,
}

impl ChaCha20Generator {
    /// Creates a new ChaCha20 generator from a 256-bit (32-byte) key and 96-bit (12-byte) nonce.
    pub fn new(key_bytes: &[u8; 32], nonce_bytes: &[u8; 12], initial_counter: u32) -> Self {
        let mut key = [0u32; 8];
        for (i, chunk) in key_bytes.chunks_exact(4).enumerate() {
            key[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        let mut nonce = [0u32; 3];
        for (i, chunk) in nonce_bytes.chunks_exact(4).enumerate() {
            nonce[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        Self {
            key,
            nonce,
            counter: initial_counter,
            initial_counter,
            block_buf: [0u8; 64],
            buf_idx: 64,
        }
    }

    #[inline]
    fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        state[a] = state[a].wrapping_add(state[b]);
        state[d] = (state[d] ^ state[a]).rotate_left(16);

        state[c] = state[c].wrapping_add(state[d]);
        state[b] = (state[b] ^ state[c]).rotate_left(12);

        state[a] = state[a].wrapping_add(state[b]);
        state[d] = (state[d] ^ state[a]).rotate_left(8);

        state[c] = state[c].wrapping_add(state[d]);
        state[b] = (state[b] ^ state[c]).rotate_left(7);
    }

    /// Generates next 64-byte ChaCha20 keystream block.
    #[inline]
    fn next_block(&mut self) {
        let mut state = [
            0x61707865,
            0x3320646e,
            0x79622d32,
            0x6b206574,
            self.key[0],
            self.key[1],
            self.key[2],
            self.key[3],
            self.key[4],
            self.key[5],
            self.key[6],
            self.key[7],
            self.counter,
            self.nonce[0],
            self.nonce[1],
            self.nonce[2],
        ];

        let initial_state = state;

        for _ in 0..10 {
            // Column rounds
            Self::quarter_round(&mut state, 0, 4, 8, 12);
            Self::quarter_round(&mut state, 1, 5, 9, 13);
            Self::quarter_round(&mut state, 2, 6, 10, 14);
            Self::quarter_round(&mut state, 3, 7, 11, 15);
            // Diagonal rounds
            Self::quarter_round(&mut state, 0, 5, 10, 15);
            Self::quarter_round(&mut state, 1, 6, 11, 12);
            Self::quarter_round(&mut state, 2, 7, 8, 13);
            Self::quarter_round(&mut state, 3, 4, 9, 14);
        }

        for i in 0..16 {
            state[i] = state[i].wrapping_add(initial_state[i]);
            let bytes = state[i].to_le_bytes();
            self.block_buf[i * 4..i * 4 + 4].copy_from_slice(&bytes);
        }

        self.counter = self.counter.wrapping_add(1);
        self.buf_idx = 0;
    }
}

/// Computes a standalone 64-byte ChaCha20 block from key, nonce, and counter.
pub fn chacha20_block(key_bytes: &[u8; 32], nonce_bytes: &[u8; 12], counter: u32) -> [u8; 64] {
    let mut gen = ChaCha20Generator::new(key_bytes, nonce_bytes, counter);
    gen.next_block();
    gen.block_buf
}

impl Default for ChaCha20Generator {
    fn default() -> Self {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        Self::new(&key, &nonce, 1)
    }
}

impl ByteGenerator for ChaCha20Generator {
    #[inline]
    fn next_byte(&mut self) -> u8 {
        if self.buf_idx >= 64 {
            self.next_block();
        }
        let b = self.block_buf[self.buf_idx];
        self.buf_idx += 1;
        b
    }

    fn reset(&mut self) {
        self.counter = self.initial_counter;
        self.buf_idx = 64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha20_generator() {
        let mut chacha = ChaCha20Generator::default();
        let mut buf = [0u8; 128];
        chacha.fill_bytes(&mut buf);
        assert_ne!(buf, [0u8; 128]);

        chacha.reset();
        let mut buf2 = [0u8; 128];
        chacha.fill_bytes(&mut buf2);
        assert_eq!(buf, buf2);
    }
}
