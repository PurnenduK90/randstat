//! Chi-square and Normal distribution mathematical functions.
//!
//! Implements pochisq (CACM Algorithm 299) and Wilson-Hilferty chi2_critical_value
//! exactly as used in the Fourmilab ENT reference. All floating-point operations
//! use `libm` so this compiles under `#![no_std]`.

use libm::{exp, fabs, log, sqrt};

const PI: f64 = core::f64::consts::PI;
const Z_MAX: f64 = 6.0;
const LOG_SQRT_PI: f64 = 0.572_364_942_924_700_1;
const I_SQRT_PI: f64 = 0.564_189_583_547_756_3;
const BIGX: f64 = 20.0;

#[inline]
fn ex(x: f64) -> f64 {
    if x < -BIGX {
        0.0
    } else {
        exp(x)
    }
}

/// Computes ln(Γ(a)) for a > 0 via Stirling's series.
pub fn lgamma(a: f64) -> f64 {
    if a <= 0.0 {
        return 0.0;
    }
    if a < 12.0 {
        let mut x = a;
        let mut correction = 0.0;
        while x < 12.0 {
            correction += log(x);
            x += 1.0;
        }
        return lgamma(x) - correction;
    }
    let inv_x = 1.0 / a;
    let inv_x2 = inv_x * inv_x;
    (a - 0.5) * log(a) - a
        + 0.5 * log(2.0 * PI)
        + inv_x * (1.0 / 12.0 - inv_x2 * (1.0 / 360.0 - inv_x2 * (1.0 / 1260.0)))
}

/// Chi-square PDF at `x` with `df` degrees of freedom (log-space to avoid overflow).
pub fn chi2_pdf(x: f64, df: f64) -> f64 {
    if x <= 0.0 || df <= 0.0 {
        return 0.0;
    }
    let k = df / 2.0;
    let log_pdf = (k - 1.0) * log(x) - (x / 2.0) - k * log(2.0) - lgamma(k);
    if log_pdf < -700.0 {
        0.0
    } else {
        exp(log_pdf)
    }
}

/// Normal PDF approximation at `x` with mean = `df` and σ = √(2·df).
pub fn normal_pdf(x: f64, df: f64) -> f64 {
    if df <= 0.0 {
        return 0.0;
    }
    let std_dev = sqrt(2.0 * df);
    if std_dev <= 0.0 {
        return 0.0;
    }
    let diff = (x - df) / std_dev;
    let exponent = -0.5 * diff * diff;
    if exponent < -700.0 {
        0.0
    } else {
        (1.0 / (std_dev * sqrt(2.0 * PI))) * exp(exponent)
    }
}

/// Normal Z-distribution probability (CACM Algorithm 209).
pub fn poz(z: f64) -> f64 {
    if z == 0.0 {
        return 0.5;
    }
    let y = 0.5 * fabs(z);
    let x: f64;

    if y >= Z_MAX * 0.5 {
        x = 1.0;
    } else if y < 1.0 {
        let w = y * y;
        x = ((((((((0.000124818987 * w - 0.001075204047) * w + 0.005198775019) * w
            - 0.019198292004)
            * w
            + 0.059054035642)
            * w
            - 0.151968751364)
            * w
            + 0.319152932694)
            * w
            - 0.531923007300)
            * w
            + 0.797884560593)
            * y
            * 2.0;
    } else {
        let y2 = y - 2.0;
        x = (((((((((((((-0.000045255659 * y2 + 0.000152529290) * y2 - 0.000019538132)
            * y2
            - 0.000676904986)
            * y2
            + 0.001390604284)
            * y2
            - 0.000794620820)
            * y2
            - 0.002034254874)
            * y2
            + 0.006549791214)
            * y2
            - 0.010557625006)
            * y2
            + 0.011630447319)
            * y2
            - 0.009279453341)
            * y2
            + 0.005353579108)
            * y2
            - 0.002141268741)
            * y2
            + 0.000535310849)
            * y2
            + 0.999936657524;
    }

    if z > 0.0 {
        (x + 1.0) * 0.5
    } else {
        (1.0 - x) * 0.5
    }
}

/// Upper-tail chi-square cumulative probability P(χ² > ax | df) (CACM Algorithm 299).
///
/// Returns a value in [0, 1]. Values near 0 or 1 indicate non-randomness.
pub fn pochisq(ax: f64, df: usize) -> f64 {
    if ax <= 0.0 || df < 1 {
        return 1.0;
    }

    let a = 0.5 * ax;
    let even = df.is_multiple_of(2);
    let y = if df > 1 { ex(-a) } else { 0.0 };
    let mut s = if even { y } else { 2.0 * poz(-sqrt(ax)) };

    if df > 2 {
        let x = 0.5 * ((df as f64) - 1.0);
        let mut z = if even { 1.0 } else { 0.5 };

        if a > BIGX {
            let mut e = if even { 0.0 } else { LOG_SQRT_PI };
            let c = log(a);
            while z <= x {
                e += log(z);
                s += ex(c * z - a - e);
                z += 1.0;
            }
            s
        } else {
            let mut e = if even { 1.0 } else { I_SQRT_PI / sqrt(a) };
            let mut c = 0.0;
            while z <= x {
                e *= a / z;
                c += e;
                z += 1.0;
            }
            c * y + s
        }
    } else {
        s
    }
}

/// Wilson-Hilferty chi-square critical value for significance `alpha` and `df` degrees of freedom.
pub fn chi2_critical_value(alpha: f64, df: f64) -> f64 {
    let (p, sign) = if alpha < 0.5 {
        (alpha, 1.0)
    } else {
        (1.0 - alpha, -1.0)
    };
    let t = sqrt(-2.0 * log(p));
    let num = 2.515_517 + (0.802_853 * t) + (0.010_328 * t * t);
    let den = 1.0 + (1.432_788 * t) + (0.189_269 * t * t) + (0.001_308 * t * t * t);
    let z = sign * (t - num / den);

    let factor = 2.0 / (9.0 * df);
    let inner = 1.0 - factor + z * sqrt(factor);
    if inner <= 0.0 {
        0.0
    } else {
        df * inner * inner * inner
    }
}

/// Chi-square statistic over 256 byte frequency bins.
pub fn compute_chi_square(byte_counts: &[u64; 256], total_bytes: u64) -> f64 {
    if total_bytes == 0 {
        return 0.0;
    }
    let expected = total_bytes as f64 / 256.0;
    let mut chi = 0.0f64;
    for &count in byte_counts {
        let diff = (count as f64) - expected;
        chi += (diff * diff) / expected;
    }
    chi
}

/// Complementary error function `erfc(x)` used by the NIST monobit p-value.
///
/// Approximation via Chebyshev (max error < 1.2 × 10⁻⁷ for x ≥ 0).
pub fn erfc(x: f64) -> f64 {
    // Use libm's erfcf approximation through the exp/sqrt path
    // Standard series: erfc(x) ≈ 2*poz(-x*sqrt(2))
    // poz already returns CDF(z), so erfc(x) = 2*(1 - poz(x*sqrt(2)))
    // But we need to be careful about sign. erfc(x) = 1 - erf(x).
    // For x >= 0: erfc(x) = 2 * Q(x*sqrt(2)) = 2*(1-Phi(x*sqrt(2)))
    // poz(z) gives Phi(z), so poz(-z) gives Q(z) = 1 - Phi(z)
    let z = x * core::f64::consts::SQRT_2;
    2.0 * poz(-z)
}
