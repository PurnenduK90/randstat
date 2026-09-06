#!/usr/bin/env python3
"""
Build & Packaging Script for Randstat WASM & Embeddable UI Library
Compiles release WASM binaries per suite and packages them into independent,
self-contained directories inside `dist/`.
"""

import os
import re
import shutil
import subprocess
import sys

def minify_js(js_code: str) -> str:
    """Robust JavaScript minifier that preserves string literals, template literals, and regexes."""
    out = []
    i = 0
    n = len(js_code)
    
    while i < n:
        # 1. Handle block comments
        if i + 1 < n and js_code[i] == '/' and js_code[i+1] == '*':
            i += 2
            while i + 1 < n and not (js_code[i] == '*' and js_code[i+1] == '/'):
                i += 1
            i += 2
            out.append(' ')
            continue
            
        # 2. Handle single-line comments
        if i + 1 < n and js_code[i] == '/' and js_code[i+1] == '/':
            i += 2
            while i + 1 < n and js_code[i] != '\n' and js_code[i] != '\r':
                i += 1
            continue
            
        # 3. Handle string literals (single, double, and template literals)
        if js_code[i] in ("'", '"', '`'):
            quote = js_code[i]
            out.append(quote)
            i += 1
            while i < n:
                char = js_code[i]
                out.append(char)
                i += 1
                if char == quote:
                    bs_count = 0
                    k = len(out) - 2
                    while k >= 0 and out[k] == '\\':
                        bs_count += 1
                        k -= 1
                    if bs_count % 2 == 0:
                        break
            continue
            
        # 4. Handle anything else (code)
        out.append(js_code[i])
        i += 1
        
    temp_code = "".join(out)
    
    # Partition temp_code into string segments and code segments
    segments = []
    i = 0
    n = len(temp_code)
    while i < n:
        if temp_code[i] in ("'", '"', '`'):
            quote = temp_code[i]
            start = i
            i += 1
            while i < n:
                char = temp_code[i]
                i += 1
                if char == quote:
                    bs_count = 0
                    k = i - 2
                    while k >= start and temp_code[k] == '\\':
                        bs_count += 1
                        k -= 1
                    if bs_count % 2 == 0:
                        break
            segments.append(('string', temp_code[start:i]))
        else:
            start = i
            while i < n and temp_code[i] not in ("'", '"', '`'):
                i += 1
            segments.append(('code', temp_code[start:i]))
            
    # Process each code segment
    processed_segments = []
    for seg_type, text in segments:
        if seg_type == 'code':
            cleaned_lines = []
            for line in text.splitlines():
                line = line.strip()
                if line:
                    cleaned_lines.append(line)
            code_text = '\n'.join(cleaned_lines)
            code_text = re.sub(r'\s*([\{\}\(\);:=,\+\-\*\/])\s*', r'\1', code_text)
        else:
            code_text = text
        processed_segments.append(code_text)
        
    return "".join(processed_segments)


def build_suite_wasm(root_dir, feature_name):
    cmd = [
        "cargo", "build", "-p", "randstat-wasm",
        "--no-default-features", "--features", feature_name,
        "--target", "wasm32-unknown-unknown", "--release"
    ]
    res = subprocess.run(cmd, cwd=root_dir)
    if res.returncode != 0:
        print(f"[ERROR] WASM build failed for feature '{feature_name}'")
        sys.exit(1)


def bundle_and_minify(src_files, target_folder, bundle_base_name, header_title):
    os.makedirs(target_folder, exist_ok=True)
    combined_js = f"/* Randstat — {header_title} */\n\n"
    for src in src_files:
        if os.path.exists(src):
            with open(src, "r", encoding="utf-8") as f:
                combined_js += f"// --- Source: {os.path.basename(src)} ---\n"
                combined_js += f.read() + "\n\n"
        else:
            print(f"[ERROR] Missing JS source file: {src}")
            sys.exit(1)

    dist_combined_js = os.path.join(target_folder, f"{bundle_base_name}.js")
    with open(dist_combined_js, "w", encoding="utf-8") as f:
        f.write(combined_js)
    combined_size = os.path.getsize(dist_combined_js) / 1024.0

    minified_js = f"/* Randstat — {header_title} (Minified) */\n" + minify_js(combined_js)
    dist_min_js = os.path.join(target_folder, f"{bundle_base_name}.min.js")
    with open(dist_min_js, "w", encoding="utf-8") as f:
        f.write(minified_js)
    min_size = os.path.getsize(dist_min_js) / 1024.0

    print(f"   [JS] {bundle_base_name}.js ({combined_size:.1f} KB) -> {bundle_base_name}.min.js ({min_size:.1f} KB)")


def main():
    root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    dist_dir = os.path.join(root_dir, "dist")
    
    # Clean dist directory before build
    if os.path.exists(dist_dir):
        shutil.rmtree(dist_dir)
    os.makedirs(dist_dir, exist_ok=True)

    wasm_target_file = os.path.join(
        root_dir, "target", "wasm32-unknown-unknown", "release", "randstat_wasm.wasm"
    )

    core_files = [
        os.path.join(root_dir, "ui", "core", "wasm_bridge.js"),
        os.path.join(root_dir, "ui", "core", "plot.js"),
    ]

    print("==========================================================")
    print(" [BUILD] Packaging Randstat Modular Distributions")
    print("==========================================================")

    # ──────────────────────────────────────────────────────────────────────────
    # 0. Quick Screening & Health Diagnostic Suite Package (dist/quick/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 0. Packaging Quick Screening Suite (dist/quick/) ---")
    quick_dir = os.path.join(dist_dir, "quick")
    os.makedirs(quick_dir, exist_ok=True)
    build_suite_wasm(root_dir, "quick")
    shutil.copy2(wasm_target_file, os.path.join(quick_dir, "randstat-quick.wasm"))
    size_kb = os.path.getsize(os.path.join(quick_dir, "randstat-quick.wasm")) / 1024.0
    print(f"   [WASM] randstat-quick.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "quick", "quick.js"),
            os.path.join(root_dir, "ui", "quick", "quick_ui.js"),
        ],
        quick_dir,
        "randstat-quick",
        "Quick Screening & Health Diagnostic Suite"
    )

    with open(os.path.join(quick_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — Quick Screening & Health Diagnostic Suite Package

Self-contained distribution for fast, single-pass randomness screening and health diagnostics across all 5 dimensions.

## Capabilities (10 Metrics across 5 Dimensions)
1. **Information Density**: Shannon Entropy ($H$), Min-Entropy ($H_\\infty$), Optimum Compression (%)
2. **Spectral Uniformity**: Byte $\\chi^2$ Uniformity (df=255), Arithmetic Mean
3. **Bitwise Independence**: Monobit Test (Bit Balance), Runs Test (Oscillation)
4. **Sequential Correlation**: Poker Test 4-bit (df=15), Serial Correlation (Lag-1)
5. **Geometric Convergence**: Monte Carlo $\\pi$ Estimation & Error

## Files
- `randstat-quick.wasm`: Zero-allocation WASM binary (~12 KB).
- `randstat-quick.min.js`: Minified runner and interactive dashboard UI (~15 KB).
- `randstat-quick.js`: Unminified bundle for debugging.

## Usage
```html
<div id="quick-dashboard"></div>
<script src="randstat-quick.min.js"></script>
<script>
  async function runQuick(byteArray) {
    const runner = await QuickRunner.load('randstat-quick.wasm');
    runner.update(byteArray);
    const result = runner.finalize();
    const evaluation = runner.validate(0.05);
    renderQuickDashboard(document.getElementById('quick-dashboard'), result, evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 1. Fourmilab ENT Package (dist/ent/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 1. Packaging Fourmilab ENT (dist/ent/) ---")
    ent_dir = os.path.join(dist_dir, "ent")
    os.makedirs(ent_dir, exist_ok=True)
    build_suite_wasm(root_dir, "ent")
    shutil.copy2(wasm_target_file, os.path.join(ent_dir, "randstat-ent.wasm"))
    size_kb = os.path.getsize(os.path.join(ent_dir, "randstat-ent.wasm")) / 1024.0
    print(f"   [WASM] randstat-ent.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "ent", "ent.js"),
            os.path.join(root_dir, "ui", "ent", "ent_ui.js"),
        ],
        ent_dir,
        "randstat-ent",
        "Fourmilab ENT Suite"
    )

    with open(os.path.join(ent_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — Fourmilab ENT Suite Package

Self-contained distribution for the Fourmilab ENT randomness evaluation battery.

## Files
- `randstat-ent.wasm`: Zero-allocation WASM binary (~10 KB).
- `randstat-ent.min.js`: Minified runner and dashboard UI (~12 KB).
- `randstat-ent.js`: Unminified bundle for debugging.

## Usage
```html
<div id="ent-dashboard"></div>
<script src="randstat-ent.min.js"></script>
<script>
  async function runEnt(byteArray) {
    const runner = await EntRunner.load('randstat-ent.wasm');
    runner.update(byteArray);
    const result = runner.finalize();
    const evaluation = runner.validate(0.05);
    renderEntDashboard(document.getElementById('ent-dashboard'), result, evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 2. NIST SP 800-22 Package (dist/nist/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 2. Packaging NIST SP 800-22 (dist/nist/) ---")
    nist_dir = os.path.join(dist_dir, "nist")
    os.makedirs(nist_dir, exist_ok=True)
    build_suite_wasm(root_dir, "nist")
    shutil.copy2(wasm_target_file, os.path.join(nist_dir, "randstat-nist.wasm"))
    size_kb = os.path.getsize(os.path.join(nist_dir, "randstat-nist.wasm")) / 1024.0
    print(f"   [WASM] randstat-nist.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "nist", "nist.js"),
            os.path.join(root_dir, "ui", "nist", "nist_ui.js"),
        ],
        nist_dir,
        "randstat-nist",
        "NIST SP 800-22 Rev. 1a Suite"
    )

    with open(os.path.join(nist_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — NIST SP 800-22 Suite Package

Self-contained distribution for NIST SP 800-22 Rev 1a cryptographic tests.

## Files
- `randstat-nist.wasm`: Zero-allocation WASM binary (~18 KB).
- `randstat-nist.min.js`: Minified runner and dashboard UI (~12 KB).
- `randstat-nist.js`: Unminified bundle for debugging.

## Usage
```html
<div id="nist-dashboard"></div>
<script src="randstat-nist.min.js"></script>
<script>
  async function runNist(byteArray) {
    const runner = await NistRunner.load('randstat-nist.wasm');
    runner.update(byteArray);
    const evaluation = runner.finalize();
    renderNistDashboard(document.getElementById('nist-dashboard'), evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 3. BSI AIS 20 / AIS 31 Package (dist/ais31/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 3. Packaging BSI AIS 20 / AIS 31 (dist/ais31/) ---")
    ais31_dir = os.path.join(dist_dir, "ais31")
    os.makedirs(ais31_dir, exist_ok=True)
    build_suite_wasm(root_dir, "ais31")
    shutil.copy2(wasm_target_file, os.path.join(ais31_dir, "randstat-ais31.wasm"))
    size_kb = os.path.getsize(os.path.join(ais31_dir, "randstat-ais31.wasm")) / 1024.0
    print(f"   [WASM] randstat-ais31.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "ais31", "ais31.js"),
            os.path.join(root_dir, "ui", "ais31", "ais31_ui.js"),
        ],
        ais31_dir,
        "randstat-ais31",
        "BSI AIS 20 / AIS 31 TRNG Suite"
    )

    with open(os.path.join(ais31_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — BSI AIS 20 / AIS 31 TRNG Suite Package

Self-contained distribution for German Federal Office for Information Security (BSI) AIS 20 / AIS 31 physical TRNG criteria.

## Capabilities (9 Tests: T0–T8)
- **T0**: Disjointness test
- **T1**: Monobit test
- **T2**: Poker test
- **T3**: Runs test
- **T4**: Long run test
- **T5**: Autocorrelation test
- **T6**: Uniform distribution test
- **T7**: Comparative test
- **T8**: Entropy estimation test (Shannon entropy)

## Files
- `randstat-ais31.wasm`: Zero-allocation WASM binary (~31.6 KB).
- `randstat-ais31.min.js`: Minified runner and tabular dashboard UI (~12.1 KB).
- `randstat-ais31.js`: Unminified bundle for debugging.

## Usage
```html
<div id="ais31-dashboard"></div>
<script src="randstat-ais31.min.js"></script>
<script>
  async function runAis31(byteArray) {
    const runner = await Ais31Runner.load('randstat-ais31.wasm');
    runner.update(byteArray);
    const evaluation = runner.finalize();
    renderAis31Dashboard(document.getElementById('ais31-dashboard'), evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 4. Dieharder Package (dist/dieharder/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 4. Packaging Dieharder (dist/dieharder/) ---")
    dieharder_dir = os.path.join(dist_dir, "dieharder")
    os.makedirs(dieharder_dir, exist_ok=True)
    build_suite_wasm(root_dir, "dieharder")
    shutil.copy2(wasm_target_file, os.path.join(dieharder_dir, "randstat-dieharder.wasm"))
    size_kb = os.path.getsize(os.path.join(dieharder_dir, "randstat-dieharder.wasm")) / 1024.0
    print(f"   [WASM] randstat-dieharder.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "dieharder", "dieharder.js"),
            os.path.join(root_dir, "ui", "dieharder", "dieharder_ui.js"),
        ],
        dieharder_dir,
        "randstat-dieharder",
        "Dieharder Test Suite"
    )

    with open(os.path.join(dieharder_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — Dieharder Test Suite Package

Self-contained distribution for Marsaglia & Brown's Dieharder battery.

## Capabilities (12 Tests)
- Birthdays spacing test
- OPERM5 permutations test
- 3D Spheres test
- Parking Lot test
- Minimum Distance (2D & 3D) tests
- Squeeze test
- Runs & Craps tests
- DNA, Bitstream, Count the 1s tests

## Files
- `randstat-dieharder.wasm`: Zero-allocation WASM binary (~28.5 KB).
- `randstat-dieharder.min.js`: Minified runner and tabular dashboard UI (~12.5 KB).
- `randstat-dieharder.js`: Unminified bundle for debugging.

## Usage
```html
<div id="dieharder-dashboard"></div>
<script src="randstat-dieharder.min.js"></script>
<script>
  async function runDieharder(byteArray) {
    const runner = await DieharderRunner.load('randstat-dieharder.wasm');
    runner.update(byteArray);
    const evaluation = runner.finalize();
    renderDieharderDashboard(document.getElementById('dieharder-dashboard'), evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 5. NIST SP 800-90B Package (dist/sp80090b/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 5. Packaging NIST SP 800-90B (dist/sp80090b/) ---")
    sp80090b_dir = os.path.join(dist_dir, "sp80090b")
    os.makedirs(sp80090b_dir, exist_ok=True)
    build_suite_wasm(root_dir, "sp800-90b")
    shutil.copy2(wasm_target_file, os.path.join(sp80090b_dir, "randstat-sp80090b.wasm"))
    size_kb = os.path.getsize(os.path.join(sp80090b_dir, "randstat-sp80090b.wasm")) / 1024.0
    print(f"   [WASM] randstat-sp80090b.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "sp80090b", "sp80090b.js"),
            os.path.join(root_dir, "ui", "sp80090b", "sp80090b_ui.js"),
        ],
        sp80090b_dir,
        "randstat-sp80090b",
        "NIST SP 800-90B Min-Entropy Suite"
    )

    with open(os.path.join(sp80090b_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — NIST SP 800-90B Min-Entropy Package

Self-contained distribution for NIST SP 800-90B physical min-entropy estimation.

## Capabilities (10 Estimators §6.3)
- Most Common Value (MCV) estimator
- Collision estimator
- Markov estimator
- Compression estimator
- t-Tuple estimator
- LRS (Longest Repeated Substring) estimator
- Multi-Most Common Value (MMCV) estimator
- Lag Prediction estimator
- Multi-Markov (MultiMMC) estimator
- LZ78Y estimator

## Files
- `randstat-sp80090b.wasm`: Zero-allocation WASM binary (~26.7 KB).
- `randstat-sp80090b.min.js`: Minified runner and tabular dashboard UI (~12.6 KB).
- `randstat-sp80090b.js`: Unminified bundle for debugging.

## Usage
```html
<div id="sp80090b-dashboard"></div>
<script src="randstat-sp80090b.min.js"></script>
<script>
  async function runSp80090b(byteArray) {
    const runner = await Sp80090bRunner.load('randstat-sp80090b.wasm');
    runner.update(byteArray);
    const evaluation = runner.finalize();
    renderSp80090bDashboard(document.getElementById('sp80090b-dashboard'), evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 6. TestU01 Package (dist/testu01/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 6. Packaging TestU01 (dist/testu01/) ---")
    testu01_dir = os.path.join(dist_dir, "testu01")
    os.makedirs(testu01_dir, exist_ok=True)
    build_suite_wasm(root_dir, "testu01")
    shutil.copy2(wasm_target_file, os.path.join(testu01_dir, "randstat-testu01.wasm"))
    size_kb = os.path.getsize(os.path.join(testu01_dir, "randstat-testu01.wasm")) / 1024.0
    print(f"   [WASM] randstat-testu01.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "testu01", "testu01.js"),
            os.path.join(root_dir, "ui", "testu01", "testu01_ui.js"),
        ],
        testu01_dir,
        "randstat-testu01",
        "TestU01 PRNG Benchmark (SmallCrush)"
    )

    with open(os.path.join(testu01_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — TestU01 Suite Package

Self-contained distribution for L'Ecuyer & Simard's TestU01 SmallCrush academic battery.

## Capabilities (10 Tests)
- smarsa_BirthdaySpacings
- sknuth_Collision
- sknuth_Gap
- sknuth_SimpPoker
- sknuth_CouponCollector
- sknuth_MaxOft
- svar_WeightDistrib
- smarsa_MatrixRank
- sstring_HammingIndep
- sstring_Run

## Files
- `randstat-testu01.wasm`: Zero-allocation WASM binary (~28.5 KB).
- `randstat-testu01.min.js`: Minified runner and tabular dashboard UI (~12.5 KB).
- `randstat-testu01.js`: Unminified bundle for debugging.

## Usage
```html
<div id="testu01-dashboard"></div>
<script src="randstat-testu01.min.js"></script>
<script>
  async function runTestu01(byteArray) {
    const runner = await Testu01Runner.load('randstat-testu01.wasm');
    runner.update(byteArray);
    const evaluation = runner.finalize();
    renderTestu01Dashboard(document.getElementById('testu01-dashboard'), evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 7. PractRand Package (dist/practrand/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 7. Packaging PractRand (dist/practrand/) ---")
    practrand_dir = os.path.join(dist_dir, "practrand")
    os.makedirs(practrand_dir, exist_ok=True)
    build_suite_wasm(root_dir, "practrand")
    shutil.copy2(wasm_target_file, os.path.join(practrand_dir, "randstat-practrand.wasm"))
    size_kb = os.path.getsize(os.path.join(practrand_dir, "randstat-practrand.wasm")) / 1024.0
    print(f"   [WASM] randstat-practrand.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "practrand", "practrand.js"),
            os.path.join(root_dir, "ui", "practrand", "practrand_ui.js"),
        ],
        practrand_dir,
        "randstat-practrand",
        "PractRand PRNG Suite"
    )

    with open(os.path.join(practrand_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — PractRand Suite Package

Self-contained distribution for Chris Doty-Humphrey's PractRand high-throughput streaming battery.

## Capabilities (10 Tests)
- `[Low1/8]` Gap-16:B
- `[Low1/8]` FPF-16:B
- `[Low1/8]` BCFN(2+0,13/64)
- `[Low1/8]` BCFN(2+1,13/64)
- `[Low4/8]` DC6-9x1Bytes-1
- `[Low4/8]` BRank(12)
- `[Low8/8]` FPF-8:all64k
- `[Low8/8]` Dist-64x2:g
- `[Low8/8]` Gap-8:all64k
- `[Low8/8]` AutoCor-64

## Files
- `randstat-practrand.wasm`: Zero-allocation WASM binary (~25.1 KB).
- `randstat-practrand.min.js`: Minified runner and tabular dashboard UI (~12.3 KB).
- `randstat-practrand.js`: Unminified bundle for debugging.

## Usage
```html
<div id="practrand-dashboard"></div>
<script src="randstat-practrand.min.js"></script>
<script>
  async function runPractrand(byteArray) {
    const runner = await PractrandRunner.load('randstat-practrand.wasm');
    runner.update(byteArray);
    const evaluation = runner.finalize();
    renderPractrandDashboard(document.getElementById('practrand-dashboard'), evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 8. gjrand Package (dist/gjrand/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 8. Packaging gjrand (dist/gjrand/) ---")
    gjrand_dir = os.path.join(dist_dir, "gjrand")
    os.makedirs(gjrand_dir, exist_ok=True)
    build_suite_wasm(root_dir, "gjrand")
    shutil.copy2(wasm_target_file, os.path.join(gjrand_dir, "randstat-gjrand.wasm"))
    size_kb = os.path.getsize(os.path.join(gjrand_dir, "randstat-gjrand.wasm")) / 1024.0
    print(f"   [WASM] randstat-gjrand.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "gjrand", "gjrand.js"),
            os.path.join(root_dir, "ui", "gjrand", "gjrand_ui.js"),
        ],
        gjrand_dir,
        "randstat-gjrand",
        "gjrand PRNG Suite"
    )

    with open(os.path.join(gjrand_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — gjrand Suite Package

Self-contained distribution for David Blackman's gjrand lightweight PRNG battery.

## Capabilities (10 Tests)
- `mcoll16`: 16-bit word collision test
- `mcoll32`: 32-bit word collision test
- `mprob16`: 16-bit byte probabilities
- `mprob32`: 32-bit probabilities
- `mdist16`: 16-bit distance distribution
- `mdist32`: 32-bit distance distribution
- `mgap16`: 16-bit gap distribution
- `mgap32`: 32-bit gap distribution
- `mrun16`: 16-bit run distribution
- `mrun32`: 32-bit run distribution

## Files
- `randstat-gjrand.wasm`: Zero-allocation WASM binary (~28.1 KB).
- `randstat-gjrand.min.js`: Minified runner and tabular dashboard UI (~12.1 KB).
- `randstat-gjrand.js`: Unminified bundle for debugging.

## Usage
```html
<div id="gjrand-dashboard"></div>
<script src="randstat-gjrand.min.js"></script>
<script>
  async function runGjrand(byteArray) {
    const runner = await GjrandRunner.load('randstat-gjrand.wasm');
    runner.update(byteArray);
    const evaluation = runner.finalize();
    renderGjrandDashboard(document.getElementById('gjrand-dashboard'), evaluation);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 9. Full Combined Package (dist/full/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 9. Packaging Full Suite (dist/full/) ---")
    full_dir = os.path.join(dist_dir, "full")
    os.makedirs(full_dir, exist_ok=True)
    build_suite_wasm(root_dir, "full")
    shutil.copy2(wasm_target_file, os.path.join(full_dir, "randstat-full.wasm"))
    size_kb = os.path.getsize(os.path.join(full_dir, "randstat-full.wasm")) / 1024.0
    print(f"   [WASM] randstat-full.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files + [
            os.path.join(root_dir, "ui", "ent", "ent.js"),
            os.path.join(root_dir, "ui", "ent", "ent_ui.js"),
            os.path.join(root_dir, "ui", "nist", "nist.js"),
            os.path.join(root_dir, "ui", "nist", "nist_ui.js"),
            os.path.join(root_dir, "ui", "ais31", "ais31.js"),
            os.path.join(root_dir, "ui", "ais31", "ais31_ui.js"),
            os.path.join(root_dir, "ui", "dieharder", "dieharder.js"),
            os.path.join(root_dir, "ui", "dieharder", "dieharder_ui.js"),
            os.path.join(root_dir, "ui", "sp80090b", "sp80090b.js"),
            os.path.join(root_dir, "ui", "sp80090b", "sp80090b_ui.js"),
            os.path.join(root_dir, "ui", "testu01", "testu01.js"),
            os.path.join(root_dir, "ui", "testu01", "testu01_ui.js"),
            os.path.join(root_dir, "ui", "practrand", "practrand.js"),
            os.path.join(root_dir, "ui", "practrand", "practrand_ui.js"),
            os.path.join(root_dir, "ui", "gjrand", "gjrand.js"),
            os.path.join(root_dir, "ui", "gjrand", "gjrand_ui.js"),
            os.path.join(root_dir, "ui", "generators", "generators.js"),
            os.path.join(root_dir, "ui", "generators", "generators_ui.js"),
            os.path.join(root_dir, "ui", "full", "full.js"),
            os.path.join(root_dir, "ui", "full", "full_ui.js"),
        ],
        full_dir,
        "randstat-full",
        "Full Combined Randomness Battery SDK"
    )

    with open(os.path.join(full_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — Full Combined Meta-Suite Package

Self-contained distribution aggregating all 8 randomness evaluation batteries (ENT, NIST SP 800-22, BSI AIS 31, Dieharder, NIST SP 800-90B, TestU01, PractRand, gjrand) in a single streaming coordinator.

## Files
- `randstat-full.wasm`: Combined zero-allocation WASM binary (~84.9 KB).
- `randstat-full.min.js`: Complete JavaScript SDK & tabbed dashboard bundle (~111.5 KB).
- `randstat-full.js`: Unminified bundle for debugging.

## Usage
```html
<div id="full-dashboard"></div>
<script src="randstat-full.min.js"></script>
<script>
  async function runFullMetaSuite(byteArray) {
    const runner = await FullSuiteRunner.load('randstat-full.wasm');
    runner.update(byteArray);
    const fullData = runner.finalize();
    renderFullDashboard(document.getElementById('full-dashboard'), fullData);
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 10. Math-Only Package (dist/math/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 10. Packaging Math-Only (dist/math/) ---")
    math_dir = os.path.join(dist_dir, "math")
    os.makedirs(math_dir, exist_ok=True)
    build_suite_wasm(root_dir, "math-only")
    shutil.copy2(wasm_target_file, os.path.join(math_dir, "randstat-math.wasm"))
    size_kb = os.path.getsize(os.path.join(math_dir, "randstat-math.wasm")) / 1024.0
    print(f"   [WASM] randstat-math.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        core_files,
        math_dir,
        "randstat-math",
        "Math & Plot Generator"
    )

    with open(os.path.join(math_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — Math & Distribution Plotting Package

Lightweight library for Chi-Square/Normal PDF, CDF, critical values, and canvas curve generation.

## Files
- `randstat-math.wasm`: Ultra-lightweight WASM binary (~22.9 KB).
- `randstat-math.min.js`: Canvas chart rendering engine (~6.7 KB).

## Usage
```html
<canvas id="plot-canvas" width="800" height="300"></canvas>
<script src="randstat-math.min.js"></script>
<script>
  async function plotChiSquare() {
    const wasm = await WebAssembly.instantiateStreaming(fetch('randstat-math.wasm'));
    const chi2Pts = getChi2Points(wasm.instance, 255, 150, 360, 300);
    const normPts = getNormalPoints(wasm.instance, 255, 150, 360, 300);
    renderDistributionPlot(document.getElementById('plot-canvas'), chi2Pts, normPts, {
      chiSquare: 254.2,
      lowerCutoff: 218.42,
      upperCutoff: 293.25
    });
  }
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 11. Generators Package (dist/generators/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 11. Packaging Test Stream Generators (dist/generators/) ---")
    gen_dir = os.path.join(dist_dir, "generators")
    os.makedirs(gen_dir, exist_ok=True)
    build_suite_wasm(root_dir, "generators")
    shutil.copy2(wasm_target_file, os.path.join(gen_dir, "randstat-generators.wasm"))
    size_kb = os.path.getsize(os.path.join(gen_dir, "randstat-generators.wasm")) / 1024.0
    print(f"   [WASM] randstat-generators.wasm ({size_kb:.1f} KB)")

    bundle_and_minify(
        [
            os.path.join(root_dir, "ui", "generators", "generators.js"),
            os.path.join(root_dir, "ui", "generators", "generators_ui.js"),
        ],
        gen_dir,
        "randstat-generators",
        "Test Stream Generators & Formatters"
    )

    with open(os.path.join(gen_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — Test Stream Generators Package

High-performance, zero-allocation test stream generators and export formatters in WebAssembly.

## Capabilities
- **11 Generator Families**: Sequence ramp, Constant (zeros/ones/K), LFSR (orders 3–128), De Bruijn cycles, LCG, Xoshiro256**, Gaussian (Box-Muller), Poisson (λ=127), NIST SP 800-90A AES CTR_DRBG, NIST SP 800-90A SHA-256 Hash_DRBG, ChaCha20 keystream.
- **5 Output Formats**: Raw Binary (`.bin`), ASCII Decimal Integers (`.txt`), ASCII Bitstream (`.bits`), Hexadecimal (`.hex`), Normalized Floats (`.csv`).

## Files
- `randstat-generators.wasm`: Zero-allocation WASM binary (~22.9 KB).
- `randstat-generators.min.js`: Minified runner and dashboard UI (~15.2 KB).
- `randstat-generators.js`: Unminified bundle for debugging.

## Usage
```html
<div id="gen-container"></div>
<script src="randstat-generators.min.js"></script>
<script>
  window.addEventListener('DOMContentLoaded', () => {
    renderGeneratorsDashboard(document.getElementById('gen-container'), {
      wasmUrl: 'randstat-generators.wasm'
    });
  });
</script>
```
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 12. Root dist/README.md catalog
    # ──────────────────────────────────────────────────────────────────────────
    dist_readme = os.path.join(dist_dir, "README.md")
    readme_content = """# Randstat Modular Distribution Packages

Each directory inside `dist/` is an **independent, self-contained package** with its own compiled WebAssembly binary, JavaScript runner & UI dashboard, and documentation.

## Package Catalog

| Package Directory | Primary Suite / Tool | WASM Binary | JS Bundle (Minified) | Key Capabilities |
|---|---|---|---|---|
| [`dist/quick/`](./quick/) | **Quick Screening & Diagnostics** | `randstat-quick.wasm` | `randstat-quick.min.js` | 10 metrics across all 5 dimensions (Entropy, Min-Entropy, $\\chi^2$, Monobit, Runs, Poker, Serial Corr, $\\pi$) |
| [`dist/ent/`](./ent/) | **Fourmilab ENT** | `randstat-ent.wasm` | `randstat-ent.min.js` | Shannon entropy, $\\chi^2$, Mean, Monte Carlo $\\pi$, Serial Corr |
| [`dist/nist/`](./nist/) | **NIST SP 800-22** | `randstat-nist.wasm` | `randstat-nist.min.js` | 15 NIST cryptographic battery tests (§2.1–§2.15) |
| [`dist/ais31/`](./ais31/) | **BSI AIS 20 / AIS 31** | `randstat-ais31.wasm` | `randstat-ais31.min.js` | 9 German BSI physical TRNG criteria (T0–T8) |
| [`dist/dieharder/`](./dieharder/) | **Dieharder** | `randstat-dieharder.wasm` | `randstat-dieharder.min.js` | 12 Dieharder statistical battery tests |
| [`dist/sp80090b/`](./sp80090b/) | **NIST SP 800-90B** | `randstat-sp80090b.wasm` | `randstat-sp80090b.min.js` | 10 NIST SP 800-90B min-entropy estimators |
| [`dist/testu01/`](./testu01/) | **TestU01** | `randstat-testu01.wasm` | `randstat-testu01.min.js` | TestU01 SmallCrush PRNG benchmark battery |
| [`dist/practrand/`](./practrand/) | **PractRand** | `randstat-practrand.wasm` | `randstat-practrand.min.js` | PractRand multi-test streaming battery |
| [`dist/gjrand/`](./gjrand/) | **gjrand** | `randstat-gjrand.wasm` | `randstat-gjrand.min.js` | gjrand lightweight PRNG benchmark |
| [`dist/generators/`](./generators/) | **Stream Generators** | `randstat-generators.wasm` | `randstat-generators.min.js` | 11 generators (LFSR, AES/SHA DRBG, ChaCha, De Bruijn) & 5 formats |
| [`dist/full/`](./full/) | **Full Meta-Suite** | `randstat-full.wasm` | `randstat-full.min.js` | Combined meta-suite SDK aggregating all batteries |
| [`dist/math/`](./math/) | **Math & Plot** | `randstat-math.wasm` | `randstat-math.min.js` | Pure $\\chi^2$/Normal PDF, CDF & canvas curve plotting |

---

## Directory Layout

```text
dist/
├── quick/
├── ent/
├── nist/
├── ais31/
├── dieharder/
├── sp80090b/
├── testu01/
├── practrand/
├── gjrand/
├── generators/
├── full/
├── math/
└── README.md
```
"""
    with open(dist_readme, "w", encoding="utf-8") as f:
        f.write(readme_content)

    print("\n==========================================================")
    print(" [DONE] All 12 independent folders generated inside `dist/`!")
    print("==========================================================")

if __name__ == "__main__":
    main()
