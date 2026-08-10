//! `randstat` CLI — Streaming randomness evaluator.
//!
//! Reads binary or ASCII data from a file or stdin, runs the ENT test suite,
//! and outputs results as a terminal table, Markdown report, or JSON.
//!
//! # Usage
//! ```text
//! randstat [FILE] [OPTIONS]
//!
//! Options:
//!   -a, --alpha <FLOAT>   Significance level alpha (default: 0.05)
//!   -t, --ascii, --text   ASCII mode (one integer/float value per line)
//!   -m, --md, --markdown  GitHub-Flavoured Markdown report output
//!   -j, --json            JSON metrics output
//!   -h, --help            Show this help message
//! ```

mod report;

use randstat_core::bitstream::sha256::Sha256;
use randstat_suite_ent::EntSuite;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

struct CliArgs {
    file_path: Option<String>,
    alpha: f64,
    ascii_mode: bool,
    json_mode: bool,
    md_mode: bool,
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
    }
}

fn print_help() {
    println!("randstat — Streaming Randomness Evaluator (randstat workspace)");
    println!("Usage: randstat [FILE] [--alpha <FLOAT>] [--ascii] [--md] [--json]");
    println!();
    println!("Options:");
    println!("  -a, --alpha <FLOAT>   Significance level alpha (default: 0.05)");
    println!("  -t, --ascii, --text   ASCII mode (reads one integer or float per line)");
    println!("  -m, --md, --markdown  Output GitHub-Flavoured Markdown report");
    println!("  -j, --json            Output raw JSON metrics");
    println!("  -h, --help            Show this help message");
}

fn main() {
    if let Err(e) = run_cli_app(std::env::args().collect()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run_cli_app(args: Vec<String>) -> Result<(), String> {
    let cli_args = parse_args_from(args);
    let mut suite = EntSuite::new();
    // SHA-256 is computed independently — it is file identity, not a statistical test.
    let mut hasher = Sha256::new();

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
                suite.update(&batch);
                hasher.update(&batch);
                batch.clear();
            }
        }
        if !batch.is_empty() {
            suite.update(&batch);
            hasher.update(&batch);
        }
    } else {
        let mut reader = BufReader::with_capacity(65536, input_source);
        let mut buf = [0u8; 65536];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    suite.update(&buf[..n]);
                    hasher.update(&buf[..n]);
                }
                Err(e) => {
                    return Err(format!("Read error: {}", e));
                }
            }
        }
    }

    // --- Finalise and report ---
    match suite.finalize() {
        None => {
            return Err("No data processed.".to_string());
        }
        Some(mut res) => {
            // SHA-256 is file identity — computed independently of the test suite.
            res.sha256 = hasher.finalize();

            let eval = suite.evaluate(cli_args.alpha).unwrap();
            let file_label = cli_args.file_path.as_deref().unwrap_or("<stdin>");

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
            "-a".to_string(),
            "0.01".to_string(),
            "-t".to_string(),
            "-j".to_string(),
            "-m".to_string(),
            "test_file.bin".to_string(),
        ];
        let parsed = parse_args_from(args);
        assert_eq!(parsed.alpha, 0.01);
        assert!(parsed.ascii_mode);
        assert!(parsed.json_mode);
        assert!(parsed.md_mode);
        assert_eq!(parsed.file_path.unwrap(), "test_file.bin");

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

        // Test with actual file (test_random.bin exists in the workspace root)
        let test_bin_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../test_random.bin")
            .to_str()
            .unwrap()
            .to_string();
        let args = vec!["randstat".to_string(), test_bin_path];
        let res = run_cli_app(args);
        assert!(res.is_ok());

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
    }
}
