//! WASM C-ABI exports for Chi-square and Normal math and plot generation.

use randstat_core::math::chi2::{chi2_critical_value, chi2_pdf, normal_pdf, pochisq};
use randstat_core::math::plot::{generate_chi2_points, generate_normal_points};

static mut POINT_BUFFER: [f64; 1000] = [0.0f64; 1000];

/// Chi-Square PDF at `x` with `df` degrees of freedom.
#[no_mangle]
pub extern "C" fn chi2_pdf_export(x: f64, df: f64) -> f64 {
    chi2_pdf(x, df)
}

/// Normal distribution PDF at `x` with mean=`df`, σ=√(2·df).
#[no_mangle]
pub extern "C" fn normal_pdf_export(x: f64, df: f64) -> f64 {
    normal_pdf(x, df)
}

/// Chi-square critical value (Wilson-Hilferty) at `alpha` significance, `df` degrees.
#[no_mangle]
pub extern "C" fn chi2_critical_value_export(alpha: f64, df: f64) -> f64 {
    chi2_critical_value(alpha, df)
}

/// Upper-tail chi-square probability P(χ² > x | df).
#[no_mangle]
pub extern "C" fn chi2_pochisq(x: f64, df: usize) -> f64 {
    pochisq(x, df)
}

/// Pointer to the shared 1000-element f64 plot buffer. Read after calling a `generate_*` fn.
#[no_mangle]
pub extern "C" fn get_buffer_ptr() -> *const f64 {
    unsafe { POINT_BUFFER.as_ptr() }
}

/// Fills buffer with `steps` interleaved (x,y) pairs for Chi-Square PDF. Returns values written.
#[no_mangle]
pub extern "C" fn wasm_generate_chi2_points(
    df: f64,
    x_min: f64,
    x_max: f64,
    steps: usize,
) -> usize {
    unsafe { generate_chi2_points(df, x_min, x_max, steps, &mut POINT_BUFFER) }
}

/// Fills buffer with `steps` interleaved (x,y) pairs for Normal approx PDF. Returns values written.
#[no_mangle]
pub extern "C" fn wasm_generate_normal_points(
    df: f64,
    x_min: f64,
    x_max: f64,
    steps: usize,
) -> usize {
    unsafe { generate_normal_points(df, x_min, x_max, steps, &mut POINT_BUFFER) }
}
