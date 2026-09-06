/**
 * Fourmilab ENT Web UI Component Library & Dashboard Renderer
 * Full-featured randomness analysis dashboard:
 * - EntropyAnalyzer class: Embeddable container with live dropzone, metric boxes, plot, alpha selector & report exporters
 * - renderEntDashboard function: Pure functional renderer
 * - <entropy-analyzer> custom web element
 */

class EntropyAnalyzer {
  /**
   * @param {HTMLElement|string} container - Target container element or CSS selector
   * @param {Object} options - Configuration options
   * @param {string} [options.wasmUrl='randstat-ent.wasm'] - Path to WASM binary
   * @param {Object} [options.theme] - Custom CSS theme variables override
   */
  constructor(container, options = {}) {
    this.container = typeof container === 'string' ? document.querySelector(container) : container;
    if (!this.container) {
      throw new Error(`EntropyAnalyzer: Container '${container}' not found.`);
    }

    this.options = {
      wasmUrl: options.wasmUrl || 'randstat-ent.wasm',
      theme: options.theme || {},
      df: 255
    };

    this.wasmInstance = null;
    this.wasmRunner = null;
    this.currentResult = null;
    this.currentFile = null;
    this.selectedAlpha = 0.05;

    this.init();
  }

  async init() {
    this.renderSkeleton();
    this.applyTheme();
    await this.loadWasm();
    this.attachEvents();
  }

  applyTheme() {
    const defaultTheme = {
      '--entropy-bg': '#090d16',
      '--entropy-card-bg': '#131b2e',
      '--entropy-accent': '#38bdf8',
      '--entropy-emerald': '#10b981',
      '--entropy-amber': '#f59e0b',
      '--entropy-rose': '#f43f5e',
      '--entropy-text': '#f8fafc',
      '--entropy-text-muted': '#94a3b8',
      '--entropy-border': '#1e293b'
    };

    const theme = { ...defaultTheme, ...this.options.theme };
    for (const [key, val] of Object.entries(theme)) {
      this.container.style.setProperty(key, val);
    }
  }

  renderSkeleton() {
    this.container.innerHTML = `
      <style>
        .ea-wrapper {
          font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
          background: var(--entropy-bg, #090d16);
          color: var(--entropy-text, #f8fafc);
          border-radius: 14px;
          max-width: 1000px;
          margin: 0 auto;
        }
        .ea-card {
          background: var(--entropy-card-bg, #131b2e);
          border: 1px solid var(--entropy-border, #1e293b);
          border-radius: 12px;
          padding: 1.25rem;
          margin-bottom: 1.25rem;
        }
        .ea-card-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 0.75rem;
        }
        .ea-dropzone {
          border: 2px dashed var(--entropy-accent, #38bdf8);
          border-radius: 10px;
          padding: 1.75rem;
          text-align: center;
          cursor: pointer;
          background: rgba(56, 189, 248, 0.02);
          transition: all 0.2s ease;
        }
        .ea-dropzone:hover {
          background: rgba(56, 189, 248, 0.08);
        }
        .ea-file-badge {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
          background: rgba(56, 189, 248, 0.1);
          border: 1px solid rgba(56, 189, 248, 0.3);
          color: var(--entropy-accent, #38bdf8);
          padding: 0.35rem 0.75rem;
          border-radius: 20px;
          font-size: 0.85rem;
          margin-top: 0.75rem;
          max-width: 90%;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }
        .ea-verdict {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 0.85rem 1.1rem;
          border-radius: 8px;
          margin-top: 1rem;
          font-weight: 700;
          font-size: 1.1rem;
        }
        .ea-verdict.pass {
          background: rgba(16, 185, 129, 0.12);
          border: 1px solid rgba(16, 185, 129, 0.4);
          color: var(--entropy-emerald, #10b981);
        }
        .ea-verdict.fail {
          background: rgba(244, 63, 94, 0.12);
          border: 1px solid rgba(244, 63, 94, 0.4);
          color: var(--entropy-rose, #f43f5e);
        }
        .ea-verdict.warn {
          background: rgba(245, 158, 11, 0.12);
          border: 1px solid rgba(245, 158, 11, 0.4);
          color: var(--entropy-amber, #f59e0b);
        }
        .ea-btn-group {
          display: flex;
          gap: 0.5rem;
        }
        .ea-btn {
          background: var(--entropy-accent, #38bdf8);
          color: #090d16;
          border: none;
          font-weight: 700;
          font-size: 0.85rem;
          padding: 0.45rem 0.9rem;
          border-radius: 6px;
          cursor: pointer;
          transition: opacity 0.2s ease;
          display: inline-flex;
          align-items: center;
          gap: 0.35rem;
        }
        .ea-btn:hover {
          opacity: 0.9;
        }
        .ea-btn.secondary {
          background: rgba(255, 255, 255, 0.1);
          color: var(--entropy-text, #f8fafc);
        }
        .ea-grid {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
          gap: 1rem;
          margin-top: 1.25rem;
        }
        .ea-box {
          background: rgba(255, 255, 255, 0.02);
          border: 1px solid var(--entropy-border, #1e293b);
          border-radius: 8px;
          padding: 1rem;
          transition: all 0.3s ease;
        }
        .ea-box.pass { border-color: rgba(16, 185, 129, 0.4); background: rgba(16, 185, 129, 0.03); }
        .ea-box.warn { border-color: rgba(245, 158, 11, 0.4); background: rgba(245, 158, 11, 0.03); }
        .ea-box.fail { border-color: rgba(244, 63, 94, 0.5); background: rgba(244, 63, 94, 0.05); }
        .ea-box-label { font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--entropy-text-muted, #94a3b8); margin-bottom: 0.3rem; }
        .ea-box-val { font-size: 1.3rem; font-weight: 700; }
        .ea-box-sub { font-size: 0.75rem; color: var(--entropy-text-muted, #94a3b8); margin-top: 0.3rem; }
        .ea-select {
          background: var(--entropy-bg, #090d16);
          color: var(--entropy-text, #f8fafc);
          border: 1px solid var(--entropy-border, #1e293b);
          border-radius: 6px;
          padding: 0.35rem 0.65rem;
          font-size: 0.85rem;
          cursor: pointer;
          outline: none;
        }
        .ea-canvas { width: 100%; height: 320px; border-radius: 8px; background: var(--entropy-bg, #090d16); border: 1px solid var(--entropy-border, #1e293b); display: block; }
      </style>

      <div class="ea-wrapper">
        <div class="ea-card">
          <div class="ea-dropzone" id="eaDropzone">
            <p style="margin:0; font-size:1.05rem; color:var(--entropy-accent, #38bdf8); font-weight:600;">📁 Click or Drop File Here to Analyze</p>
            <div id="eaFileBadge" class="ea-file-badge" style="display:none;">
              <span id="eaFileName">No file</span> • <span id="eaFileSize">0 B</span><span id="eaFileHash"></span>
            </div>
            <input type="file" id="eaFileInput" style="display:none;">
          </div>

          <div id="eaVerdictBanner" class="ea-verdict pass" style="display:none;">
            <div id="eaVerdictTitle">✓ LIKELY RANDOM</div>
            <div class="ea-btn-group">
              <button id="eaDownloadMdBtn" class="ea-btn">📥 Download Report (.md)</button>
              <button id="eaDownloadJsonBtn" class="ea-btn secondary">JSON</button>
            </div>
          </div>

          <div class="ea-grid">
            <div class="ea-box" id="eaBoxBytes">
              <div class="ea-box-label">Total Bytes</div>
              <div class="ea-box-val" id="eaValBytes">-</div>
              <div class="ea-box-sub">Evaluated byte stream</div>
            </div>
            <div class="ea-box" id="eaBoxEntropy">
              <div class="ea-box-label">Shannon Entropy</div>
              <div class="ea-box-val" id="eaValEntropy">-</div>
              <div class="ea-box-sub">Ideal = 8.0 bits/byte</div>
            </div>
            <div class="ea-box" id="eaBoxCompress">
              <div class="ea-box-label">Optimum Compression</div>
              <div class="ea-box-val" id="eaValCompress">-</div>
              <div class="ea-box-sub" id="eaSubCompress">Size reduction %</div>
            </div>
            <div class="ea-box" id="eaBoxChi">
              <div class="ea-box-label">Chi-Square (χ²)</div>
              <div class="ea-box-val" id="eaValChi">-</div>
              <div class="ea-box-sub">Degrees of freedom df = 255</div>
            </div>
            <div class="ea-box" id="eaBoxExceed">
              <div class="ea-box-label">Exceed Probability</div>
              <div class="ea-box-val" id="eaValExceed">-</div>
              <div class="ea-box-sub">pochisq tail probability %</div>
            </div>
            <div class="ea-box" id="eaBoxMean">
              <div class="ea-box-label">Arithmetic Mean</div>
              <div class="ea-box-val" id="eaValMean">-</div>
              <div class="ea-box-sub">Random average = 127.5000</div>
            </div>
            <div class="ea-box" id="eaBoxPi">
              <div class="ea-box-label">Monte Carlo Pi</div>
              <div class="ea-box-val" id="eaValPi">-</div>
              <div class="ea-box-sub" id="eaSubPi">Ideal = 3.141592653...</div>
            </div>
            <div class="ea-box" id="eaBoxScc">
              <div class="ea-box-label">Serial Correlation</div>
              <div class="ea-box-val" id="eaValScc">-</div>
              <div class="ea-box-sub" id="eaSubScc">Totally uncorrelated = 0.0</div>
            </div>
          </div>
        </div>

        <div class="ea-card">
          <div class="ea-card-header">
            <h3 style="margin:0; font-size:1.1rem; color:var(--entropy-text);">Chi-Square vs Normal Distribution Plot</h3>
            <div style="display:flex; align-items:center; gap:0.5rem;">
              <label for="eaAlphaSelect" style="font-size:0.85rem; color:var(--entropy-text-muted);">Significance (α):</label>
              <select id="eaAlphaSelect" class="ea-select">
                <option value="0.05" selected>α = 0.05 (95% Conf)</option>
                <option value="0.01">α = 0.01 (99% Conf)</option>
                <option value="0.10">α = 0.10 (90% Conf)</option>
              </select>
            </div>
          </div>
          <canvas class="ea-canvas" id="eaPlotCanvas" width="900" height="320"></canvas>
        </div>
      </div>
    `;
  }

  async loadWasm() {
    let response = null;
    try {
      response = await fetch(this.options.wasmUrl);
    } catch (e) {
      console.warn("EntropyAnalyzer: Could not fetch from provided URL", this.options.wasmUrl);
    }

    if (!response || !response.ok) return;

    try {
      const bytes = await response.arrayBuffer();
      const { instance } = await WebAssembly.instantiate(bytes, {});
      this.wasmInstance = instance;

      if (typeof EntRunner !== 'undefined') {
        this.wasmRunner = new EntRunner(instance);
      } else if (typeof EntWasmRunner !== 'undefined') {
        this.wasmRunner = new EntWasmRunner(instance);
      }
      this.updatePlot();
    } catch (err) {
      console.error(err);
    }
  }

  attachEvents() {
    const dropzone = this.container.querySelector('#eaDropzone');
    const fileInput = this.container.querySelector('#eaFileInput');
    const alphaSelect = this.container.querySelector('#eaAlphaSelect');
    const downloadMdBtn = this.container.querySelector('#eaDownloadMdBtn');
    const downloadJsonBtn = this.container.querySelector('#eaDownloadJsonBtn');

    dropzone.addEventListener('click', () => fileInput.click());
    fileInput.addEventListener('change', (e) => {
      if (e.target.files.length > 0) this.processFile(e.target.files[0]);
    });
    dropzone.addEventListener('dragover', (e) => e.preventDefault());
    dropzone.addEventListener('drop', (e) => {
      e.preventDefault();
      if (e.dataTransfer.files.length > 0) this.processFile(e.dataTransfer.files[0]);
    });

    alphaSelect.addEventListener('change', (e) => {
      this.selectedAlpha = parseFloat(e.target.value) || 0.05;
      this.updatePlot();
    });

    downloadMdBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      this.downloadReport('markdown');
    });

    downloadJsonBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      this.downloadReport('json');
    });
  }

  formatBytes(bytes) {
    if (bytes >= 1024 * 1024 * 1024) return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
    if (bytes >= 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
    if (bytes >= 1024) return (bytes / 1024).toFixed(2) + ' KB';
    return bytes + ' B';
  }

  setBoxStatus(boxId, valId, status, color) {
    const box = this.container.querySelector('#' + boxId);
    const val = this.container.querySelector('#' + valId);
    box.className = `ea-box ${status}`;
    if (color) val.style.color = color;
  }

  async processFile(file) {
    if (!file) return;

    this.currentFile = file;
    this.container.querySelector('#eaFileName').innerText = file.name;
    this.container.querySelector('#eaFileSize').innerText = this.formatBytes(file.size);
    this.container.querySelector('#eaFileBadge').style.display = 'inline-flex';

    if (!this.wasmRunner) {
      alert("EntropyAnalyzer: WASM runner is not loaded.");
      return;
    }

    this.wasmRunner.reset();
    const chunkSize = 64 * 1024;
    let offset = 0;

    while (offset < file.size) {
      const chunk = file.slice(offset, offset + chunkSize);
      const buf = new Uint8Array(await chunk.arrayBuffer());
      this.wasmRunner.update(buf);
      offset += chunkSize;
    }

    const res = this.wasmRunner.finalize();
    this.currentResult = res;

    // Display Hash inside Dropzone File Badge
    if (res.sha256) {
      const shortHash = res.sha256.substring(0, 16) + '...';
      this.container.querySelector('#eaFileHash').innerText = ` • SHA-256: ${shortHash}`;
      this.container.querySelector('#eaFileHash').title = `SHA-256: ${res.sha256}`;
    } else {
      this.container.querySelector('#eaFileHash').innerText = '';
    }

    let rawChip = 0.5;
    let exceedText = "N/A";
    if (this.wasmInstance && this.wasmInstance.exports && this.wasmInstance.exports.chi2_pochisq) {
      rawChip = this.wasmInstance.exports.chi2_pochisq(res.chiSquare, 255);
      if (rawChip < 0.0001) exceedText = "< 0.01%";
      else if (rawChip > 0.9999) exceedText = "> 99.99%";
      else exceedText = (rawChip * 100).toFixed(2) + '%';
    }

    const compressReduction = ((8.0 - res.entropy) / 8.0 * 100).toFixed(2);
    const piErr = Math.abs(res.monteCarloPi - Math.PI) / Math.PI * 100;
    const absScc = Math.abs(res.serialCorrelation);

    // --- GUARDRAIL & VERDICT EVALUATION ---
    let isTooUniform = rawChip > 0.9999 || res.chiSquare < 210.0;
    let isNonUniform = rawChip < 0.0001 || absScc >= 0.05 || piErr >= 3.0 || Math.abs(res.mean - 127.5) >= 5.0;

    // Stat Box Highlighting
    if (absScc >= 0.05) this.setBoxStatus('eaBoxScc', 'eaValScc', 'fail', 'var(--entropy-rose)');
    else if (absScc >= 0.01) this.setBoxStatus('eaBoxScc', 'eaValScc', 'warn', 'var(--entropy-amber)');
    else this.setBoxStatus('eaBoxScc', 'eaValScc', 'pass', 'var(--entropy-emerald)');

    if (rawChip < 0.0001 || rawChip > 0.9999) {
      this.setBoxStatus('eaBoxExceed', 'eaValExceed', 'fail', 'var(--entropy-rose)');
      this.setBoxStatus('eaBoxChi', 'eaValChi', 'fail', 'var(--entropy-rose)');
    } else if (rawChip < 0.01 || rawChip > 0.99) {
      this.setBoxStatus('eaBoxExceed', 'eaValExceed', 'warn', 'var(--entropy-amber)');
      this.setBoxStatus('eaBoxChi', 'eaValChi', 'warn', 'var(--entropy-amber)');
    } else {
      this.setBoxStatus('eaBoxExceed', 'eaValExceed', 'pass', 'var(--entropy-emerald)');
      this.setBoxStatus('eaBoxChi', 'eaValChi', 'pass', 'var(--entropy-emerald)');
    }

    if (piErr >= 3.0) this.setBoxStatus('eaBoxPi', 'eaValPi', 'fail', 'var(--entropy-rose)');
    else if (piErr >= 1.0) this.setBoxStatus('eaBoxPi', 'eaValPi', 'warn', 'var(--entropy-amber)');
    else this.setBoxStatus('eaBoxPi', 'eaValPi', 'pass', 'var(--entropy-emerald)');

    const meanDiff = Math.abs(res.mean - 127.5);
    if (meanDiff >= 5.0) this.setBoxStatus('eaBoxMean', 'eaValMean', 'fail', 'var(--entropy-rose)');
    else if (meanDiff >= 1.0) this.setBoxStatus('eaBoxMean', 'eaValMean', 'warn', 'var(--entropy-amber)');
    else this.setBoxStatus('eaBoxMean', 'eaValMean', 'pass', 'var(--entropy-emerald)');

    if (res.entropy < 7.9) this.setBoxStatus('eaBoxEntropy', 'eaValEntropy', 'warn', 'var(--entropy-amber)');
    else this.setBoxStatus('eaBoxEntropy', 'eaValEntropy', 'pass', 'var(--entropy-emerald)');

    if (compressReduction > 5.0) this.setBoxStatus('eaBoxCompress', 'eaValCompress', 'warn', 'var(--entropy-amber)');
    else this.setBoxStatus('eaBoxCompress', 'eaValCompress', 'pass', 'var(--entropy-emerald)');

    // Verdict Banner
    const banner = this.container.querySelector('#eaVerdictBanner');
    const vTitle = this.container.querySelector('#eaVerdictTitle');
    banner.style.display = 'flex';

    if (isTooUniform) {
      banner.className = 'ea-verdict warn';
      vTitle.innerText = '⚠ ARTIFICIAL UNIFORMITY/STRUCTURED';
    } else if (isNonUniform) {
      banner.className = 'ea-verdict fail';
      vTitle.innerText = '⚠ LIKELY BIASED/ANOMALY';
    } else {
      banner.className = 'ea-verdict pass';
      vTitle.innerText = '✓ LIKELY RANDOM';
    }

    this.container.querySelector('#eaValBytes').innerText = res.totalBytes.toLocaleString();
    this.container.querySelector('#eaValEntropy').innerText = res.entropy.toFixed(6) + ' bits/byte';
    this.container.querySelector('#eaValCompress').innerText = compressReduction + '%';
    this.container.querySelector('#eaSubCompress').innerText = `Reduces size by ${Math.round(compressReduction)}%`;
    this.container.querySelector('#eaValChi').innerText = res.chiSquare.toFixed(2);
    this.container.querySelector('#eaValExceed').innerText = exceedText;
    this.container.querySelector('#eaValMean').innerText = res.mean.toFixed(4);
    this.container.querySelector('#eaValPi').innerText = res.monteCarloPi.toFixed(9);
    this.container.querySelector('#eaSubPi').innerText = `Error: ${piErr.toFixed(2)}%`;
    this.container.querySelector('#eaValScc').innerText = res.serialCorrelation.toFixed(6);

    this.updatePlot();
  }

  downloadReport(format = 'markdown') {
    if (!this.currentResult || !this.currentFile) {
      alert("No evaluation result available to download.");
      return;
    }

    const res = this.currentResult;
    const file = this.currentFile;
    const dateStr = new Date().toISOString();
    const compressReduction = ((8.0 - res.entropy) / 8.0 * 100).toFixed(2);
    const piErr = (Math.abs(res.monteCarloPi - Math.PI) / Math.PI * 100).toFixed(2);

    let rawChip = 0.5;
    let exceedText = "N/A";
    if (this.wasmInstance && this.wasmInstance.exports && this.wasmInstance.exports.chi2_pochisq) {
      rawChip = this.wasmInstance.exports.chi2_pochisq(res.chiSquare, 255);
      if (rawChip < 0.0001) exceedText = "< 0.01%";
      else if (rawChip > 0.9999) exceedText = "> 99.99%";
      else exceedText = (rawChip * 100).toFixed(2) + '%';
    }

    const critFn = this.wasmInstance && this.wasmInstance.exports && (this.wasmInstance.exports.chi2_critical_value || this.wasmInstance.exports.chi2_critical_value_export);
    const upperCutoff = critFn ? critFn(this.selectedAlpha / 2.0, 255) : 293.25;
    const lowerCutoff = critFn ? critFn(1.0 - this.selectedAlpha / 2.0, 255) : 218.42;

    const verdictTitle = this.container.querySelector('#eaVerdictTitle').innerText;

    // Status Badges
    const entropyStatus = res.entropy >= 7.9 ? 'PASS' : 'WARN';
    const chiStatus = rawChip < 0.0001 || rawChip > 0.9999 ? 'FAIL' : (rawChip < 0.01 || rawChip > 0.99 ? 'WARN' : 'PASS');
    const meanDiff = Math.abs(res.mean - 127.5);
    const meanStatus = meanDiff >= 5.0 ? 'FAIL' : (meanDiff >= 1.0 ? 'WARN' : 'PASS');
    const piStatus = parseFloat(piErr) >= 3.0 ? 'FAIL' : (parseFloat(piErr) >= 1.0 ? 'WARN' : 'PASS');
    const sccAbs = Math.abs(res.serialCorrelation);
    const sccStatus = sccAbs >= 0.05 ? 'FAIL' : (sccAbs >= 0.01 ? 'WARN' : 'PASS');

    if (format === 'markdown') {
      const meanDiffSign = (res.mean - 127.5) >= 0 ? `+${(res.mean - 127.5).toFixed(4)}` : (res.mean - 127.5).toFixed(4);
      const sccMargin = sccAbs >= 0.05 ? '⚠ High Correlation' : 'Uncorrelated';
      const lowerPct = (this.selectedAlpha / 2.0 * 100).toFixed(1);
      const upperPct = (100 - this.selectedAlpha / 2.0 * 100).toFixed(1);

      const mdContent = `# ENT Randomness Evaluation Report

* **File:** \`${file.name}\` (${this.formatBytes(res.totalBytes)} / ${res.totalBytes.toLocaleString()} bytes)
* **Timestamp:** ${dateStr}
* **SHA-256:** \`${res.sha256 || 'N/A'}\`
* **Overall Verdict:** \`${verdictTitle}\`

---

## Metric Breakdown

| Test Metric | Calculated Value | Ideal Random Val | Deviation / Margin | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Shannon Entropy** | \`${res.entropy.toFixed(6)}\` b/B | 8.000000 bits/byte | Redundancy: ${compressReduction}% | \`${entropyStatus}\` |
| **Chi-Square (\\(\\chi^2\\))** | \`${res.chiSquare.toFixed(2)}\` (df=255) | 255.000000 | Exceedance: ${exceedText} | \`${chiStatus}\` |
| **Arithmetic Mean** | \`${res.mean.toFixed(4)}\` | 127.500000 | Diff: ${meanDiffSign} | \`${meanStatus}\` |
| **Monte Carlo \\(\\pi\\)** | \`${res.monteCarloPi.toFixed(6)}\` | 3.141592654 | Error: ${piErr}% | \`${piStatus}\` |
| **Serial Correlation** | \`${res.serialCorrelation.toFixed(6)}\` | 0.000000 | ${sccMargin} | \`${sccStatus}\` |

---

## Test Interpretation & Guardrails (\\(\\alpha = ${this.selectedAlpha}\\))

> **Note on Chi-Square Interpretation:**  
> A random sequence passes if ${lowerCutoff.toFixed(2)} \\(\\le \\chi^2 \\le\\) ${upperCutoff.toFixed(2)} (${lowerPct}% \\(\\le \\text{Exceedance} \\le\\) ${upperPct}%).  
> * \\(\\chi^2 > ${upperCutoff.toFixed(2)}\\): Non-uniform / biased sequence.
> * \\(\\chi^2 < ${lowerCutoff.toFixed(2)}\\): Artificially forced / overly uniform sequence.

* **Shannon Entropy:** Measures information density. Higher is more random (\\(8.0\\) = completely uncompressible).
* **Arithmetic Mean:** Average byte value. Pure random byte streams center on \\(127.5000\\).
* **Monte Carlo \\(\\pi\\):** Evaluates 2D spatial clustering using 6-byte coordinate pairs. Error \\(< 3.0\\%\\) passes guardrails.
* **Serial Correlation:** Measures sequence memory (\\(x_i\\) vs \\(x_{i+1}\\)). Values near \\(0.0\\) indicate independent bytes.
`;

      const blob = new Blob([mdContent], { type: 'text/markdown;charset=utf-8' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `entropy_report_${file.name.replace(/[^a-zA-Z0-9._-]/g, '_')}.md`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } else {
      // JSON Export Mode
      const reportData = {
        title: "ENT Randomness Evaluation Report",
        timestamp: dateStr,
        file: {
          name: file.name,
          size_bytes: res.totalBytes,
          size_formatted: this.formatBytes(res.totalBytes),
          sha256: res.sha256
        },
        verdict: verdictTitle,
        guardrail_thresholds: {
          significance_alpha: this.selectedAlpha,
          chi_square_ideal_range: [parseFloat(lowerCutoff.toFixed(2)), parseFloat(upperCutoff.toFixed(2))],
          serial_correlation_max_abs: 0.05,
          monte_carlo_pi_max_error_pct: 3.0,
          arithmetic_mean_max_diff: 5.0
        },
        metrics: {
          shannon_entropy_bits_per_byte: parseFloat(res.entropy.toFixed(6)),
          optimum_compression_percent: parseFloat(compressReduction),
          chi_square_statistic: parseFloat(res.chiSquare.toFixed(2)),
          chi_square_degrees_of_freedom: 255,
          chi_square_exceed_probability_percent: exceedText,
          arithmetic_mean: parseFloat(res.mean.toFixed(4)),
          monte_carlo_pi: parseFloat(res.monteCarloPi.toFixed(9)),
          monte_carlo_pi_error_percent: parseFloat(piErr),
          serial_correlation: parseFloat(res.serialCorrelation.toFixed(6))
        },
        status: {
          entropy: entropyStatus,
          chi_square: chiStatus,
          mean: meanStatus,
          pi: piStatus,
          serial_correlation: sccStatus
        }
      };

      const jsonStr = JSON.stringify(reportData, null, 2);
      const blob = new Blob([jsonStr], { type: 'application/json' });
      const url = URL.createObjectURL(blob);

      const a = document.createElement('a');
      a.href = url;
      a.download = `entropy_report_${file.name.replace(/[^a-zA-Z0-9._-]/g, '_')}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    }
  }

  updatePlot() {
    if (!this.wasmInstance || typeof renderDistributionPlot === 'undefined') return;
    const canvas = this.container.querySelector('#eaPlotCanvas');
    const df = 255;
    const stdDev = Math.sqrt(2 * df);
    const xMin = Math.max(0, df - 4 * stdDev);
    const xMax = df + 4 * stdDev;
    const alpha = this.selectedAlpha || 0.05;

    const chi2Pts = getChi2Points(this.wasmInstance, df, xMin, xMax, 350);
    const normPts = getNormalPoints(this.wasmInstance, df, xMin, xMax, 350);

    const critFn = this.wasmInstance.exports && (this.wasmInstance.exports.chi2_critical_value || this.wasmInstance.exports.chi2_critical_value_export);
    const upperCutoff = critFn ? critFn(alpha / 2.0, df) : 293.25;
    const lowerCutoff = critFn ? critFn(1.0 - alpha / 2.0, df) : 218.42;

    renderDistributionPlot(canvas, chi2Pts, normPts, {
      chiSquare: this.currentResult ? this.currentResult.chiSquare : null,
      lowerCutoff: lowerCutoff,
      upperCutoff: upperCutoff,
      alpha: alpha,
      df: df
    });
  }
}

/**
 * Functional Fourmilab ENT Dashboard Renderer.
 * Renders the standardized 6-test ENT evaluation table and metrics.
 */
function renderEntDashboard(container, result, evaluation, options = {}) {
  if (!container || !result) return;
  const isInitial = options.initial === true || result.totalBytes === 0;

  const evalObj = evaluation || {
    overallStatus: 'PASS',
    entropyStatus: 'PASS',
    chiSquareStatus: 'PASS',
    meanStatus: 'PASS',
    piStatus: 'PASS',
    serialCorrelationStatus: 'PASS',
    pochisqExceedProb: 50.0,
    piErrorPercent: 0.0,
  };

  const getBadge = (status) => {
    if (isInitial) {
      return '<span style="background:#0284c722;color:#38bdf8;border:1px solid #38bdf844;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:600;">READY</span>';
    } else if (status === 'PASS') {
      return '<span style="background:#10b981;color:#fff;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:700;">PASS</span>';
    } else if (status === 'WARN') {
      return '<span style="background:#f59e0b;color:#1e293b;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:700;">WARN</span>';
    } else if (status === 'FAIL') {
      return '<span style="background:#f43f5e;color:#fff;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:700;">FAIL</span>';
    } else {
      return '<span style="background:#334155;color:#94a3b8;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:600;">NOT IMPLEMENTED</span>';
    }
  };

  const totalBytesFormatted = (result.totalBytes || 0).toLocaleString();
  const entropyFormatted = (result.entropy != null) ? result.entropy.toFixed(6) : '0.000000';
  const compressFormatted = (result.compressionPercent != null) ? result.compressionPercent.toFixed(2) + '%' : '0.00%';
  const chiFormatted = (result.chiSquare != null) ? result.chiSquare.toFixed(2) : '0.00';
  const exceedFormatted = (evalObj.pochisqExceedProb != null) ? evalObj.pochisqExceedProb.toFixed(2) + '%' : '50.00%';
  const meanFormatted = (result.mean != null) ? result.mean.toFixed(4) : '127.5000';
  const piFormatted = (result.monteCarloPi != null) ? result.monteCarloPi.toFixed(7) : '3.1415927';
  const piErrFormatted = (evalObj.piErrorPercent != null) ? evalObj.piErrorPercent.toFixed(4) + '%' : '0.0000%';
  const scFormatted = (result.serialCorrelation != null) ? result.serialCorrelation.toFixed(6) : '0.000000';

  const testItems = [
    { id: "ENT01", name: "Shannon Entropy", ideal: "8.000000 bits/byte", val: isInitial ? "—" : `${entropyFormatted} bits/byte`, status: evalObj.entropyStatus },
    { id: "ENT02", name: "Optimum Compression", ideal: "0.00% reduction", val: isInitial ? "—" : compressFormatted, status: evalObj.entropyStatus },
    { id: "ENT03", name: "Chi-Square Uniformity (χ²)", ideal: "df = 255 (p = 0.500)", val: isInitial ? "—" : `${chiFormatted} (exceed: ${exceedFormatted})`, status: evalObj.chiSquareStatus },
    { id: "ENT04", name: "Arithmetic Mean", ideal: "127.5000", val: isInitial ? "—" : meanFormatted, status: evalObj.meanStatus },
    { id: "ENT05", name: "Monte Carlo Pi", ideal: "3.14159265", val: isInitial ? "—" : `${piFormatted} (err: ${piErrFormatted})`, status: evalObj.piStatus },
    { id: "ENT06", name: "Serial Correlation (Lag-1)", ideal: "0.000000", val: isInitial ? "—" : scFormatted, status: evalObj.serialCorrelationStatus },
  ];

  const rows = testItems.map(r => `
    <tr style="border-bottom:1px solid #1e293b;">
      <td style="padding:10px;color:#94a3b8;font-family:monospace;">${r.id}</td>
      <td style="padding:10px;font-weight:600;color:#f8fafc;">${r.name}</td>
      <td style="padding:10px;color:#cbd5e1;font-size:12px;font-family:monospace;">${r.ideal}</td>
      <td style="padding:10px;color:#38bdf8;font-family:monospace;">${r.val}</td>
      <td style="padding:10px;">${getBadge(r.status)}</td>
    </tr>
  `).join('');

  const passedCount = testItems.filter(r => r.status === 'PASS').length;

  const statusSummary = isInitial
    ? `Active: <strong style="color:#38bdf8;">6 / 6</strong> | Status: <strong style="color:#38bdf8;">Ready for test input</strong>`
    : `Active: <strong style="color:#38bdf8;">6 / 6</strong> | Passed: <strong style="color:#10b981;">${passedCount}</strong> | Overall: <strong style="color:${evalObj.overallStatus === 'PASS' ? '#10b981' : '#f59e0b'};">${evalObj.overallStatus}</strong>`;

  container.innerHTML = `
    <div class="randstat-ent-card" style="font-family:system-ui,-apple-system,sans-serif;background:#0f172a;color:#f8fafc;padding:24px;border-radius:12px;border:1px solid #1e293b;">
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px;flex-wrap:wrap;gap:12px;">
        <h3 style="margin:0;font-size:18px;font-weight:700;color:#38bdf8;">Fourmilab ENT Randomness Battery</h3>
        <div style="font-size:12px;color:#94a3b8;">
          ${statusSummary}
        </div>
      </div>

      <div style="overflow-x:auto;">
        <table style="width:100%;border-collapse:collapse;text-align:left;font-size:13px;">
          <thead>
            <tr style="border-bottom:2px solid #334155;color:#94a3b8;">
              <th style="padding:10px;">ID</th>
              <th style="padding:10px;">Metric Name</th>
              <th style="padding:10px;">Theoretical Ideal</th>
              <th style="padding:10px;">Evaluated Value</th>
              <th style="padding:10px;">Status</th>
            </tr>
          </thead>
          <tbody>
            ${rows}
          </tbody>
        </table>
      </div>
    </div>
  `;
}

// Custom Element definition for <entropy-analyzer>
if (typeof customElements !== 'undefined' && !customElements.get('entropy-analyzer')) {
  customElements.define('entropy-analyzer', class extends HTMLElement {
    connectedCallback() {
      const wasmUrl = this.getAttribute('wasm-url') || 'randstat-ent.wasm';
      this.analyzer = new EntropyAnalyzer(this, { wasmUrl });
    }
  });
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    ...module.exports,
    EntropyAnalyzer,
    renderEntDashboard,
    EntRunner: typeof EntRunner !== 'undefined' ? EntRunner : module.exports.EntRunner,
  };
}
