//! CLI subcommand and streaming engine for test stream generation.

use randstat_core::generators::{
    format_stream, AesCtrDrbg, ByteGenerator, ChaCha20Generator, DeBruijnGenerator, FixedGenerator,
    GaussianGenerator, GeneratorType, LcgGenerator, LcgPreset, LfsrGenerator, OutputFormat,
    PoissonGenerator, SequenceGenerator, Sha256Drbg, Xoshiro256StarStar,
};
use std::fs::File;
use std::io::{self, BufWriter, Write};

pub struct GenerateArgs {
    pub gen_type: GeneratorType,
    pub size_bytes: u64,
    pub output_file: Option<String>,
    pub format: OutputFormat,
    pub order: u32,
    pub start: u8,
    pub step: u8,
    pub value: u8,
    pub seed: u64,
}

pub fn parse_size_str(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();
    if s.is_empty() {
        return None;
    }
    if s.ends_with("KB") || s.ends_with('K') {
        let num_str = s.trim_end_matches("KB").trim_end_matches('K');
        num_str.parse::<f64>().ok().map(|n| (n * 1024.0) as u64)
    } else if s.ends_with("MB") || s.ends_with('M') {
        let num_str = s.trim_end_matches("MB").trim_end_matches('M');
        num_str
            .parse::<f64>()
            .ok()
            .map(|n| (n * 1024.0 * 1024.0) as u64)
    } else if s.ends_with("GB") || s.ends_with('G') {
        let num_str = s.trim_end_matches("GB").trim_end_matches('G');
        num_str
            .parse::<f64>()
            .ok()
            .map(|n| (n * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if s.ends_with("TB") || s.ends_with('T') {
        let num_str = s.trim_end_matches("TB").trim_end_matches('T');
        num_str
            .parse::<f64>()
            .ok()
            .map(|n| (n * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64)
    } else {
        s.parse::<u64>().ok()
    }
}

pub fn parse_generate_args(args: &[String]) -> Result<GenerateArgs, String> {
    let mut gen_type = GeneratorType::Sequence;
    let mut size_bytes = 1048576u64; // 1 MB default
    let mut output_file = None;
    let mut format = OutputFormat::Binary;
    let mut order = 19u32;
    let mut start = 0u8;
    let mut step = 1u8;
    let mut value = 0u8;
    let mut seed = 42u64;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-t" | "--type" => {
                if i + 1 < args.len() {
                    gen_type = match args[i + 1].to_lowercase().as_str() {
                        "seq" | "sequence" | "ramp" => GeneratorType::Sequence,
                        "fixed" | "const" | "constant" | "zeros" | "ones" => GeneratorType::Fixed,
                        "lfsr" => GeneratorType::Lfsr,
                        "debruijn" | "db" => GeneratorType::DeBruijn,
                        "lcg" => GeneratorType::Lcg,
                        "xoshiro" | "xoroshiro" | "prng" => GeneratorType::Xoshiro,
                        "gaussian" | "normal" => GeneratorType::Gaussian,
                        "poisson" => GeneratorType::Poisson,
                        "aes" | "aes_drbg" | "ctr_drbg" => GeneratorType::AesCtrDrbg,
                        "sha" | "sha256" | "hash_drbg" => GeneratorType::Sha256Drbg,
                        "chacha" | "chacha20" => GeneratorType::ChaCha20,
                        other => return Err(format!("Unknown generator type '{}'", other)),
                    };
                    i += 1;
                }
            }
            "-s" | "--size" => {
                if i + 1 < args.len() {
                    size_bytes = parse_size_str(&args[i + 1])
                        .ok_or_else(|| format!("Invalid size string '{}'", args[i + 1]))?;
                    i += 1;
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-f" | "--format" => {
                if i + 1 < args.len() {
                    format = match args[i + 1].to_lowercase().as_str() {
                        "bin" | "binary" | "raw" => OutputFormat::Binary,
                        "int" | "ascii" | "txt" | "decimal" => OutputFormat::AsciiInt,
                        "bits" | "bitstream" => OutputFormat::AsciiBits,
                        "hex" => OutputFormat::Hex,
                        "float" | "csv" => OutputFormat::Float,
                        other => return Err(format!("Unknown output format '{}'", other)),
                    };
                    i += 1;
                }
            }
            "--order" => {
                if i + 1 < args.len() {
                    order = args[i + 1]
                        .parse::<u32>()
                        .map_err(|_| format!("Invalid order '{}'", args[i + 1]))?;
                    i += 1;
                }
            }
            "--start" => {
                if i + 1 < args.len() {
                    start = args[i + 1]
                        .parse::<u8>()
                        .map_err(|_| format!("Invalid start byte '{}'", args[i + 1]))?;
                    i += 1;
                }
            }
            "--step" => {
                if i + 1 < args.len() {
                    step = args[i + 1]
                        .parse::<u8>()
                        .map_err(|_| format!("Invalid step '{}'", args[i + 1]))?;
                    i += 1;
                }
            }
            "--val" | "--value" => {
                if i + 1 < args.len() {
                    value = args[i + 1]
                        .parse::<u8>()
                        .map_err(|_| format!("Invalid value byte '{}'", args[i + 1]))?;
                    i += 1;
                }
            }
            "--seed" => {
                if i + 1 < args.len() {
                    seed = args[i + 1]
                        .parse::<u64>()
                        .map_err(|_| format!("Invalid seed '{}'", args[i + 1]))?;
                    i += 1;
                }
            }
            arg if !arg.starts_with('-') && output_file.is_none() => {
                output_file = Some(arg.to_string());
            }
            _ => {}
        }
        i += 1;
    }

    Ok(GenerateArgs {
        gen_type,
        size_bytes,
        output_file,
        format,
        order,
        start,
        step,
        value,
        seed,
    })
}

pub fn run_generate_cli(args: &[String]) -> Result<(), String> {
    let parsed = parse_generate_args(args)?;

    let mut generator: Box<dyn ByteGenerator> = match parsed.gen_type {
        GeneratorType::Sequence => Box::new(SequenceGenerator::new(parsed.start, parsed.step)),
        GeneratorType::Fixed => Box::new(FixedGenerator::new(parsed.value)),
        GeneratorType::Lfsr => Box::new(LfsrGenerator::new(parsed.order, parsed.seed as u128)),
        GeneratorType::DeBruijn => Box::new(DeBruijnGenerator::new(parsed.order)),
        GeneratorType::Lcg => Box::new(LcgGenerator::new(LcgPreset::AnsiC, parsed.seed)),
        GeneratorType::Xoshiro => Box::new(Xoshiro256StarStar::from_seed(parsed.seed)),
        GeneratorType::Gaussian => Box::new(GaussianGenerator::new(127.5, 30.0, parsed.seed)),
        GeneratorType::Poisson => Box::new(PoissonGenerator::new(127.0, parsed.seed)),
        GeneratorType::AesCtrDrbg => {
            let mut key = [0u8; 32];
            key[..8].copy_from_slice(&parsed.seed.to_le_bytes());
            let v = [0u8; 16];
            Box::new(AesCtrDrbg::new_256(&key, &v))
        }
        GeneratorType::Sha256Drbg => {
            let mut seed_bytes = [0u8; 32];
            seed_bytes[..8].copy_from_slice(&parsed.seed.to_le_bytes());
            Box::new(Sha256Drbg::new(&seed_bytes))
        }
        GeneratorType::ChaCha20 => {
            let mut key = [0u8; 32];
            key[..8].copy_from_slice(&parsed.seed.to_le_bytes());
            let nonce = [0u8; 12];
            Box::new(ChaCha20Generator::new(&key, &nonce, 1))
        }
    };

    let mut writer: Box<dyn Write> = match &parsed.output_file {
        Some(path) => {
            let file =
                File::create(path).map_err(|e| format!("Cannot create file '{}': {}", path, e))?;
            Box::new(BufWriter::with_capacity(131072, file))
        }
        None => Box::new(BufWriter::with_capacity(131072, io::stdout())),
    };

    const CHUNK_SIZE: usize = 65536;
    let mut raw_chunk = [0u8; CHUNK_SIZE];
    let mut fmt_chunk = [0u8; CHUNK_SIZE * 10];
    let mut remaining = parsed.size_bytes;

    while remaining > 0 {
        let take = remaining.min(CHUNK_SIZE as u64) as usize;
        generator.fill_bytes(&mut raw_chunk[..take]);

        if parsed.format == OutputFormat::Binary {
            writer
                .write_all(&raw_chunk[..take])
                .map_err(|e| format!("Write error: {}", e))?;
        } else {
            let n = format_stream(&raw_chunk[..take], parsed.format, &mut fmt_chunk);
            writer
                .write_all(&fmt_chunk[..n])
                .map_err(|e| format!("Write error: {}", e))?;
        }

        remaining -= take as u64;
    }

    writer.flush().map_err(|e| format!("Flush error: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_str() {
        assert_eq!(parse_size_str("1024"), Some(1024));
        assert_eq!(parse_size_str("1KB"), Some(1024));
        assert_eq!(parse_size_str("10MB"), Some(10 * 1024 * 1024));
        assert_eq!(parse_size_str("1GB"), Some(1024 * 1024 * 1024));
        assert_eq!(parse_size_str("invalid"), None);
    }

    #[test]
    fn test_run_generate_cli() {
        let temp_file = "test_gen_output.bin";
        let args = vec![
            "-t".to_string(),
            "lfsr".to_string(),
            "-s".to_string(),
            "1KB".to_string(),
            "-o".to_string(),
            temp_file.to_string(),
        ];
        let res = run_generate_cli(&args);
        assert!(res.is_ok());
        let meta = std::fs::metadata(temp_file).unwrap();
        assert_eq!(meta.len(), 1024);
        let _ = std::fs::remove_file(temp_file);
    }
}
