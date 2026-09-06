/**
 * Interactive UI widget for Randstat Test Stream Generators.
 */
function renderGeneratorUI(container, runner, onRunSuite = null) {
  if (!container) return;

  const generatorOptions = [
    { id: 0, name: 'Sequential Ramp', desc: '(start + i * step) % 256' },
    { id: 1, name: 'Fixed Constant', desc: 'All zeros (0x00), ones (0xFF), or byte K' },
    { id: 2, name: 'LFSR (Shift Register)', desc: 'Orders 3 to 128 with primitive polynomials' },
    { id: 3, name: 'De Bruijn Cycle', desc: 'Full 2^n cycle with equal bit balance' },
    { id: 4, name: 'LCG (Linear Congruential)', desc: 'ANSI C rand(), MINSTD, Knuth 64' },
    { id: 5, name: 'Xoshiro256**', desc: 'Modern high-speed simulation PRNG' },
    { id: 6, name: 'Gaussian (Normal)', desc: 'Box-Muller continuous distribution' },
    { id: 7, name: 'Poisson (λ=127)', desc: 'Discrete Poisson distribution' },
    { id: 8, name: 'NIST SP 800-90A AES CTR_DRBG', desc: 'AES-128 / AES-256 counter mode' },
    { id: 9, name: 'NIST SP 800-90A SHA-256 Hash_DRBG', desc: 'SHA-256 hash based stream' },
    { id: 10, name: 'ChaCha20 Keystream', desc: 'RFC 8439 256-bit stream cipher' },
  ];

  const formatOptions = [
    { id: 0, label: 'Raw Binary (.bin)', ext: 'bin' },
    { id: 1, label: 'ASCII Integers (.txt)', ext: 'txt' },
    { id: 2, label: 'ASCII Bits (.bits)', ext: 'bits' },
    { id: 3, label: 'Hexadecimal (.hex)', ext: 'hex' },
    { id: 4, label: 'Normalized Float (.csv)', ext: 'csv' },
  ];

  const sizePresets = [
    { label: '1 kB', bytes: 1024 },
    { label: '64 kB', bytes: 65536 },
    { label: '1 MB', bytes: 1048576 },
    { label: '10 MB', bytes: 10485760 },
    { label: '50 MB', bytes: 52428800 },
    { label: '100 MB', bytes: 104857600 },
  ];

  container.innerHTML = `
    <div class="randstat-gen-card" style="border: 1px solid var(--border-color, #e2e8f0); border-radius: 8px; padding: 20px; background: var(--card-bg, #ffffff); font-family: system-ui, -apple-system, sans-serif;">
      <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #e2e8f0; padding-bottom: 12px; margin-bottom: 16px;">
        <h3 style="margin: 0; font-size: 1.15rem; color: var(--text-color, #1e293b); display: flex; align-items: center; gap: 8px;">
          <span>⚡</span> Randstat Test Stream Generator
        </h3>
        <span style="font-size: 0.8rem; background: #ecfdf5; color: #047857; padding: 2px 8px; border-radius: 12px; font-weight: 500;">Zero-Alloc WASM</span>
      </div>

      <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 16px; margin-bottom: 16px;">
        <!-- Generator Type -->
        <div>
          <label style="display: block; font-size: 0.85rem; font-weight: 600; margin-bottom: 6px; color: #475569;">Generator Type</label>
          <select id="gen-type-select" style="width: 100%; padding: 8px 10px; border-radius: 6px; border: 1px solid #cbd5e1; font-size: 0.9rem; background: #fff;">
            ${generatorOptions.map(g => `<option value="${g.id}">${g.name}</option>`).join('')}
          </select>
          <div id="gen-desc" style="font-size: 0.75rem; color: #64748b; margin-top: 4px;">${generatorOptions[0].desc}</div>
        </div>

        <!-- Output Format -->
        <div>
          <label style="display: block; font-size: 0.85rem; font-weight: 600; margin-bottom: 6px; color: #475569;">Export Format</label>
          <select id="gen-format-select" style="width: 100%; padding: 8px 10px; border-radius: 6px; border: 1px solid #cbd5e1; font-size: 0.9rem; background: #fff;">
            ${formatOptions.map(f => `<option value="${f.id}">${f.label}</option>`).join('')}
          </select>
        </div>

        <!-- Target Size -->
        <div>
          <label style="display: block; font-size: 0.85rem; font-weight: 600; margin-bottom: 6px; color: #475569;">Target Size</label>
          <div style="display: flex; gap: 6px; flex-wrap: wrap;">
            ${sizePresets.map((p, idx) => `
              <button type="button" class="gen-size-btn" data-bytes="${p.bytes}" style="padding: 4px 10px; border-radius: 4px; border: 1px solid #cbd5e1; background: ${idx === 2 ? '#3b82f6' : '#f8fafc'}; color: ${idx === 2 ? '#fff' : '#334155'}; font-size: 0.8rem; cursor: pointer; font-weight: 500;">${p.label}</button>
            `).join('')}
          </div>
          <input type="number" id="gen-custom-bytes" value="1048576" style="margin-top: 6px; width: 100%; padding: 6px 10px; border-radius: 6px; border: 1px solid #cbd5e1; font-size: 0.85rem;" placeholder="Bytes (e.g. 1048576)">
        </div>
      </div>

      <!-- Generator Dynamic Parameters -->
      <div id="gen-params-container" style="background: #f8fafc; border-radius: 6px; padding: 12px; margin-bottom: 16px; border: 1px solid #e2e8f0;">
        <div style="font-size: 0.85rem; font-weight: 600; margin-bottom: 8px; color: #334155;">Parameters</div>
        <div id="gen-params-fields" style="display: flex; gap: 12px; flex-wrap: wrap;"></div>
      </div>

      <!-- Live Stream Preview -->
      <div style="margin-bottom: 16px;">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px;">
          <span style="font-size: 0.85rem; font-weight: 600; color: #475569;">Live Stream Preview (First 128 Bytes)</span>
          <button id="gen-preview-refresh" type="button" style="background: none; border: none; color: #3b82f6; font-size: 0.8rem; cursor: pointer; font-weight: 500;">↺ Refresh Preview</button>
        </div>
        <pre id="gen-preview-box" style="margin: 0; padding: 10px; background: #0f172a; color: #38bdf8; font-family: ui-monospace, monospace; font-size: 0.78rem; border-radius: 6px; overflow-x: auto; max-height: 120px; line-height: 1.4;"></pre>
      </div>

      <!-- Action Buttons -->
      <div style="display: flex; gap: 10px; flex-wrap: wrap; align-items: center;">
        <button id="gen-download-btn" type="button" style="background: #2563eb; color: #fff; border: none; padding: 8px 18px; border-radius: 6px; font-weight: 600; font-size: 0.9rem; cursor: pointer; display: flex; align-items: center; gap: 6px;">
          <span>💾</span> Download File
        </button>

        ${onRunSuite ? `
          <button id="gen-run-suite-btn" type="button" style="background: #059669; color: #fff; border: none; padding: 8px 18px; border-radius: 6px; font-weight: 600; font-size: 0.9rem; cursor: pointer; display: flex; align-items: center; gap: 6px;">
            <span>▶</span> Run in Test Suite
          </button>
        ` : ''}

        <span id="gen-status-text" style="font-size: 0.85rem; color: #64748b; font-weight: 500;"></span>
      </div>
    </div>
  `;

  const typeSelect = container.querySelector('#gen-type-select');
  const formatSelect = container.querySelector('#gen-format-select');
  const descEl = container.querySelector('#gen-desc');
  const paramsFields = container.querySelector('#gen-params-fields');
  const customBytesInput = container.querySelector('#gen-custom-bytes');
  const previewBox = container.querySelector('#gen-preview-box');
  const downloadBtn = container.querySelector('#gen-download-btn');
  const runSuiteBtn = container.querySelector('#gen-run-suite-btn');
  const statusText = container.querySelector('#gen-status-text');
  const sizeBtns = container.querySelectorAll('.gen-size-btn');

  let currentTargetBytes = 1048576;

  sizeBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      sizeBtns.forEach(b => {
        b.style.background = '#f8fafc';
        b.style.color = '#334155';
      });
      btn.style.background = '#3b82f6';
      btn.style.color = '#fff';
      currentTargetBytes = parseInt(btn.dataset.bytes, 10);
      customBytesInput.value = currentTargetBytes;
    });
  });

  customBytesInput.addEventListener('input', () => {
    currentTargetBytes = parseInt(customBytesInput.value, 10) || 1024;
  });

  function updateParamFields() {
    const typeId = parseInt(typeSelect.value, 10);
    descEl.textContent = generatorOptions[typeId].desc;

    let html = '';
    if (typeId === 0) { // Seq
      html = `
        <div><label style="font-size: 0.75rem; font-weight: 600;">Start Byte:</label><input id="p-seq-start" type="number" value="0" min="0" max="255" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1; width: 70px;"></div>
        <div><label style="font-size: 0.75rem; font-weight: 600;">Step:</label><input id="p-seq-step" type="number" value="1" min="0" max="255" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1; width: 70px;"></div>
      `;
    } else if (typeId === 1) { // Fixed
      html = `
        <div><label style="font-size: 0.75rem; font-weight: 600;">Byte Value (0-255):</label><input id="p-fixed-val" type="number" value="0" min="0" max="255" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1; width: 80px;"></div>
      `;
    } else if (typeId === 2) { // LFSR
      html = `
        <div><label style="font-size: 0.75rem; font-weight: 600;">Order (Bits):</label>
          <select id="p-lfsr-order" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1;">
            ${[3, 4, 8, 9, 11, 13, 19, 27, 31, 32, 64, 96, 128].map(o => `<option value="${o}" ${o === 19 ? 'selected' : ''}>${o}-bit</option>`).join('')}
          </select>
        </div>
        <div><label style="font-size: 0.75rem; font-weight: 600;">Seed:</label><input id="p-lfsr-seed" type="number" value="1" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1; width: 100px;"></div>
      `;
    } else if (typeId === 3) { // De Bruijn
      html = `
        <div><label style="font-size: 0.75rem; font-weight: 600;">Order:</label>
          <select id="p-debruijn-order" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1;">
            ${[4, 9, 13, 19, 31].map(o => `<option value="${o}" ${o === 19 ? 'selected' : ''}>Order ${o} (Period 2^${o})</option>`).join('')}
          </select>
        </div>
      `;
    } else if (typeId === 4) { // LCG
      html = `
        <div><label style="font-size: 0.75rem; font-weight: 600;">Preset:</label>
          <select id="p-lcg-preset" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1;">
            <option value="0">ANSI C rand()</option>
            <option value="1">MINSTD (Park & Miller)</option>
            <option value="2">Knuth 64-bit</option>
          </select>
        </div>
        <div><label style="font-size: 0.75rem; font-weight: 600;">Seed:</label><input id="p-lcg-seed" type="number" value="12345" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1; width: 90px;"></div>
      `;
    } else if (typeId === 6 || typeId === 7) { // Gaussian / Poisson
      html = `
        <div><label style="font-size: 0.75rem; font-weight: 600;">Seed:</label><input id="p-dist-seed" type="number" value="42" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1; width: 90px;"></div>
      `;
    } else {
      html = `
        <div><label style="font-size: 0.75rem; font-weight: 600;">Seed Key (64-bit Int):</label><input id="p-crypto-seed" type="number" value="987654321" style="padding: 4px; border-radius: 4px; border: 1px solid #cbd5e1; width: 140px;"></div>
      `;
    }
    paramsFields.innerHTML = html;
    updatePreview();
  }

  function configureRunner() {
    const typeId = parseInt(typeSelect.value, 10);
    let p1 = 0, p2 = 0, seedLo = 0n, seedHi = 0n;

    if (typeId === 0) {
      p1 = parseInt(container.querySelector('#p-seq-start')?.value || '0', 10);
      p2 = parseInt(container.querySelector('#p-seq-step')?.value || '1', 10);
    } else if (typeId === 1) {
      p1 = parseInt(container.querySelector('#p-fixed-val')?.value || '0', 10);
    } else if (typeId === 2) {
      p1 = parseInt(container.querySelector('#p-lfsr-order')?.value || '19', 10);
      seedLo = BigInt(container.querySelector('#p-lfsr-seed')?.value || '1');
    } else if (typeId === 3) {
      p1 = parseInt(container.querySelector('#p-debruijn-order')?.value || '19', 10);
    } else if (typeId === 4) {
      p1 = parseInt(container.querySelector('#p-lcg-preset')?.value || '0', 10);
      seedLo = BigInt(container.querySelector('#p-lcg-seed')?.value || '12345');
    } else if (typeId === 6 || typeId === 7) {
      seedLo = BigInt(container.querySelector('#p-dist-seed')?.value || '42');
    } else {
      seedLo = BigInt(container.querySelector('#p-crypto-seed')?.value || '987654321');
    }

    runner.init(typeId, p1, p2, seedLo, seedHi);
  }

  function updatePreview() {
    configureRunner();
    const formatCode = parseInt(formatSelect.value, 10);
    const chunk = runner.formatChunk(64, formatCode);

    if (formatCode === 0) {
      // Hex + ASCII view
      let hex = '', ascii = '';
      for (let i = 0; i < chunk.length; i++) {
        hex += chunk[i].toString(16).padStart(2, '0') + ' ';
        ascii += (chunk[i] >= 32 && chunk[i] <= 126) ? String.fromCharCode(chunk[i]) : '.';
        if ((i + 1) % 16 === 0) {
          hex += '  ' + ascii + '\n';
          ascii = '';
        }
      }
      if (ascii) hex += '  ' + ascii;
      previewBox.textContent = hex.trim();
    } else {
      const text = new TextDecoder().decode(chunk);
      previewBox.textContent = text.slice(0, 300) + (text.length > 300 ? '...' : '');
    }
  }

  typeSelect.addEventListener('change', updateParamFields);
  formatSelect.addEventListener('change', updatePreview);
  container.querySelector('#gen-preview-refresh').addEventListener('click', updatePreview);

  downloadBtn.addEventListener('click', async () => {
    downloadBtn.disabled = true;
    statusText.textContent = 'Generating stream...';
    configureRunner();

    const formatCode = parseInt(formatSelect.value, 10);
    const ext = formatOptions[formatCode].ext;
    const typeName = generatorOptions[parseInt(typeSelect.value, 10)].name.toLowerCase().replace(/[^a-z0-9]+/g, '_');
    const filename = `${typeName}_${currentTargetBytes}B.${ext}`;

    const blob = await runner.createDownloadBlob(currentTargetBytes, formatCode, (done, total) => {
      statusText.textContent = `Generated ${Math.round((done / total) * 100)}% (${Math.round(done / 1024)} KB)`;
    });

    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);

    statusText.textContent = `Downloaded ${filename}!`;
    downloadBtn.disabled = false;
  });

  if (runSuiteBtn && onRunSuite) {
    runSuiteBtn.addEventListener('click', () => {
      configureRunner();
      const rawBytes = runner.generateChunk(Math.min(currentTargetBytes, 1048576));
      onRunSuite(rawBytes);
    });
  }

  updateParamFields();
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { renderGeneratorUI };
}
