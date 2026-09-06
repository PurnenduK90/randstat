//! Native `#![no_std]` test stream generators and formatters.
//!
//! Provides zero-allocation implementations of:
//! - **Sequence**: Linear ramp `(start + i * step) % 256`
//! - **Fixed**: Constant stream `0x00`, `0xFF`, or byte $K$
//! - **LFSR**: Configurable orders (3 to 128 bits) with primitive feedback polynomials
//! - **De Bruijn**: Complete $2^n$ binary cycles ($O(1)$ memory)
//! - **LCG**: ANSI C, MINSTD, 64-bit Knuth
//! - **Xoshiro256\*\***: Ultra-fast simulation PRNG
//! - **Gaussian**: Box-Muller normal distribution
//! - **Poisson**: Poisson distribution ($\lambda = 127$)
//! - **AES CTR_DRBG**: NIST SP 800-90A Rev 1 counter mode DRBG
//! - **SHA-256 Hash_DRBG**: NIST SP 800-90A Rev 1 hash DRBG
//! - **ChaCha20**: RFC 8439 256-bit stream generator

pub mod aes;
pub mod aes_drbg;
pub mod chacha;
pub mod debruijn;
pub mod fixed;
pub mod formatter;
pub mod gaussian;
pub mod lcg;
pub mod lfsr;
pub mod poisson;
pub mod sequence;
pub mod sha_drbg;
pub mod traits;
pub mod xoshiro;

pub use aes::{aes128_encrypt_block, aes256_encrypt_block, AesKey};
pub use aes_drbg::AesCtrDrbg;
pub use chacha::{chacha20_block, ChaCha20Generator};
pub use debruijn::DeBruijnGenerator;
pub use fixed::FixedGenerator;
pub use formatter::format_stream;
pub use gaussian::GaussianGenerator;
pub use lcg::{LcgGenerator, LcgPreset};
pub use lfsr::LfsrGenerator;
pub use poisson::PoissonGenerator;
pub use sequence::SequenceGenerator;
pub use sha_drbg::Sha256Drbg;
pub use traits::{ByteGenerator, GeneratorType, OutputFormat};
pub use xoshiro::Xoshiro256StarStar;
