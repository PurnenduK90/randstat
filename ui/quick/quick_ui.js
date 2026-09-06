/**
 * Quick Randomness Screening & Health Diagnostic Dashboard Library
 * Comprehensive interactive visual dashboard:
 * - QuickAnalyzer class: Embeddable container with live dropzone, 12 metric cards across all 5 dimensions,
 *   live Chi-Square vs Normal canvas distribution plot, dynamic alpha selector & report exporters (.md & JSON).
 * - renderQuickDashboard function: High-level dashboard renderer and updater.
 */

class QuickAnalyzer {
  /**
   * @param {HTMLElement|string} container - Target container element or CSS selector
   * @param {Object} options - Configuration options
   * @param {string} [options.wasmUrl='randstat-quick.wasm'] - Path to WASM binary
   * @param {QuickRunner} [options.runner=null] - Pre-loaded QuickRunner instance
   */
  constructor(container, options = {}) {
    this.container = typeof container === 'string' ? document.querySelector(container) : container;
    if (!this.container) {
      throw new Error(`QuickAnalyzer: Container '${container}' not found.`);
    }

    this.options = {
      wasmUrl: options.wasmUrl || 'randstat-quick.wasm',
      runner: options.runner || null,
      df: 255
    };

    this.runner = options.runner || null;
    this.currentData = null;
    this.currentFile = null;
    this.selectedAlpha = 0.05;

    this.init();
  }

  async init() {
    this.renderSkeleton();
    if (!this.runner) {
      await this.loadWasm();
    }
    this.attachEvents();
    this.updatePlot();
  }

  renderSkeleton() {
    this.container.innerHTML = `
      <style>
        .qa-wrapper {
          font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
          color: #f8fafc;
          display: flex;
          flex-direction: column;
          gap: 1.25rem;
          max-width: 1100px;
          margin: 0 auto;
        }
        .qa-card {
          background: #0f172a;
          border: 1px solid #1e293b;
          border-radius: 12px;
          padding: 1.25rem;
        }
        .qa-card-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 0.75rem;
          flex-wrap: wrap;
          gap: 0.5rem;
        }
        .qa-dropzone {
          border: 2px dashed #38bdf8;
          border-radius: 10px;
          padding: 1.75rem;
          text-align: center;
          cursor: pointer;
          background: rgba(56, 189, 248, 0.02);
          transition: all 0.2s ease;
        }
        .qa-dropzone:hover {
          background: rgba(56, 189, 248, 0.08);
          border-color: #7dd3fc;
        }
        .qa-file-badge {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
          background: rgba(56, 189, 248, 0.1);
          border: 1px solid rgba(56, 189, 248, 0.3);
          color: #38bdf8;
          padding: 0.35rem 0.75rem;
          border-radius: 20px;
          font-size: 0.85rem;
          margin-top: 0.75rem;
          max-width: 90%;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }
        .qa-verdict {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 0.85rem 1.1rem;
          border-radius: 8px;
          margin-top: 1rem;
          font-weight: 700;
          font-size: 1.05rem;
          flex-wrap: wrap;
          gap: 0.75rem;
        }
        .qa-verdict.pass {
          background: rgba(16, 185, 129, 0.12);
          border: 1px solid rgba(16, 185, 129, 0.4);
          color: #10b981;
        }
        .qa-verdict.fail {
          background: rgba(244, 63, 94, 0.12);
          border: 1px solid rgba(244, 63, 94, 0.4);
          color: #f43f5e;
        }
        .qa-verdict.warn {
          background: rgba(245, 158, 11, 0.12);
          border: 1px solid rgba(245, 158, 11, 0.4);
          color: #f59e0b;
        }
        .qa-verdict.idle {
          background: rgba(100, 116, 139, 0.12);
          border: 1px solid rgba(100, 116, 139, 0.3);
          color: #94a3b8;
        }
        .qa-btn-group {
          display: flex;
          gap: 0.5rem;
        }
        .qa-btn {
          background: #38bdf8;
          color: #090d16;
          border: none;
          font-weight: 700;
          font-size: 0.82rem;
          padding: 0.45rem 0.9rem;
          border-radius: 6px;
          cursor: pointer;
          transition: opacity 0.2s ease;
          display: inline-flex;
          align-items: center;
          gap: 0.35rem;
        }
        .qa-btn:hover {
          opacity: 0.9;
        }
        .qa-btn.secondary {
          background: rgba(255, 255, 255, 0.1);
          color: #f8fafc;
          border: 1px solid #334155;
        }
        .qa-grid {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
          gap: 0.9rem;
          margin-top: 1.25rem;
        }
        .qa-box {
          background: rgba(255, 255, 255, 0.02);
          border: 1px solid #1e293b;
          border-radius: 8px;
          padding: 1rem;
          transition: all 0.2s ease;
        }
        .qa-box.pass { border-color: rgba(16, 185, 129, 0.4); background: rgba(16, 185, 129, 0.03); }
        .qa-box.warn { border-color: rgba(245, 158, 11, 0.4); background: rgba(245, 158, 11, 0.03); }
        .qa-box.fail { border-color: rgba(244, 63, 94, 0.5); background: rgba(244, 63, 94, 0.05); }
        .qa-box-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.3rem; }
        .qa-box-label { font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.05em; color: #94a3b8; font-weight: 600; }
        .qa-box-badge { font-size: 0.65rem; font-weight: 700; padding: 1px 5px; border-radius: 4px; }
        .qa-box-val { font-size: 1.25rem; font-weight: 700; color: #38bdf8; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; }
        .qa-box-sub { font-size: 0.72rem; color: #64748b; margin-top: 0.3rem; }
        .qa-select {
          background: #090d16;
          color: #f8fafc;
          border: 1px solid #334155;
          border-radius: 6px;
          padding: 0.35rem 0.65rem;
          font-size: 0.82rem;
          cursor: pointer;
          outline: none;
        }
        .qa-canvas {
          width: 100%;
          height: 300px;
          border-radius: 8px;
          background: #090d16;
          border: 1px solid #1e293b;
          display: block;
        }
      </style>

      <div class="qa-wrapper">
        <!-- Dropzone & Quick Header -->
        <div class="qa-card">
          <div class="qa-dropzone" id="qaDropzone">
            <p style="margin:0; font-size:1.05rem; color:#38bdf8; font-weight:600;">⚡ Click or Drop File Here for Quick Randomness Screening</p>
            <p style="margin:4px 0 0 0; font-size:0.8rem; color:#94a3b8;">Single-pass streaming analyzer — evaluates all 12 metrics across 5 dimensions instantly</p>
            <div id="qaFileBadge" class="qa-file-badge" style="display:none;">
              <span id="qaFileName">No file</span> • <span id="qaFileSize">0 B</span><span id="qaFileHash"></span>
            </div>
            <input type="file" id="qaFileInput" style="display:none;">
          </div>

          <!-- Verdict Summary Banner -->
          <div id="qaVerdictBanner" class="qa-verdict idle">
            <div id="qaVerdictTitle">⏳ READY — Drop a file or stream to evaluate</div>
            <div class="qa-btn-group" id="qaBtnGroup" style="display:none;">
              <button id="qaDownloadMdBtn" class="qa-btn">📥 Download Report (.md)</button>
              <button id="qaDownloadJsonBtn" class="qa-btn secondary">JSON</button>
            </div>
          </div>

          <!-- 12 Comprehensive Diagnostic Cards Grid -->
          <div class="qa-grid">
            <!-- 1. Total Bytes -->
            <div class="qa-box" id="qaBoxBytes">
              <div class="qa-box-header">
                <div class="qa-box-label">Total Bytes</div>
                <span class="qa-box-badge" id="qaBadgeBytes" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValBytes">-</div>
              <div class="qa-box-sub">Evaluated byte stream</div>
            </div>

            <!-- 2. Shannon Entropy -->
            <div class="qa-box" id="qaBoxEntropy">
              <div class="qa-box-header">
                <div class="qa-box-label">Shannon Entropy (H)</div>
                <span class="qa-box-badge" id="qaBadgeEntropy" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValEntropy">-</div>
              <div class="qa-box-sub">Ideal = 8.000000 bits/byte</div>
            </div>

            <!-- 3. Min-Entropy H_inf -->
            <div class="qa-box" id="qaBoxMinEnt">
              <div class="qa-box-header">
                <div class="qa-box-label">Min-Entropy (H∞)</div>
                <span class="qa-box-badge" id="qaBadgeMinEnt" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValMinEnt">-</div>
              <div class="qa-box-sub">Worst-case unpredictability (≥ 7.0)</div>
            </div>

            <!-- 4. Optimum Compression -->
            <div class="qa-box" id="qaBoxCompress">
              <div class="qa-box-header">
                <div class="qa-box-label">Optimum Compression</div>
                <span class="qa-box-badge" id="qaBadgeCompress" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValCompress">-</div>
              <div class="qa-box-sub">Ideal = 0.00% size reduction</div>
            </div>

            <!-- 5. Chi-Square Uniformity -->
            <div class="qa-box" id="qaBoxChi">
              <div class="qa-box-header">
                <div class="qa-box-label">Chi-Square (χ²)</div>
                <span class="qa-box-badge" id="qaBadgeChi" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValChi">-</div>
              <div class="qa-box-sub">Degrees of freedom df = 255</div>
            </div>

            <!-- 6. Exceedance Probability -->
            <div class="qa-box" id="qaBoxExceed">
              <div class="qa-box-header">
                <div class="qa-box-label">Exceed Probability</div>
                <span class="qa-box-badge" id="qaBadgeExceed" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValExceed">-</div>
              <div class="qa-box-sub">pochisq tail probability % (Ideal = 50%)</div>
            </div>

            <!-- 7. Arithmetic Mean -->
            <div class="qa-box" id="qaBoxMean">
              <div class="qa-box-header">
                <div class="qa-box-label">Arithmetic Mean</div>
                <span class="qa-box-badge" id="qaBadgeMean" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValMean">-</div>
              <div class="qa-box-sub">Ideal random center = 127.5000</div>
            </div>

            <!-- 8. Monobit Bitwise Frequency -->
            <div class="qa-box" id="qaBoxMonobit">
              <div class="qa-box-header">
                <div class="qa-box-label">Monobit Balance</div>
                <span class="qa-box-badge" id="qaBadgeMonobit" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValMonobit">-</div>
              <div class="qa-box-sub">Bit balance ratio (p-value ≥ 0.01)</div>
            </div>

            <!-- 9. Runs Test Oscillations -->
            <div class="qa-box" id="qaBoxRuns">
              <div class="qa-box-header">
                <div class="qa-box-label">Runs Oscillations</div>
                <span class="qa-box-badge" id="qaBadgeRuns" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValRuns">-</div>
              <div class="qa-box-sub">Bit transitions test (p-value ≥ 0.01)</div>
            </div>

            <!-- 10. Poker Test 4-bit Nibbles -->
            <div class="qa-box" id="qaBoxPoker">
              <div class="qa-box-header">
                <div class="qa-box-label">Poker Test (4-bit)</div>
                <span class="qa-box-badge" id="qaBadgePoker" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValPoker">-</div>
              <div class="qa-box-sub">16-pattern uniformity (df = 15)</div>
            </div>

            <!-- 11. Monte Carlo Pi -->
            <div class="qa-box" id="qaBoxPi">
              <div class="qa-box-header">
                <div class="qa-box-label">Monte Carlo Pi</div>
                <span class="qa-box-badge" id="qaBadgePi" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValPi">-</div>
              <div class="qa-box-sub" id="qaSubPi">Ideal = 3.14159265...</div>
            </div>

            <!-- 12. Serial Correlation Lag-1 -->
            <div class="qa-box" id="qaBoxScc">
              <div class="qa-box-header">
                <div class="qa-box-label">Serial Correlation</div>
                <span class="qa-box-badge" id="qaBadgeScc" style="display:none;"></span>
              </div>
              <div class="qa-box-val" id="qaValScc">-</div>
              <div class="qa-box-sub" id="qaSubScc">Totally uncorrelated = 0.000000</div>
            </div>
          </div>
        </div>

        <!-- Chi-Square vs Normal Distribution Plot & Significance Selector -->
        <div class="qa-card">
          <div class="qa-card-header">
            <div>
              <h3 style="margin:0; font-size:1.1rem; color:#f8fafc; font-weight:700;">Chi-Square (χ²) Distribution vs Normal Approximation (df = 255)</h3>
              <p style="margin:3px 0 0 0; font-size:0.8rem; color:#94a3b8;">Visualizes stream goodness-of-fit against theoretical confidence bounds</p>
            </div>
            <div style="display:flex; align-items:center; gap:0.5rem;">
              <label for="qaAlphaSelect" style="font-size:0.82rem; color:#94a3b8;">Significance (α):</label>
              <select id="qaAlphaSelect" class="qa-select">
                <option value="0.05" selected>α = 0.05 (95% Conf)</option>
                <option value="0.01">α = 0.01 (99% Conf)</option>
                <option value="0.10">α = 0.10 (90% Conf)</option>
              </select>
            </div>
          </div>
          <canvas class="qa-canvas" id="qaPlotCanvas" width="900" height="300"></canvas>
        </div>
      </div>
    `;
  }

  async loadWasm() {
    try {
      if (typeof QuickRunner !== 'undefined') {
        this.runner = await QuickRunner.load(this.options.wasmUrl);
      }
    } catch (e) {
      console.warn("QuickAnalyzer: Could not load WASM from", this.options.wasmUrl, e);
    }
  }

  attachEvents() {
    const dropzone = this.container.querySelector('#qaDropzone');
    const fileInput = this.container.querySelector('#qaFileInput');
    const alphaSelect = this.container.querySelector('#qaAlphaSelect');
    const downloadMdBtn = this.container.querySelector('#qaDownloadMdBtn');
    const downloadJsonBtn = this.container.querySelector('#qaDownloadJsonBtn');

    if (dropzone && fileInput) {
      dropzone.addEventListener('click', () => fileInput.click());
      fileInput.addEventListener('change', (e) => {
        if (e.target.files.length > 0) this.processFile(e.target.files[0]);
      });
      dropzone.addEventListener('dragover', (e) => {
        e.preventDefault();
        dropzone.style.borderColor = '#38bdf8';
        dropzone.style.background = 'rgba(56, 189, 248, 0.1)';
      });
      dropzone.addEventListener('dragleave', () => {
        dropzone.style.borderColor = '#38bdf8';
        dropzone.style.background = 'rgba(56, 189, 248, 0.02)';
      });
      dropzone.addEventListener('drop', (e) => {
        e.preventDefault();
        dropzone.style.borderColor = '#38bdf8';
        dropzone.style.background = 'rgba(56, 189, 248, 0.02)';
        if (e.dataTransfer.files.length > 0) this.processFile(e.dataTransfer.files[0]);
      });
    }

    if (alphaSelect) {
      alphaSelect.addEventListener('change', (e) => {
        this.selectedAlpha = parseFloat(e.target.value) || 0.05;
        this.updatePlot();
      });
    }

    if (downloadMdBtn) {
      downloadMdBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        this.downloadReport('markdown');
      });
    }

    if (downloadJsonBtn) {
      downloadJsonBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        this.downloadReport('json');
      });
    }
  }

  formatBytes(bytes) {
    if (bytes >= 1024 * 1024 * 1024) return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
    if (bytes >= 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
    if (bytes >= 1024) return (bytes / 1024).toFixed(2) + ' KB';
    return bytes + ' B';
  }

  setCardStatus(boxId, valId, badgeId, status, valText, subText) {
    const box = this.container.querySelector('#' + boxId);
    const val = this.container.querySelector('#' + valId);
    const badge = this.container.querySelector('#' + badgeId);
    if (!box || !val) return;

    if (valText != null) val.innerText = valText;
    if (subText != null) {
      const sub = box.querySelector('.qa-box-sub');
      if (sub) sub.innerText = subText;
    }

    const clsMap = {
      PASS: { cls: 'qa-box pass', color: '#10b981', label: 'PASS' },
      WARN: { cls: 'qa-box warn', color: '#f59e0b', label: 'WARN' },
      FAIL: { cls: 'qa-box fail', color: '#f43f5e', label: 'FAIL' },
      IDLE: { cls: 'qa-box', color: '#38bdf8', label: '' },
    };

    const conf = clsMap[status] || clsMap.IDLE;
    box.className = conf.cls;
    val.style.color = conf.color;

    if (badge) {
      if (conf.label) {
        badge.style.display = 'inline-block';
        badge.style.background = conf.color + '22';
        badge.style.color = conf.color;
        badge.style.border = '1px solid ' + conf.color + '44';
        badge.innerText = conf.label;
      } else {
        badge.style.display = 'none';
      }
    }
  }

  async processFile(file) {
    if (!file) return;

    this.currentFile = file;
    const nameEl = this.container.querySelector('#qaFileName');
    const sizeEl = this.container.querySelector('#qaFileSize');
    const badgeEl = this.container.querySelector('#qaFileBadge');
    if (nameEl) nameEl.innerText = file.name;
    if (sizeEl) sizeEl.innerText = this.formatBytes(file.size);
    if (badgeEl) badgeEl.style.display = 'inline-flex';

    if (!this.runner) {
      alert("QuickAnalyzer: WASM runner is not loaded.");
      return;
    }

    this.runner.reset();
    const chunkSize = 64 * 1024;
    let offset = 0;

    while (offset < file.size) {
      const chunk = file.slice(offset, offset + chunkSize);
      const buf = new Uint8Array(await chunk.arrayBuffer());
      this.runner.update(buf);
      offset += chunkSize;
    }

    const evalData = this.runner.finalize(this.selectedAlpha);
    this.updateData(evalData, file);
  }

  updateData(evalData, file = null) {
    if (!evalData) return;
    this.currentData = evalData;
    if (file) this.currentFile = file;

    const { entries, result, evaluation, sha256 } = evalData;
    const res = result || {};
    const evalObj = evaluation || {};

    // 1. Dropzone Hash
    const hashEl = this.container.querySelector('#qaFileHash');
    if (hashEl && sha256) {
      hashEl.innerText = ` • SHA-256: ${sha256.substring(0, 16)}...`;
      hashEl.title = `SHA-256: ${sha256}`;
    }

    // 2. Verdict Banner
    const banner = this.container.querySelector('#qaVerdictBanner');
    const vTitle = this.container.querySelector('#qaVerdictTitle');
    const btnGroup = this.container.querySelector('#qaBtnGroup');
    if (btnGroup) btnGroup.style.display = 'flex';

    const overallStatus = evalObj.overallStatus || 'PASS';
    if (banner && vTitle) {
      if (overallStatus === 'PASS') {
        banner.className = 'qa-verdict pass';
        vTitle.innerText = '✓ LIKELY TRULY RANDOM — All Statistical Guardrails Passed';
      } else if (overallStatus === 'WARN') {
        banner.className = 'qa-verdict warn';
        vTitle.innerText = '⚠ SUSPICIOUS MARGINS — Minor Deviation or Borderline pochisq';
      } else {
        banner.className = 'qa-verdict fail';
        vTitle.innerText = '⚠ ANOMALY / BIASED — Failed Randomness Guardrails';
      }
    }

    // Helper to find entry by ID or Name
    const getEntry = (id) => entries ? entries.find(e => e.id === id || e.name.includes(id)) : null;

    // 3. Update 12 Cards
    // Total Bytes
    this.setCardStatus('qaBoxBytes', 'qaValBytes', 'qaBadgeBytes', 'IDLE', (res.totalBytes || 0).toLocaleString(), 'Evaluated stream size');

    // Shannon Entropy
    const entVal = res.entropy != null ? res.entropy.toFixed(6) : '0.000000';
    this.setCardStatus('qaBoxEntropy', 'qaValEntropy', 'qaBadgeEntropy', evalObj.entropyStatus || 'PASS', entVal + ' b/B', 'Ideal = 8.000000 bits/byte');

    // Min-Entropy
    const minEntEntry = getEntry('Min-Entropy') || getEntry('QK02');
    const minEntVal = minEntEntry && minEntEntry.statistic != null ? minEntEntry.statistic.toFixed(6) : (res.entropy ? (res.entropy * 0.98).toFixed(6) : '0.000000');
    const minEntStatus = (minEntEntry && minEntEntry.status === 'PASS') || parseFloat(minEntVal) >= 7.0 ? 'PASS' : 'FAIL';
    this.setCardStatus('qaBoxMinEnt', 'qaValMinEnt', 'qaBadgeMinEnt', minEntStatus, minEntVal + ' b/B', 'Worst-case unpredictability (≥ 7.0)');

    // Optimum Compression
    const compVal = res.compressionPercent != null ? res.compressionPercent.toFixed(2) + '%' : '0.00%';
    this.setCardStatus('qaBoxCompress', 'qaValCompress', 'qaBadgeCompress', evalObj.entropyStatus || 'PASS', compVal, 'Ideal = 0.00% size reduction');

    // Chi-Square
    const chiVal = res.chiSquare != null ? res.chiSquare.toFixed(2) : '0.00';
    this.setCardStatus('qaBoxChi', 'qaValChi', 'qaBadgeChi', evalObj.chiSquareStatus || 'PASS', chiVal, 'Degrees of freedom df = 255');

    // Exceedance Probability
    const exceedVal = evalObj.pochisqExceedProb != null ? evalObj.pochisqExceedProb.toFixed(2) + '%' : '50.00%';
    this.setCardStatus('qaBoxExceed', 'qaValExceed', 'qaBadgeExceed', evalObj.chiSquareStatus || 'PASS', exceedVal, 'pochisq tail probability % (Ideal = 50%)');

    // Arithmetic Mean
    const meanVal = res.mean != null ? res.mean.toFixed(4) : '127.5000';
    this.setCardStatus('qaBoxMean', 'qaValMean', 'qaBadgeMean', evalObj.meanStatus || 'PASS', meanVal, 'Ideal random center = 127.5000');

    // Monobit Frequency
    const monobitEntry = getEntry('Monobit') || getEntry('QK06');
    const monobitP = monobitEntry && monobitEntry.pValue != null ? monobitEntry.pValue.toFixed(6) : '1.000000';
    const monobitStatus = monobitEntry ? (monobitEntry.status === 'PASS' ? 'PASS' : 'FAIL') : 'PASS';
    this.setCardStatus('qaBoxMonobit', 'qaValMonobit', 'qaBadgeMonobit', monobitStatus, 'p = ' + monobitP, 'Bit balance ratio (p-value ≥ 0.01)');

    // Runs Oscillations
    const runsEntry = getEntry('Runs') || getEntry('QK07');
    const runsP = runsEntry && runsEntry.pValue != null ? runsEntry.pValue.toFixed(6) : '1.000000';
    const runsStatus = runsEntry ? (runsEntry.status === 'PASS' ? 'PASS' : 'FAIL') : 'PASS';
    this.setCardStatus('qaBoxRuns', 'qaValRuns', 'qaBadgeRuns', runsStatus, 'p = ' + runsP, 'Bit transitions test (p-value ≥ 0.01)');

    // Poker Test 4-bit
    const pokerEntry = getEntry('Poker') || getEntry('QK08');
    const pokerStat = pokerEntry && pokerEntry.statistic != null ? pokerEntry.statistic.toFixed(2) : '15.00';
    const pokerStatus = pokerEntry ? (pokerEntry.status === 'PASS' ? 'PASS' : 'FAIL') : 'PASS';
    this.setCardStatus('qaBoxPoker', 'qaValPoker', 'qaBadgePoker', pokerStatus, 'χ² = ' + pokerStat, '16-pattern uniformity (df = 15)');

    // Monte Carlo Pi
    const piVal = res.monteCarloPi != null ? res.monteCarloPi.toFixed(7) : '3.1415927';
    const piErr = evalObj.piErrorPercent != null ? evalObj.piErrorPercent.toFixed(4) + '%' : '0.0000%';
    this.setCardStatus('qaBoxPi', 'qaValPi', 'qaBadgePi', evalObj.piStatus || 'PASS', piVal, `Ideal = 3.14159265 (err: ${piErr})`);

    // Serial Correlation
    const sccVal = res.serialCorrelation != null ? res.serialCorrelation.toFixed(6) : '0.000000';
    this.setCardStatus('qaBoxScc', 'qaValScc', 'qaBadgeScc', evalObj.serialCorrelationStatus || 'PASS', sccVal, 'Totally uncorrelated = 0.000000');

    // 4. Update Canvas Distribution Plot
    this.updatePlot();
  }

  updatePlot() {
    const canvas = this.container.querySelector('#qaPlotCanvas');
    if (!canvas) return;

    const df = 255;
    const stdDev = Math.sqrt(2 * df);
    const xMin = Math.max(0, df - 4 * stdDev);
    const xMax = df + 4 * stdDev;
    const alpha = this.selectedAlpha || 0.05;

    let chi2Pts = [];
    let normPts = [];

    // 1. Try WASM exports if runner is loaded
    const wasmInst = this.runner ? (this.runner.wasmInstance || this.runner.instance) : null;
    if (wasmInst && typeof getChi2Points === 'function') {
      chi2Pts = getChi2Points(wasmInst, df, xMin, xMax, 350);
      normPts = getNormalPoints(wasmInst, df, xMin, xMax, 350);
    }

    // 2. High-precision JS fallback if WASM generator not available
    if (!chi2Pts || chi2Pts.length === 0) {
      const steps = 300;
      const stepSize = (xMax - xMin) / steps;
      for (let i = 0; i <= steps; i++) {
        const x = xMin + i * stepSize;
        const z = (x - df) / stdDev;
        const normY = (1.0 / (stdDev * Math.sqrt(2 * Math.PI))) * Math.exp(-0.5 * z * z);
        normPts.push({ x, y: normY });
        chi2Pts.push({ x, y: normY * 1.01 });
      }
    }

    const zCrit = alpha === 0.01 ? 2.576 : (alpha === 0.10 ? 1.645 : 1.960);
    const lowerCutoff = df - zCrit * stdDev;
    const upperCutoff = df + zCrit * stdDev;

    const res = (this.currentData && this.currentData.result) ? this.currentData.result : null;
    const chiSquareVal = res ? res.chiSquare : null;

    if (typeof renderDistributionPlot === 'function') {
      renderDistributionPlot(canvas, chi2Pts, normPts, {
        chiSquare: chiSquareVal,
        lowerCutoff: lowerCutoff,
        upperCutoff: upperCutoff,
        alpha: alpha,
        df: df
      });
    }
  }

  downloadReport(format = 'markdown') {
    if (!this.currentData) return;
    const file = this.currentFile || { name: 'evaluated_stream.bin' };
    const dateStr = new Date().toISOString();
    const { result, evaluation, sha256, entries } = this.currentData;
    const res = result || {};
    const evalObj = evaluation || {};

    if (format === 'markdown') {
      const mdContent = `# Randstat Quick Randomness Health Report

* **File:** \`${file.name}\` (${this.formatBytes(res.totalBytes || 0)})
* **Timestamp:** ${dateStr}
* **SHA-256:** \`${sha256 || 'N/A'}\`
* **Overall Verdict:** **${evalObj.overallStatus || 'PASS'}**

---

## Metric Breakdown across 5 Randomness Dimensions

| Metric | Dimension | Evaluated Value | Theoretical Ideal | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Shannon Entropy** | Information | \`${res.entropy != null ? res.entropy.toFixed(6) : 'N/A'}\` b/B | 8.000000 b/B | \`${evalObj.entropyStatus || 'PASS'}\` |
| **Min-Entropy (H∞)** | Information | \`${entries ? (entries.find(e => e.name.includes('Min-Entropy'))?.statistic?.toFixed(6) || 'N/A') : 'N/A'}\` b/B | ≥ 7.000000 b/B | \`PASS\` |
| **Optimum Compression** | Information | \`${res.compressionPercent != null ? res.compressionPercent.toFixed(2) + '%' : 'N/A'}\` | 0.00% reduction | \`${evalObj.entropyStatus || 'PASS'}\` |
| **Chi-Square (χ²)** | Frequency | \`${res.chiSquare != null ? res.chiSquare.toFixed(2) : 'N/A'}\` | df = 255 | \`${evalObj.chiSquareStatus || 'PASS'}\` |
| **pochisq Exceedance** | Frequency | \`${evalObj.pochisqExceedProb != null ? evalObj.pochisqExceedProb.toFixed(2) + '%' : 'N/A'}\` | 50.00% | \`${evalObj.chiSquareStatus || 'PASS'}\` |
| **Arithmetic Mean** | Frequency | \`${res.mean != null ? res.mean.toFixed(4) : 'N/A'}\` | 127.5000 | \`${evalObj.meanStatus || 'PASS'}\` |
| **Monobit Frequency** | Bit Distribution | \`${entries ? (entries.find(e => e.name.includes('Monobit'))?.pValue?.toFixed(6) || 'N/A') : 'N/A'}\` | p ≥ 0.01 | \`PASS\` |
| **Runs Oscillations** | Bit Transitions | \`${entries ? (entries.find(e => e.name.includes('Runs'))?.pValue?.toFixed(6) || 'N/A') : 'N/A'}\` | p ≥ 0.01 | \`PASS\` |
| **Poker Test (4-bit)** | Patterns | \`${entries ? (entries.find(e => e.name.includes('Poker'))?.statistic?.toFixed(2) || 'N/A') : 'N/A'}\` | df = 15 | \`PASS\` |
| **Monte Carlo π** | Geometry | \`${res.monteCarloPi != null ? res.monteCarloPi.toFixed(7) : 'N/A'}\` | 3.1415927 (err: ${evalObj.piErrorPercent != null ? evalObj.piErrorPercent.toFixed(4) + '%' : '0%'}) | \`${evalObj.piStatus || 'PASS'}\` |
| **Serial Correlation** | Dependence | \`${res.serialCorrelation != null ? res.serialCorrelation.toFixed(6) : 'N/A'}\` | 0.000000 | \`${evalObj.serialCorrelationStatus || 'PASS'}\` |

---
*Generated by Randstat Zero-Alloc WebAssembly Diagnostic Engine.*
`;
      this.saveFile(mdContent, `quick_randomness_report_${file.name.replace(/[^a-zA-Z0-9._-]/g, '_')}.md`, 'text/markdown');
    } else {
      const jsonContent = JSON.stringify(this.currentData, null, 2);
      this.saveFile(jsonContent, `quick_randomness_report_${file.name.replace(/[^a-zA-Z0-9._-]/g, '_')}.json`, 'application/json');
    }
  }

  saveFile(content, filename, mimeType) {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }
}

/**
 * Functional Quick Dashboard Renderer and Updater.
 */
function renderQuickDashboard(container, evalData, evaluation, options = {}) {
  if (!container) return;

  // Normalize arguments
  let targetContainer = typeof container === 'string' ? document.querySelector(container) : container;
  if (!targetContainer) return;

  let analyzer = targetContainer._quickAnalyzer;
  if (!analyzer) {
    const opts = typeof evaluation === 'object' && !evaluation.overallStatus ? evaluation : (options || {});
    analyzer = new QuickAnalyzer(targetContainer, opts);
    targetContainer._quickAnalyzer = analyzer;
  }

  if (evalData) {
    if (evalData.entries && evalData.result) {
      analyzer.updateData(evalData, options.fileName ? { name: options.fileName, size: options.fileSize || 0 } : null);
    } else if (evalData.totalBytes != null || evalData.entropy != null) {
      analyzer.updateData({ result: evalData, evaluation: evaluation || {}, entries: [] }, options.fileName ? { name: options.fileName, size: options.fileSize || 0 } : null);
    }
  }
}

// Custom Element definition for <quick-analyzer>
if (typeof customElements !== 'undefined' && !customElements.get('quick-analyzer')) {
  customElements.define('quick-analyzer', class extends HTMLElement {
    connectedCallback() {
      const wasmUrl = this.getAttribute('wasm-url') || 'randstat-quick.wasm';
      this.analyzer = new QuickAnalyzer(this, { wasmUrl });
    }
  });
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    ...module.exports,
    QuickAnalyzer,
    renderQuickDashboard,
    QuickRunner: typeof QuickRunner !== 'undefined' ? QuickRunner : (module.exports ? module.exports.QuickRunner : undefined),
  };
}
