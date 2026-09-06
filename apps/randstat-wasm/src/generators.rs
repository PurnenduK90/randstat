//! WASM C-ABI exports for byte stream generators and formatters.

use randstat_core::generators::{
    format_stream, AesCtrDrbg, ByteGenerator, ChaCha20Generator, DeBruijnGenerator, FixedGenerator,
    GaussianGenerator, GeneratorType, LcgGenerator, LcgPreset, LfsrGenerator, OutputFormat,
    PoissonGenerator, SequenceGenerator, Sha256Drbg, Xoshiro256StarStar,
};

/// 64 KB raw byte chunk buffer in WASM linear memory.
pub const CHUNK_CAPACITY: usize = 65536;
pub static mut GEN_CHUNK_BUFFER: [u8; CHUNK_CAPACITY] = [0u8; CHUNK_CAPACITY];

/// Formatted text output buffer (fits worst-case expansion of 10x for floats/bits).
pub const FORMATTED_CAPACITY: usize = CHUNK_CAPACITY * 10;
pub static mut GEN_FORMATTED_BUFFER: [u8; FORMATTED_CAPACITY] = [0u8; FORMATTED_CAPACITY];

enum ActiveGen {
    None,
    Sequence(SequenceGenerator),
    Fixed(FixedGenerator),
    Lfsr(LfsrGenerator),
    DeBruijn(DeBruijnGenerator),
    Lcg(LcgGenerator),
    Xoshiro(Xoshiro256StarStar),
    Gaussian(GaussianGenerator),
    Poisson(PoissonGenerator),
    Aes(AesCtrDrbg),
    Sha(Sha256Drbg),
    ChaCha(ChaCha20Generator),
}

static mut ACTIVE_GENERATOR: ActiveGen = ActiveGen::None;

/// Initializes the active generator in WASM memory.
///
/// # Arguments
/// - `gen_type`: GeneratorType (0=Seq, 1=Fixed, 2=LFSR, 3=DeBruijn, 4=LCG, 5=Xoshiro, 6=Gaussian, 7=Poisson, 8=AES, 9=SHA, 10=ChaCha)
/// - `p1`: Primary parameter (e.g. start byte, constant val, order, preset, mean*10, etc.)
/// - `p2`: Secondary parameter (e.g. step, std_dev*10, etc.)
/// - `seed_lo`: Lower 64 bits of seed
/// - `seed_hi`: Higher 64 bits of seed
#[no_mangle]
pub extern "C" fn generator_init(
    gen_type: u32,
    p1: u32,
    p2: u32,
    seed_lo: u64,
    seed_hi: u64,
) -> u32 {
    let seed128 = (seed_hi as u128) << 64 | (seed_lo as u128);
    let gen = match GeneratorType::from_u32(gen_type) {
        Some(GeneratorType::Sequence) => {
            ActiveGen::Sequence(SequenceGenerator::new(p1 as u8, p2 as u8))
        }
        Some(GeneratorType::Fixed) => ActiveGen::Fixed(FixedGenerator::new(p1 as u8)),
        Some(GeneratorType::Lfsr) => ActiveGen::Lfsr(LfsrGenerator::new(p1, seed128)),
        Some(GeneratorType::DeBruijn) => ActiveGen::DeBruijn(DeBruijnGenerator::new(p1)),
        Some(GeneratorType::Lcg) => {
            let preset = match p1 {
                0 => LcgPreset::AnsiC,
                1 => LcgPreset::Minstd,
                _ => LcgPreset::Knuth64,
            };
            ActiveGen::Lcg(LcgGenerator::new(preset, seed_lo))
        }
        Some(GeneratorType::Xoshiro) => ActiveGen::Xoshiro(Xoshiro256StarStar::from_seed(seed_lo)),
        Some(GeneratorType::Gaussian) => {
            let mean = if p1 == 0 { 127.5 } else { p1 as f64 / 10.0 };
            let std_dev = if p2 == 0 { 30.0 } else { p2 as f64 / 10.0 };
            ActiveGen::Gaussian(GaussianGenerator::new(mean, std_dev, seed_lo))
        }
        Some(GeneratorType::Poisson) => {
            let lambda = if p1 == 0 { 127.0 } else { p1 as f64 / 10.0 };
            ActiveGen::Poisson(PoissonGenerator::new(lambda, seed_lo))
        }
        Some(GeneratorType::AesCtrDrbg) => {
            let mut key = [0u8; 32];
            key[..8].copy_from_slice(&seed_lo.to_le_bytes());
            key[8..16].copy_from_slice(&seed_hi.to_le_bytes());
            let v = [0u8; 16];
            ActiveGen::Aes(AesCtrDrbg::new_256(&key, &v))
        }
        Some(GeneratorType::Sha256Drbg) => {
            let mut seed = [0u8; 32];
            seed[..8].copy_from_slice(&seed_lo.to_le_bytes());
            seed[8..16].copy_from_slice(&seed_hi.to_le_bytes());
            ActiveGen::Sha(Sha256Drbg::new(&seed))
        }
        Some(GeneratorType::ChaCha20) => {
            let mut key = [0u8; 32];
            key[..8].copy_from_slice(&seed_lo.to_le_bytes());
            key[8..16].copy_from_slice(&seed_hi.to_le_bytes());
            let nonce = [0u8; 12];
            ActiveGen::ChaCha(ChaCha20Generator::new(&key, &nonce, 1))
        }
        None => return 0,
    };

    unsafe {
        ACTIVE_GENERATOR = gen;
    }
    1
}

/// Fills destination buffer with raw bytes from active generator.
///
/// # Safety
/// Caller must ensure `dest_ptr` points to a valid, mutable buffer of at least `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn generator_fill(dest_ptr: *mut u8, len: usize) -> usize {
    if dest_ptr.is_null() || len == 0 {
        return 0;
    }
    let slice = core::slice::from_raw_parts_mut(dest_ptr, len);
    match &mut ACTIVE_GENERATOR {
        ActiveGen::Sequence(g) => g.fill_bytes(slice),
        ActiveGen::Fixed(g) => g.fill_bytes(slice),
        ActiveGen::Lfsr(g) => g.fill_bytes(slice),
        ActiveGen::DeBruijn(g) => g.fill_bytes(slice),
        ActiveGen::Lcg(g) => g.fill_bytes(slice),
        ActiveGen::Xoshiro(g) => g.fill_bytes(slice),
        ActiveGen::Gaussian(g) => g.fill_bytes(slice),
        ActiveGen::Poisson(g) => g.fill_bytes(slice),
        ActiveGen::Aes(g) => g.fill_bytes(slice),
        ActiveGen::Sha(g) => g.fill_bytes(slice),
        ActiveGen::ChaCha(g) => g.fill_bytes(slice),
        ActiveGen::None => return 0,
    }
    len
}

/// Fills internal WASM chunk buffer (`GEN_CHUNK_BUFFER`) with up to `CHUNK_CAPACITY` raw bytes.
///
/// # Safety
/// Modifies static memory without thread synchronisation (standard for single-threaded WASM).
#[no_mangle]
pub unsafe extern "C" fn generator_fill_chunk(len: usize) -> usize {
    let actual_len = len.min(CHUNK_CAPACITY);
    generator_fill(GEN_CHUNK_BUFFER.as_mut_ptr(), actual_len)
}

/// Formats `raw_len` bytes from `GEN_CHUNK_BUFFER` into `GEN_FORMATTED_BUFFER` with `format_code`.
///
/// # Safety
/// Reads and writes static memory buffers without thread synchronisation.
#[no_mangle]
pub unsafe extern "C" fn generator_format_chunk(raw_len: usize, format_code: u32) -> usize {
    let actual_raw = raw_len.min(CHUNK_CAPACITY);
    let format = OutputFormat::from_u32(format_code).unwrap_or(OutputFormat::Binary);
    format_stream(
        &GEN_CHUNK_BUFFER[..actual_raw],
        format,
        &mut GEN_FORMATTED_BUFFER,
    )
}

/// Resets the active generator state.
///
/// # Safety
/// Modifies static generator state without thread synchronisation.
#[no_mangle]
pub unsafe extern "C" fn generator_reset() {
    match &mut ACTIVE_GENERATOR {
        ActiveGen::Sequence(g) => g.reset(),
        ActiveGen::Fixed(g) => g.reset(),
        ActiveGen::Lfsr(g) => g.reset(),
        ActiveGen::DeBruijn(g) => g.reset(),
        ActiveGen::Lcg(g) => g.reset(),
        ActiveGen::Xoshiro(g) => g.reset(),
        ActiveGen::Gaussian(g) => g.reset(),
        ActiveGen::Poisson(g) => g.reset(),
        ActiveGen::Aes(g) => g.reset(),
        ActiveGen::Sha(g) => g.reset(),
        ActiveGen::ChaCha(g) => g.reset(),
        ActiveGen::None => {}
    }
}

/// Returns pointer to raw chunk buffer.
#[no_mangle]
pub extern "C" fn generator_get_chunk_ptr() -> *const u8 {
    unsafe { GEN_CHUNK_BUFFER.as_ptr() }
}

/// Returns capacity of raw chunk buffer (65536 bytes).
#[no_mangle]
pub extern "C" fn generator_get_chunk_capacity() -> usize {
    CHUNK_CAPACITY
}

/// Returns pointer to formatted buffer.
#[no_mangle]
pub extern "C" fn generator_get_formatted_ptr() -> *const u8 {
    unsafe { GEN_FORMATTED_BUFFER.as_ptr() }
}

/// Returns capacity of formatted buffer.
#[no_mangle]
pub extern "C" fn generator_get_formatted_capacity() -> usize {
    FORMATTED_CAPACITY
}
