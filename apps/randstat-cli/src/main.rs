//! `randstat` CLI — Streaming randomness evaluator.
//!
//! Reads binary or ASCII data from a file or stdin, runs the chosen test suite,
//! and outputs results as a terminal table, Markdown report, or JSON.
//!
//! # Usage
//! ```text
//! randstat [FILE] [OPTIONS]
//!
//! Options:
//!   -s, --suite <NAME>    Test suite: ent (default), nist, ais31, ...
//!   -a, --alpha <FLOAT>   Significance level alpha (default: 0.05)
//!   -t, --ascii, --text   ASCII mode (one integer/float value per line)
//!   -m, --md, --markdown  GitHub-Flavoured Markdown report output
//!   -j, --json            JSON metrics output
//!   -h, --help            Show this help message
//! ```

mod generate_cli;
mod report;

use randstat_core::bitstream::sha256::Sha256;
use randstat_suite_ais31::Ais31Suite;
use randstat_suite_ent::EntSuite;
use randstat_suite_nist::NistSuite;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuiteKind {
    Ent,
    Nist,
    Sp80090b,
    Ais31,
    Dieharder,
    Testu01,
    Practrand,
    Gjrand,
    Full,
}

struct CliArgs {
    file_path: Option<String>,
    alpha: f64,
    ascii_mode: bool,
    json_mode: bool,
    md_mode: bool,
    suite: SuiteKind,
}

#[allow(dead_code)]
fn parse_args() -> CliArgs {
    parse_args_from(std::env::args().collect())
}

fn parse_args_from(args: Vec<String>) -> CliArgs {
    let mut file_path = None;
    let mut alpha = 0.05_f64;
    let mut ascii_mode = false;
    let mut json_mode = false;
    let mut md_mode = false;
    let mut suite = SuiteKind::Ent;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-a" | "--alpha" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<f64>() {
                        alpha = val;
                    }
                    i += 1;
                }
            }
            "-s" | "--suite" => {
                if i + 1 < args.len() {
                    suite = match args[i + 1].to_lowercase().as_str() {
                        "nist" => SuiteKind::Nist,
                        "sp80090b" | "sp800-90b" | "90b" => SuiteKind::Sp80090b,
                        "ais31" | "ais-31" => SuiteKind::Ais31,
                        "dieharder" | "diehard" => SuiteKind::Dieharder,
                        "testu01" | "u01" => SuiteKind::Testu01,
                        "practrand" => SuiteKind::Practrand,
                        "gjrand" => SuiteKind::Gjrand,
                        "full" => SuiteKind::Full,
                        _ => SuiteKind::Ent,
                    };
                    i += 1;
                }
            }
            "-t" | "--text" | "--ascii" => ascii_mode = true,
            "-j" | "--json" => json_mode = true,
            "-m" | "--md" | "--markdown" => md_mode = true,
            "-h" | "--help" => {
                print_help();
                #[cfg(not(test))]
                std::process::exit(0);
            }
            arg if !arg.starts_with('-') && file_path.is_none() => {
                file_path = Some(arg.to_string());
            }
            _ => {}
        }
        i += 1;
    }

    CliArgs {
        file_path,
        alpha,
        ascii_mode,
        json_mode,
        md_mode,
        suite,
    }
}

fn print_help() {
    println!("randstat — Streaming Randomness Evaluator (randstat workspace)");
    println!("Usage:");
    println!("  randstat [FILE] [OPTIONS]                      Run randomness evaluation suite");
    println!("  randstat generate [OPTIONS] [-o OUTFILE]       Generate test stream vectors");
    println!();
    println!("Evaluation Options:");
    println!("  -s, --suite <NAME>    Test suite: ent (default), nist, ais31, full, sp80090b, dieharder, testu01, practrand, gjrand");
    println!("  -a, --alpha <FLOAT>   Significance level alpha (default: 0.05)");
    println!("  -t, --ascii, --text   ASCII mode (reads one integer or float per line)");
    println!("  -m, --md, --markdown  Output GitHub-Flavoured Markdown report");
    println!("  -j, --json            Output raw JSON metrics");
    println!("  -h, --help            Show this help message");
    println!();
    println!("Generator Options (`randstat generate`):");
    println!("  -t, --type <TYPE>     Generator type: seq, fixed, lfsr, debruijn, lcg, xoshiro, gaussian, poisson, aes, sha, chacha");
    println!("  -s, --size <SIZE>     Stream size (e.g. 1024, 64KB, 1MB, 10MB, 1GB, 10GB)");
    println!("  -f, --format <FMT>    Format: bin (default), int, bits, hex, float");
    println!("  -o, --output <FILE>   Output destination (default: stdout)");
    println!("      --order <N>       LFSR/DeBruijn order (bits)");
    println!("      --seed <N>        RNG seed integer");
}

fn main() {
    if let Err(e) = run_cli_app(std::env::args().collect()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run_cli_app(args: Vec<String>) -> Result<(), String> {
    if args.len() > 1 && (args[1] == "generate" || args[1] == "gen") {
        return generate_cli::run_generate_cli(&args[2..]);
    }

    let cli_args = parse_args_from(args);
    let mut ent_suite = EntSuite::new();
    let mut nist_suite = NistSuite::new();
    let mut ais31_suite = Ais31Suite::new();
    let mut hasher = Sha256::new();
    let mut total_bytes = 0u64;

    // --- Open input source ---
    let input_source: Box<dyn Read> = match &cli_args.file_path {
        Some(path) => match File::open(path) {
            Ok(file) => Box::new(file),
            Err(err) => {
                return Err(format!("Error opening file '{}': {}", path, err));
            }
        },
        None => Box::new(io::stdin()),
    };

    // --- Stream data into suite ---
    if cli_args.ascii_mode {
        let reader = BufReader::new(input_source);
        let mut batch = Vec::with_capacity(8192);

        for line in reader.lines().map_while(Result::ok) {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(val) = trimmed.parse::<i64>() {
                batch.push((val.rem_euclid(256)) as u8);
            } else if let Ok(val) = trimmed.parse::<f64>() {
                batch.push(((val as i64).rem_euclid(256)) as u8);
            } else {
                batch.extend_from_slice(trimmed.as_bytes());
            }

            if batch.len() >= 8192 {
                total_bytes += batch.len() as u64;
                ent_suite.update(&batch);
                nist_suite.update(&batch);
                ais31_suite.update(&batch);
                hasher.update(&batch);
                batch.clear();
            }
        }
        if !batch.is_empty() {
            total_bytes += batch.len() as u64;
            ent_suite.update(&batch);
            nist_suite.update(&batch);
            ais31_suite.update(&batch);
            hasher.update(&batch);
        }
    } else {
        let mut reader = BufReader::with_capacity(65536, input_source);
        let mut buf = [0u8; 65536];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let chunk = &buf[..n];
                    total_bytes += n as u64;
                    ent_suite.update(chunk);
                    nist_suite.update(chunk);
                    ais31_suite.update(chunk);
                    hasher.update(chunk);
                }
                Err(e) => {
                    return Err(format!("Read error: {}", e));
                }
            }
        }
    }

    if total_bytes == 0 {
        return Err("No data processed.".to_string());
    }

    let file_label = cli_args.file_path.as_deref().unwrap_or("<stdin>");
    let sha256_digest = hasher.finalize();

    match cli_args.suite {
        SuiteKind::Nist => {
            let eval = nist_suite.evaluate();
            if cli_args.json_mode {
                report::print_nist_json(
                    &eval,
                    file_label,
                    total_bytes,
                    &sha256_digest,
                    cli_args.alpha,
                );
            } else if cli_args.md_mode {
                report::print_nist_markdown(
                    &eval,
                    file_label,
                    total_bytes,
                    &sha256_digest,
                    cli_args.alpha,
                );
            } else {
                report::print_nist_terminal(
                    &eval,
                    file_label,
                    total_bytes,
                    &sha256_digest,
                    cli_args.alpha,
                );
            }
        }
        SuiteKind::Ais31 => {
            let eval = ais31_suite.evaluate();
            if cli_args.json_mode {
                report::print_ais31_json(
                    &eval,
                    file_label,
                    total_bytes,
                    &sha256_digest,
                    cli_args.alpha,
                );
            } else if cli_args.md_mode {
                report::print_ais31_markdown(
                    &eval,
                    file_label,
                    total_bytes,
                    &sha256_digest,
                    cli_args.alpha,
                );
            } else {
                report::print_ais31_terminal(
                    &eval,
                    file_label,
                    total_bytes,
                    &sha256_digest,
                    cli_args.alpha,
                );
            }
        }
        _ => {
            // Default: ENT Suite
            let mut res = ent_suite.finalize().unwrap();
            res.sha256 = sha256_digest;
            let eval = ent_suite.evaluate(cli_args.alpha).unwrap();

            if cli_args.json_mode {
                report::print_json(&res, &eval, file_label, cli_args.alpha);
            } else if cli_args.md_mode {
                report::print_markdown(&res, &eval, file_label, cli_args.alpha);
            } else {
                report::print_terminal(
                    &res,
                    &eval,
                    file_label,
                    cli_args.alpha,
                    cli_args.ascii_mode,
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args_parsing() {
        // Test parsing help
        let args = vec!["randstat".to_string(), "-h".to_string()];
        let parsed = parse_args_from(args);
        assert!(!parsed.json_mode);

        // Test parsing different options
        let args = vec![
            "randstat".to_string(),
            "-s".to_string(),
            "nist".to_string(),
            "-a".to_string(),
            "0.01".to_string(),
            "-t".to_string(),
            "-j".to_string(),
            "-m".to_string(),
            "test_file.bin".to_string(),
        ];
        let parsed = parse_args_from(args);
        assert_eq!(parsed.suite, SuiteKind::Nist);
        assert_eq!(parsed.alpha, 0.01);
        assert!(parsed.ascii_mode);
        assert!(parsed.json_mode);
        assert!(parsed.md_mode);
        assert_eq!(parsed.file_path.unwrap(), "test_file.bin");

        // Test parsing AIS 31 suite
        let args_ais = vec![
            "randstat".to_string(),
            "-s".to_string(),
            "ais31".to_string(),
        ];
        let parsed_ais = parse_args_from(args_ais);
        assert_eq!(parsed_ais.suite, SuiteKind::Ais31);

        // Test fallback for invalid alpha parse
        let args = vec![
            "randstat".to_string(),
            "-a".to_string(),
            "invalid_float".to_string(),
        ];
        let parsed = parse_args_from(args);
        assert_eq!(parsed.alpha, 0.05); // default
    }

    #[test]
    fn test_run_cli_app() {
        // Test with non-existent file
        let args = vec!["randstat".to_string(), "non_existent_file.bin".to_string()];
        let res = run_cli_app(args);
        assert!(res.is_err());

        // Test with actual file (cryptorandom_1KB.bin exists in tests/testfiles/)
        let test_bin_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/testfiles/cryptorandom_1KB.bin")
            .to_str()
            .unwrap()
            .to_string();
        let args = vec!["randstat".to_string(), test_bin_path.clone()];
        let res = run_cli_app(args);
        assert!(res.is_ok());

        // Test NIST suite mode
        let args_nist = vec![
            "randstat".to_string(),
            "-s".to_string(),
            "nist".to_string(),
            test_bin_path.clone(),
        ];
        let res_nist = run_cli_app(args_nist);
        assert!(res_nist.is_ok());

        // Test AIS 31 suite mode
        let args_ais = vec![
            "randstat".to_string(),
            "-s".to_string(),
            "ais31".to_string(),
            test_bin_path,
        ];
        let res_ais = run_cli_app(args_ais);
        assert!(res_ais.is_ok());

        // Test ASCII mode
        let temp_filename = "temp_test_ascii.txt";
        std::fs::write(temp_filename, "12\n34.5\nhello\n").unwrap();
        let args = vec![
            "randstat".to_string(),
            "-t".to_string(),
            "-j".to_string(),
            temp_filename.to_string(),
        ];
        let res = run_cli_app(args);
        assert!(res.is_ok());
        let _ = std::fs::remove_file(temp_filename);

        // Test generate subcommand
        let gen_temp = "temp_gen_lfsr.bin";
        let args_gen = vec![
            "randstat".to_string(),
            "generate".to_string(),
            "-t".to_string(),
            "lfsr".to_string(),
            "-s".to_string(),
            "2048".to_string(),
            "-o".to_string(),
            gen_temp.to_string(),
        ];
        let res_gen = run_cli_app(args_gen);
        assert!(res_gen.is_ok());
        assert_eq!(std::fs::metadata(gen_temp).unwrap().len(), 2048);
        let _ = std::fs::remove_file(gen_temp);
    }
}
