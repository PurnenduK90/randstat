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

Self-contained distribution for German BSI AIS 20 / AIS 31 TRNG criteria.

## Files
- `randstat-ais31.wasm`: Zero-allocation WASM binary (~16 KB).
- `randstat-ais31.min.js`: Minified runner and dashboard UI (~12 KB).
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
    # 4. Full Combined Package (dist/full/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 4. Packaging Full Suite (dist/full/) ---")
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
            os.path.join(root_dir, "ui", "entropy_ui.js"),
        ],
        full_dir,
        "randstat-full",
        "Full Combined Randomness Battery SDK"
    )

    with open(os.path.join(full_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write("""# Randstat — Full Combined Test Suite Package

Self-contained distribution aggregating all randomness evaluation batteries.

## Files
- `randstat-full.wasm`: Combined zero-allocation WASM binary (~36 KB).
- `randstat-full.min.js`: Complete JavaScript SDK bundle (~45 KB).
- `randstat-full.js`: Unminified bundle for debugging.
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 5. Math-Only Package (dist/math/)
    # ──────────────────────────────────────────────────────────────────────────
    print("\n--- 5. Packaging Math-Only (dist/math/) ---")
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

Lightweight library for Chi-Square/Normal PDF, CDF, critical values, and curve generation.

## Files
- `randstat-math.wasm`: Ultra-lightweight WASM binary (~7 KB).
- `randstat-math.min.js`: Canvas chart rendering engine (~8 KB).
""")

    # ──────────────────────────────────────────────────────────────────────────
    # 6. Root dist/README.md catalog
    # ──────────────────────────────────────────────────────────────────────────
    dist_readme = os.path.join(dist_dir, "README.md")
    readme_content = """# Randstat Modular Distribution Packages

Each directory inside `dist/` is an **independent, self-contained package** with its own compiled WebAssembly binary, JavaScript runner & UI dashboard, and documentation.

## Package Catalog

| Package Directory | Primary Suite | WASM Binary | JS Bundle (Minified) | Key Capabilities |
|---|---|---|---|---|
| [`dist/ent/`](./ent/) | **Fourmilab ENT** | `randstat-ent.wasm` (10 KB) | `randstat-ent.min.js` (12 KB) | Shannon entropy, $\\chi^2$, Mean, Monte Carlo $\\pi$, Serial Corr |
| [`dist/nist/`](./nist/) | **NIST SP 800-22** | `randstat-nist.wasm` (18 KB) | `randstat-nist.min.js` (12 KB) | 15 NIST cryptographic battery tests (§2.1–§2.15) |
| [`dist/ais31/`](./ais31/) | **BSI AIS 20 / AIS 31** | `randstat-ais31.wasm` (16 KB) | `randstat-ais31.min.js` (12 KB) | 9 German BSI physical TRNG criteria (T0–T8) |
| [`dist/full/`](./full/) | **Full Meta-Suite** | `randstat-full.wasm` (36 KB) | `randstat-full.min.js` (45 KB) | Combined meta-suite SDK |
| [`dist/math/`](./math/) | **Math & Plot** | `randstat-math.wasm` (7 KB) | `randstat-math.min.js` (8 KB) | Pure $\\chi^2$/Normal PDF, CDF & canvas curve plotting |

---

## Directory Layout

```text
dist/
├── ent/
│   ├── randstat-ent.wasm
│   ├── randstat-ent.js
│   ├── randstat-ent.min.js
│   └── README.md
├── nist/
│   ├── randstat-nist.wasm
│   ├── randstat-nist.js
│   ├── randstat-nist.min.js
│   └── README.md
├── ais31/
│   ├── randstat-ais31.wasm
│   ├── randstat-ais31.js
│   ├── randstat-ais31.min.js
│   └── README.md
├── full/
│   ├── randstat-full.wasm
│   ├── randstat-full.js
│   ├── randstat-full.min.js
│   └── README.md
├── math/
│   ├── randstat-math.wasm
│   ├── randstat-math.js
│   ├── randstat-math.min.js
│   └── README.md
└── README.md
```
"""
    with open(dist_readme, "w", encoding="utf-8") as f:
        f.write(readme_content)

    print("\n==========================================================")
    print(" [DONE] Independent folders generated inside `dist/`!")
    print("==========================================================")

if __name__ == "__main__":
    main()
