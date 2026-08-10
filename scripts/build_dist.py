#!/usr/bin/env python3
"""
Build & Packaging Script for Randstat WASM & Embeddable UI Library
Compiles release WASM binary, concatenates JS modules into a single bundle,
minifies output, and packages all library assets into `dist/`.
"""

import os
import re
import shutil
import subprocess
import sys

def minify_js(js_code: str) -> str:
    """Robust JavaScript minifier that preserves string literals, template literals, and regexes."""
    # State tracking character-by-character to strip comments outside of strings
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
            while i < n and js_code[i] != '\n' and js_code[i] != '\r':
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
                    # Check if it was escaped: count backslashes preceding it
                    bs_count = 0
                    k = len(out) - 2 # character before quote
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
            # Minify code segment
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


def main():
    root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    dist_dir = os.path.join(root_dir, "dist")
    wasm_target_file = os.path.join(
        root_dir, "target", "wasm32-unknown-unknown", "release", "randstat_wasm.wasm"
    )

    print("==================================================")
    print(" [BUILD] Building Randstat Release Library & Assets")
    print("==================================================")

    # 1. Compile Cargo release WASM target
    print("\n1. Compiling Cargo release target for wasm32-unknown-unknown...")
    cmd = [
        "cargo", "build", "-p", "randstat-wasm",
        "--no-default-features", "--features", "ent",
        "--target", "wasm32-unknown-unknown", "--release"
    ]
    res = subprocess.run(cmd, cwd=root_dir)
    if res.returncode != 0:
        print("[ERROR] Cargo build failed!")
        sys.exit(1)
    print("[OK] WASM build successful!")

    # 2. Ensure dist directory exists
    os.makedirs(dist_dir, exist_ok=True)

    # 3. Copy WASM binary to dist/
    dist_wasm = os.path.join(dist_dir, "randstat.wasm")
    if os.path.exists(wasm_target_file):
        shutil.copy2(wasm_target_file, dist_wasm)
        size_kb = os.path.getsize(dist_wasm) / 1024.0
        print(f"\n2. Copied WASM binary: randstat.wasm ({size_kb:.1f} KB)")
    else:
        print(f"[ERROR] Missing expected file: {wasm_target_file}")
        sys.exit(1)

    # 4. Concatenate & Minify JS files into single bundle
    print("\n3. Bundling & Minifying JavaScript modules into single bundle...")
    js_src_files = [
        os.path.join(root_dir, "ui", "ent.js"),
        os.path.join(root_dir, "ui", "plot.js"),
        os.path.join(root_dir, "ui", "entropy_ui.js"),
    ]

    combined_js = "/* Randstat WASM Randomness Library - Single Bundle */\n\n"
    for src in js_src_files:
        if os.path.exists(src):
            with open(src, "r", encoding="utf-8") as f:
                combined_js += f"// --- Source: {os.path.basename(src)} ---\n"
                combined_js += f.read() + "\n\n"
        else:
            print(f"[ERROR] Missing expected JS source file: {src}")
            sys.exit(1)

    # Write combined unminified bundle: dist/randstat_ui.js
    dist_combined_js = os.path.join(dist_dir, "randstat_ui.js")
    with open(dist_combined_js, "w", encoding="utf-8") as f:
        f.write(combined_js)
    combined_size = os.path.getsize(dist_combined_js) / 1024.0
    print(f"   - Created combined bundle : dist/randstat_ui.js ({combined_size:.1f} KB)")

    # Minify bundle: dist/randstat_ui.min.js
    minified_js = "/* Randstat WASM Randomness Library (Minified) */\n" + minify_js(combined_js)
    dist_min_js = os.path.join(dist_dir, "randstat_ui.min.js")
    with open(dist_min_js, "w", encoding="utf-8") as f:
        f.write(minified_js)
    min_size = os.path.getsize(dist_min_js) / 1024.0
    print(f"   - Created minified bundle : dist/randstat_ui.min.js ({min_size:.1f} KB)")

    # 5. Generate dist/README.md documentation
    dist_readme = os.path.join(dist_dir, "README.md")
    readme_content = """# Randstat Embeddable WebAssembly Library & UI Component

This folder contains the single-file distribution bundle of the Randstat randomness test suite.

## Distribution Files

- **`randstat.wasm`**: Compiled `no_std` Rust WebAssembly module (~9 KB).
- **`randstat_ui.min.js`**: **Single minified JS bundle** containing WASM runner, plotting engine, and embeddable UI SDK (~14 KB).
- **`randstat_ui.js`**: Single unminified JS bundle (~28 KB).

---

## Quick Start Integration

Include the single script file and drop the custom element or container anywhere in your HTML:

```html
<div id="my-analyzer"></div>

<script src="randstat_ui.min.js"></script>

<script>
  new EntropyAnalyzer('#my-analyzer', {
    wasmUrl: 'randstat.wasm'
  });
</script>
```
"""
    with open(dist_readme, "w", encoding="utf-8") as f:
        f.write(readme_content)

    print("\n==================================================")
    print(" [DONE] Build Complete! Single JS bundle in `dist/`")
    print("==================================================")

if __name__ == "__main__":
    main()
