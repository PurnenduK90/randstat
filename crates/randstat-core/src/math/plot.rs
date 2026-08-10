//! Distribution plot coordinate generator.
//!
//! Generates interleaved `[x₀, y₀, x₁, y₁, …]` coordinate arrays written into
//! a caller-supplied fixed-size buffer — no heap allocation required.
//! Used by `randstat-wasm` to feed canvas plot data to JavaScript.

use super::chi2::{chi2_pdf, normal_pdf};

/// Generates `steps` (x, y) points for the Chi-square PDF into `out_buffer`.
///
/// Points are interleaved: `[x0, y0, x1, y1, …]`.
/// Returns the total number of `f64` values written (= 2 × actual steps).
pub fn generate_chi2_points(
    df: f64,
    x_min: f64,
    x_max: f64,
    steps: usize,
    out_buffer: &mut [f64],
) -> usize {
    let cap = out_buffer.len() / 2;
    let n = if steps > cap { cap } else { steps };
    if n == 0 {
        return 0;
    }
    let step = (x_max - x_min) / (n as f64);
    for i in 0..n {
        let x = x_min + (i as f64) * step;
        out_buffer[i * 2] = x;
        out_buffer[i * 2 + 1] = chi2_pdf(x, df);
    }
    n * 2
}

/// Generates `steps` (x, y) points for the Normal approximation PDF into `out_buffer`.
///
/// Points are interleaved: `[x0, y0, x1, y1, …]`.
/// Returns the total number of `f64` values written (= 2 × actual steps).
pub fn generate_normal_points(
    df: f64,
    x_min: f64,
    x_max: f64,
    steps: usize,
    out_buffer: &mut [f64],
) -> usize {
    let cap = out_buffer.len() / 2;
    let n = if steps > cap { cap } else { steps };
    if n == 0 {
        return 0;
    }
    let step = (x_max - x_min) / (n as f64);
    for i in 0..n {
        let x = x_min + (i as f64) * step;
        out_buffer[i * 2] = x;
        out_buffer[i * 2 + 1] = normal_pdf(x, df);
    }
    n * 2
}
