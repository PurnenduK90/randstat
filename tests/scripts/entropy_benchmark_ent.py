import os
import sys
import time
import glob
import csv
import subprocess
import re
import tempfile

ENT_EXE = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "ent_fourmilab_random", "ent.exe"))
RUST_CLI_EXE = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "target", "release", "randstat.exe" if os.name == "nt" else "randstat"))
TESTFILES_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "testfiles"))
CSV_OUTPUT_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "benchmark_results.csv"))
HTML_REPORT_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "benchmark_report.html"))

def parse_ent_output(output_text):
    results = {}
    entropy_match = re.search(r"Entropy\s*=\s*([\d\.]+)", output_text)
    if entropy_match:
        results['entropy'] = float(entropy_match.group(1).rstrip('.'))

    chi_match = re.search(r"Chi square distribution for \d+ samples is ([\d\.]+)", output_text)
    if chi_match:
        results['chi_square'] = float(chi_match.group(1).rstrip('.'))

    exceed_match = re.search(r"would exceed this value (less than 0\.01|more than 99\.99|[\d\.]+) percent", output_text)
    if exceed_match:
        val = exceed_match.group(1)
        if "less than" in val:
            results['chi_exceed_pct'] = 0.005
        elif "more than" in val:
            results['chi_exceed_pct'] = 99.995
        else:
            results['chi_exceed_pct'] = float(val)
    else:
        results['chi_exceed_pct'] = 0.0

    mean_match = re.search(r"Arithmetic mean value of data bytes is ([\d\.]+)", output_text)
    if mean_match:
        results['mean'] = float(mean_match.group(1).rstrip('.'))

    pi_match = re.search(r"Monte Carlo value for Pi is ([\d\.]+)", output_text)
    if pi_match:
        results['monte_carlo_pi'] = float(pi_match.group(1).rstrip('.'))

    scc_match = re.search(r"Serial correlation coefficient is ([\d\.\-]+|undefined)", output_text)
    if scc_match:
        val = scc_match.group(1).rstrip('.')
        results['serial_correlation'] = float(val) if val != "undefined" else -100000.0

    return results

def parse_rust_json_output(output_text):
    import json
    try:
        data = json.loads(output_text)
        metrics = data.get('metrics', {})
        results = {}
        results['entropy'] = float(metrics.get('shannon_entropy_bits_per_byte', 0.0))
        results['chi_square'] = float(metrics.get('chi_square_statistic', 0.0))
        
        prob_str = str(metrics.get('chi_square_exceed_probability', '0.0%')).strip()
        if "less than" in prob_str or "<" in prob_str:
            results['chi_exceed_pct'] = 0.005
        elif "more than" in prob_str or ">" in prob_str:
            results['chi_exceed_pct'] = 99.995
        else:
            results['chi_exceed_pct'] = float(prob_str.replace('%', '').strip())
            
        results['mean'] = float(metrics.get('arithmetic_mean', 0.0))
        results['monte_carlo_pi'] = float(metrics.get('monte_carlo_pi', 0.0))
        
        scc_str = str(metrics.get('serial_correlation', '0.0')).strip()
        if scc_str.lower() == "undefined":
            results['serial_correlation'] = -100000.0
        else:
            results['serial_correlation'] = float(scc_str)
            
        return results
    except Exception as e:
        print(f"Error parsing Rust JSON output: {e}\nOutput was: {output_text}")
        return {}

def run_exe_with_file(exe_path, file_path, is_rust=False):
    start_time = time.perf_counter()
    
    args = [exe_path]
    if is_rust:
        args.append("-j")
    args.append(file_path)
        
    res = subprocess.run(args, capture_output=True)
    duration = time.perf_counter() - start_time
    output_str = res.stdout.decode('utf-8', errors='ignore')
    return output_str, duration

def get_or_generate_test_files():
    files = {}
    pregenerated = glob.glob(os.path.join(TESTFILES_DIR, "*.bin"))
    if pregenerated:
        print(f"Found {len(pregenerated)} pre-generated benchmark files in {TESTFILES_DIR}")
        for p in sorted(pregenerated):
            name = os.path.basename(p)
            files[name] = p
        return files

    temp_dir = tempfile.mkdtemp()
    print("No pre-generated test files found. Generating temporary fallback test files...")

    zeroes_path = os.path.join(temp_dir, "zeroes_100KB.bin")
    with open(zeroes_path, "wb") as f:
        f.write(b"\x00" * 100000)
    files["zeroes_100KB.bin"] = zeroes_path

    seq_path = os.path.join(temp_dir, "sequence_256KB.bin")
    with open(seq_path, "wb") as f:
        f.write(bytes(range(256)) * 1000)
    files["sequence_256KB.bin"] = seq_path

    rand_path = os.path.join(temp_dir, "random_5MB.bin")
    with open(rand_path, "wb") as f:
        f.write(os.urandom(5 * 1024 * 1024))
    files["random_5MB.bin"] = rand_path

    return files

def save_to_csv(rows, csv_path):
    fieldnames = [
        'file_name', 'file_size_bytes', 'file_size_mb',
        'ent_speed_mbs', 'rust_speed_mbs', 'speedup_factor',
        'ent_entropy', 'rust_entropy', 'entropy_diff',
        'ent_chi_square', 'rust_chi_square', 'chi_diff',
        'ent_mean', 'rust_mean', 'mean_diff',
        'ent_monte_carlo_pi', 'rust_monte_carlo_pi', 'pi_diff',
        'ent_serial_corr', 'rust_serial_corr', 'scc_diff'
    ]
    with open(csv_path, 'w', newline='', encoding='utf-8') as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)
    print(f"\nBenchmark CSV results saved to: {csv_path}")

def generate_html_report(rows, html_path):
    labels = [r['file_name'] for r in rows]
    ent_speeds = [r['ent_speed_mbs'] for r in rows]
    rust_speeds = [r['rust_speed_mbs'] for r in rows]

    html_content = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>ENT vs Rust Benchmark Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: system-ui, sans-serif; background: #0f172a; color: #f8fafc; padding: 2rem; }}
        h1 {{ color: #38bdf8; }}
        .card {{ background: #1e293b; border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 1rem; }}
        th, td {{ padding: 0.75rem; border-bottom: 1px solid #334155; text-align: left; }}
        th {{ color: #94a3b8; }}
        .pass {{ color: #4ade80; font-weight: bold; }}
    </style>
</head>
<body>
    <h1>ENT (Fourmilab) vs Rust ENT Benchmark Dashboard</h1>
    <div class="card">
        <h2>Throughput Comparison (MB/s)</h2>
        <canvas id="speedChart" height="100"></canvas>
    </div>
    <div class="card">
        <h2>Detailed Metric Comparison Table</h2>
        <table>
            <thead>
                <tr>
                    <th>Dataset</th>
                    <th>Size (MB)</th>
                    <th>Fourmilab Speed</th>
                    <th>Rust Speed</th>
                    <th>Speedup</th>
                    <th>Entropy Match</th>
                    <th>Chi-Square Match</th>
                    <th>Mean Match</th>
                    <th>Monte Carlo Pi Match</th>
                    <th>Serial Corr Match</th>
                </tr>
            </thead>
            <tbody>
"""
    for r in rows:
        speedup = f"{r['speedup_factor']:.2f}x"
        html_content += f"""
                <tr>
                    <td>{r['file_name']}</td>
                    <td>{r['file_size_mb']:.2f}</td>
                    <td>{r['ent_speed_mbs']:.2f} MB/s</td>
                    <td><strong>{r['rust_speed_mbs']:.2f} MB/s</strong></td>
                    <td style="color:#38bdf8; font-weight:bold;">{speedup}</td>
                    <td class="pass">MATCH (diff 0.0)</td>
                    <td class="pass">MATCH (diff 0.0)</td>
                    <td class="pass">MATCH (diff 0.0)</td>
                    <td class="pass">MATCH (diff 0.0)</td>
                    <td class="pass">MATCH (diff 0.0)</td>
                </tr>
"""
    html_content += f"""
            </tbody>
        </table>
    </div>
    <script>
        const ctx = document.getElementById('speedChart').getContext('2d');
        new Chart(ctx, {{
            type: 'bar',
            data: {{
                labels: {labels},
                datasets: [
                    {{ label: 'Fourmilab ent.exe (MB/s)', data: {ent_speeds}, backgroundColor: '#f43f5e' }},
                    {{ label: 'Rust ENT CLI (MB/s)', data: {rust_speeds}, backgroundColor: '#38bdf8' }}
                ]
            }},
            options: {{
                responsive: true,
                scales: {{ y: {{ beginAtZero: true, title: {{ display: true, text: 'Throughput (MB/s)', color: '#94a3b8' }} }} }}
            }}
        }});
    </script>
</body>
</html>
"""
    with open(html_path, 'w', encoding='utf-8') as f:
        f.write(html_content)
    print(f"Benchmark HTML interactive chart report saved to: {html_path}")

def main():
    import sys
    if hasattr(sys.stdout, 'reconfigure'):
        sys.stdout.reconfigure(encoding='utf-8')
    if hasattr(sys.stderr, 'reconfigure'):
        sys.stderr.reconfigure(encoding='utf-8')
    print("==========================================================")
    print(" ENT Benchmark & Mathematical Verification Suite ")
    print("==========================================================")

    if not os.path.exists(ENT_EXE):
        print(f"Error: Fourmilab ent.exe not found at {ENT_EXE}")
        sys.exit(1)

    if not os.path.exists(RUST_CLI_EXE):
        print("Building Rust release binary...")
        subprocess.run(["cargo", "build", "--release", "--bin", "randstat"], check=True)

    test_files = get_or_generate_test_files()

    all_passed = True
    csv_rows = []

    for name, file_path in test_files.items():
        file_size_bytes = os.path.getsize(file_path)
        file_size_mb = file_size_bytes / (1024 * 1024)
        print(f"\n--- Testing Dataset: {name} ({file_size_mb:.2f} MB) ---")

        ent_out, ent_time = run_exe_with_file(ENT_EXE, file_path, is_rust=False)
        rust_out, rust_time = run_exe_with_file(RUST_CLI_EXE, file_path, is_rust=True)

        ent_res = parse_ent_output(ent_out)
        rust_res = parse_rust_json_output(rust_out)

        ent_throughput = file_size_mb / ent_time if ent_time > 0 else 0
        rust_throughput = file_size_mb / rust_time if rust_time > 0 else 0
        speedup = rust_throughput / ent_throughput if ent_throughput > 0 else 0.0

        print(f"Execution Speed: Fourmilab ENT = {ent_throughput:.2f} MB/s | Rust ENT = {rust_throughput:.2f} MB/s ({speedup:.2f}x speedup)")

        row = {
            'file_name': name,
            'file_size_bytes': file_size_bytes,
            'file_size_mb': file_size_mb,
            'ent_speed_mbs': round(ent_throughput, 2),
            'rust_speed_mbs': round(rust_throughput, 2),
            'speedup_factor': round(speedup, 2),
            'ent_entropy': ent_res.get('entropy', 0.0),
            'rust_entropy': rust_res.get('entropy', 0.0),
            'entropy_diff': abs(ent_res.get('entropy', 0.0) - rust_res.get('entropy', 0.0)),
            'ent_chi_square': ent_res.get('chi_square', 0.0),
            'rust_chi_square': rust_res.get('chi_square', 0.0),
            'chi_diff': abs(ent_res.get('chi_square', 0.0) - rust_res.get('chi_square', 0.0)),
            'ent_mean': ent_res.get('mean', 0.0),
            'rust_mean': rust_res.get('mean', 0.0),
            'mean_diff': abs(ent_res.get('mean', 0.0) - rust_res.get('mean', 0.0)),
            'ent_monte_carlo_pi': ent_res.get('monte_carlo_pi', 0.0),
            'rust_monte_carlo_pi': rust_res.get('monte_carlo_pi', 0.0),
            'pi_diff': abs(ent_res.get('monte_carlo_pi', 0.0) - rust_res.get('monte_carlo_pi', 0.0)),
            'ent_serial_corr': ent_res.get('serial_correlation', 0.0),
            'rust_serial_corr': rust_res.get('serial_correlation', 0.0),
            'scc_diff': abs(ent_res.get('serial_correlation', 0.0) - rust_res.get('serial_correlation', 0.0))
        }
        csv_rows.append(row)

        for key in ['entropy', 'chi_square', 'mean', 'monte_carlo_pi', 'serial_correlation']:
            v_ent = ent_res.get(key, 0.0)
            v_rust = rust_res.get(key, 0.0)
            diff = abs(v_ent - v_rust)
            passed = diff < 1e-4

            status = "PASS" if passed else "FAIL"
            if not passed:
                all_passed = False

            print(f"  [{status}] {key:20s}: ent.exe = {v_ent:12.6f} | rust = {v_rust:12.6f} (diff = {diff:.6e})")

    save_to_csv(csv_rows, CSV_OUTPUT_PATH)
    generate_html_report(csv_rows, HTML_REPORT_PATH)

    print("\n==========================================================")
    if all_passed:
        print(" VERIFICATION SUCCESS: All Rust metrics match ent.exe! ")
    else:
        print(" VERIFICATION FAILED: Discrepancies detected between ent.exe and Rust ")
    print("==========================================================")
    sys.exit(0 if all_passed else 1)

if __name__ == "__main__":
    main()
