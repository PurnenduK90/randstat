//! Compact `#![no_std]` AES-128 / AES-256 block cipher implementation.
//!
//! Provides zero-allocation encryption of 16-byte blocks according to FIPS 197.

/// AES S-Box substitution lookup table.
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5e, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// Round constants (Rcon).
const RCON: [u32; 10] = [
    0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x1B000000, 0x36000000,
];

#[inline]
fn sub_word(w: u32) -> u32 {
    ((SBOX[(w >> 24) as usize] as u32) << 24)
        | ((SBOX[((w >> 16) & 0xFF) as usize] as u32) << 16)
        | ((SBOX[((w >> 8) & 0xFF) as usize] as u32) << 8)
        | (SBOX[(w & 0xFF) as usize] as u32)
}

#[inline]
fn rot_word(w: u32) -> u32 {
    w.rotate_left(8)
}

#[inline]
fn gmul_2(b: u8) -> u8 {
    (b << 1) ^ (if (b & 0x80) != 0 { 0x1B } else { 0 })
}

#[inline]
fn gmul_3(b: u8) -> u8 {
    gmul_2(b) ^ b
}

/// AES Expanded Key state supporting AES-128 (10 rounds) and AES-256 (14 rounds).
#[derive(Debug, Clone, Copy)]
pub struct AesKey {
    round_keys: [u32; 60],
    num_rounds: usize,
}

impl AesKey {
    /// Expands a 128-bit key (16 bytes).
    pub fn expand_128(key: &[u8; 16]) -> Self {
        let mut rk = [0u32; 60];
        for i in 0..4 {
            rk[i] =
                u32::from_be_bytes([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
        }

        for i in 4..44 {
            let mut temp = rk[i - 1];
            if i % 4 == 0 {
                temp = sub_word(rot_word(temp)) ^ RCON[i / 4 - 1];
            }
            rk[i] = rk[i - 4] ^ temp;
        }

        Self {
            round_keys: rk,
            num_rounds: 10,
        }
    }

    /// Expands a 256-bit key (32 bytes).
    pub fn expand_256(key: &[u8; 32]) -> Self {
        let mut rk = [0u32; 60];
        for i in 0..8 {
            rk[i] =
                u32::from_be_bytes([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
        }

        for i in 8..60 {
            let mut temp = rk[i - 1];
            if i % 8 == 0 {
                temp = sub_word(rot_word(temp)) ^ RCON[i / 8 - 1];
            } else if i % 8 == 4 {
                temp = sub_word(temp);
            }
            rk[i] = rk[i - 8] ^ temp;
        }

        Self {
            round_keys: rk,
            num_rounds: 14,
        }
    }

    /// Encrypts a 16-byte block in place.
    pub fn encrypt_block(&self, block: &mut [u8; 16]) {
        // Initial AddRoundKey
        for (i, col) in block.chunks_exact_mut(4).enumerate() {
            let w = self.round_keys[i].to_be_bytes();
            for j in 0..4 {
                col[j] ^= w[j];
            }
        }

        let nr = self.num_rounds;
        for round in 1..nr {
            // 1. SubBytes
            for b in block.iter_mut() {
                *b = SBOX[*b as usize];
            }

            // 2. ShiftRows
            let s = *block;
            block[0] = s[0];
            block[4] = s[4];
            block[8] = s[8];
            block[12] = s[12];
            block[1] = s[5];
            block[5] = s[9];
            block[9] = s[13];
            block[13] = s[1];
            block[2] = s[10];
            block[6] = s[14];
            block[10] = s[2];
            block[14] = s[6];
            block[3] = s[15];
            block[7] = s[3];
            block[11] = s[7];
            block[15] = s[11];

            // 3. MixColumns
            for col in block.chunks_exact_mut(4) {
                let (c0, c1, c2, c3) = (col[0], col[1], col[2], col[3]);
                col[0] = gmul_2(c0) ^ gmul_3(c1) ^ c2 ^ c3;
                col[1] = c0 ^ gmul_2(c1) ^ gmul_3(c2) ^ c3;
                col[2] = c0 ^ c1 ^ gmul_2(c2) ^ gmul_3(c3);
                col[3] = gmul_3(c0) ^ c1 ^ c2 ^ gmul_2(c3);
            }

            // 4. AddRoundKey
            let rk_offset = round * 4;
            for (i, col) in block.chunks_exact_mut(4).enumerate() {
                let w = self.round_keys[rk_offset + i].to_be_bytes();
                for j in 0..4 {
                    col[j] ^= w[j];
                }
            }
        }

        // Final round (no MixColumns)
        for b in block.iter_mut() {
            *b = SBOX[*b as usize];
        }
        let s = *block;
        block[0] = s[0];
        block[4] = s[4];
        block[8] = s[8];
        block[12] = s[12];
        block[1] = s[5];
        block[5] = s[9];
        block[9] = s[13];
        block[13] = s[1];
        block[2] = s[10];
        block[6] = s[14];
        block[10] = s[2];
        block[14] = s[6];
        block[3] = s[15];
        block[7] = s[3];
        block[11] = s[7];
        block[15] = s[11];

        let rk_offset = nr * 4;
        for (i, col) in block.chunks_exact_mut(4).enumerate() {
            let w = self.round_keys[rk_offset + i].to_be_bytes();
            for j in 0..4 {
                col[j] ^= w[j];
            }
        }
    }
}

/// Standalone one-shot AES-128 16-byte block encryption.
#[inline]
pub fn aes128_encrypt_block(key: &[u8; 16], block: &mut [u8; 16]) {
    let aes = AesKey::expand_128(key);
    aes.encrypt_block(block);
}

/// Standalone one-shot AES-256 16-byte block encryption.
#[inline]
pub fn aes256_encrypt_block(key: &[u8; 32], block: &mut [u8; 16]) {
    let aes = AesKey::expand_256(key);
    aes.encrypt_block(block);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes128_nist_vector() {
        // FIPS 197 Appendix B Test Vector
        let key: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let mut plaintext: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        let expected_ciphertext: [u8; 16] = [
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32,
        ];

        let aes_key = AesKey::expand_128(&key);
        aes_key.encrypt_block(&mut plaintext);
        assert_eq!(plaintext, expected_ciphertext);
    }
}
