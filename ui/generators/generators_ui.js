/**
 * Interactive UI widget for Randstat Test Stream Generators.
 */
function renderGeneratorUI(container, runner, onRunSuite = null) {
  if (!container) return;

  const generatorOptions = [
    { id: 0, name: 'Sequential Ramp', desc: 'Linear progression: (start + i × step) % 256' },
    { id: 1, name: 'Fixed Constant', desc: 'Repeating byte: all zeros (0x00), ones (0xFF), or custom byte K' },
    { id: 2, name: 'LFSR (Shift Register)', desc: 'Primitive polynomial feedback shift register of orders 3 to 128' },
    { id: 3, name: 'De Bruijn Cycle', desc: 'Maximal cycle of period 2ⁿ containing all n-bit combinations' },
    { id: 4, name: 'LCG (Linear Congruential)', desc: 'Standard recurrence: ANSI C rand(), MINSTD, or Knuth 64-bit' },
    { id: 5, name: 'Xoshiro256**', desc: 'High-speed, all-purpose simulation PRNG with 256-bit state' },
    { id: 6, name: 'Gaussian (Normal)', desc: 'Box-Muller normal distribution with mean μ=127.5, std dev σ=32.0' },
    { id: 7, name: 'Poisson (λ=127)', desc: 'Discrete Poisson distribution with rate parameter λ=127.0' },
    { id: 8, name: 'NIST SP 800-90A AES CTR_DRBG', desc: 'NIST certified AES-128 counter-mode deterministic bit generator' },
    { id: 9, name: 'NIST SP 800-90A SHA-256 Hash_DRBG', desc: 'NIST certified SHA-256 hash-based deterministic bit generator' },
    { id: 10, name: 'ChaCha20 Keystream', desc: 'RFC 8439 256-bit stream cipher keystream output' },
  ];

  const formatOptions = [
    { id: 0, label: 'Raw Binary (.bin)', ext: 'bin' },
    { id: 1, label: 'ASCII Integers (.txt)', ext: 'txt' },
    { id: 2, label: 'ASCII Bits (.bits)', ext: 'bits' },
    { id: 3, label: 'Hexadecimal (.hex)', ext: 'hex' },
    { id: 4, label: 'Normalized Float (.csv)', ext: 'csv' },
  ];

  const sizeOptions = [
    { value: '1024', label: '1 KB (1,024 bytes)' },
    { value: '65536', label: '64 KB (65,536 bytes)' },
    { value: '262144', label: '256 KB (262,144 bytes)' },
    { value: '1048576', label: '1 MB (1,048,576 bytes)' },
    { value: '10485760', label: '10 MB (10,485,760 bytes)' },
    { value: '52428800', label: '50 MB (52,428,800 bytes)' },
    { value: '104857600', label: '100 MB (104,857,600 bytes)' },
    { value: 'custom', label: 'Custom Byte Size...' },
  ];

  container.innerHTML = `
    <div class="randstat-gen-card" style="border: 1px solid #1e293b; border-radius: 12px; padding: 24px; background: #0f172a; color: #f8fafc; font-family: system-ui, -apple-system, sans-serif;">
      <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #1e293b; padding-bottom: 14px; margin-bottom: 20px;">
        <h3 style="margin: 0; font-size: 1.2rem; color: #38bdf8; display: flex; align-items: center; gap: 8px;">
          <span>⚡</span> Randstat Test Stream Generator
        </h3>
        <span style="font-size: 0.75rem; background: #10b98122; color: #10b981; border: 1px solid #10b98144; padding: 3px 10px; border-radius: 12px; font-weight: 600;">Zero-Alloc WASM</span>
      </div>

      <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 16px; margin-bottom: 20px;">
        <!-- 1. Generator Type -->
        <div>
          <label for="gen-type-select" style="display: block; font-size: 0.82rem; font-weight: 600; margin-bottom: 6px; color: #94a3b8;">Generator Type</label>
          <select id="gen-type-select" style="width: 100%; padding: 8px 10px; border-radius: 6px; border: 1px solid #334155; font-size: 0.9rem; background: #090d16; color: #f8fafc; outline: none;">
            ${generatorOptions.map(g => `<option value="${g.id}">${g.name}</option>`).join('')}
          </select>
          <div id="gen-desc" style="font-size: 0.72rem; color: #38bdf8; margin-top: 5px; line-height: 1.3;">${generatorOptions[0].desc}</div>
        </div>

        <!-- 2. Output Format -->
        <div>
          <label for="gen-format-select" style="display: block; font-size: 0.82rem; font-weight: 600; margin-bottom: 6px; color: #94a3b8;">Export Format</label>
          <select id="gen-format-select" style="width: 100%; padding: 8px 10px; border-radius: 6px; border: 1px solid #334155; font-size: 0.9rem; background: #090d16; color: #f8fafc; outline: none;">
            ${formatOptions.map(f => `<option value="${f.id}">${f.label}</option>`).join('')}
          </select>
          <div style="font-size: 0.72rem; color: #64748b; margin-top: 5px;">Binary byte stream or ASCII text representations</div>
        </div>

        <!-- 3. Target Size Dropdown -->
        <div>
          <label for="gen-size-select" style="display: block; font-size: 0.82rem; font-weight: 600; margin-bottom: 6px; color: #94a3b8;">Target Size</label>
          <select id="gen-size-select" style="width: 100%; padding: 8px 10px; border-radius: 6px; border: 1px solid #334155; font-size: 0.9rem; background: #090d16; color: #f8fafc; outline: none;">
            ${sizeOptions.map(s => `<option value="${s.value}" ${s.value === '1048576' ? 'selected' : ''}>${s.label}</option>`).join('')}
          </select>
          <div id="gen-custom-size-wrap" style="display: none; margin-top: 8px;">
            <input type="number" id="gen-custom-bytes" value="1048576" min="1" style="width: 100%; padding: 7px 10px; border-radius: 6px; border: 1px solid #38bdf866; font-size: 0.85rem; background: #090d16; color: #38bdf8; font-family: ui-monospace, monospace;" placeholder="Enter custom byte count (e.g. 1048576)">
            <div style="font-size: 0.7rem; color: #64748b; margin-top: 3px;">Supports streaming chunk generation up to 1 GB</div>
          </div>
        </div>
      </div>

      <!-- Generator Dynamic Parameters -->
      <div id="gen-params-container" style="background: #1e293b; border-radius: 8px; padding: 16px; margin-bottom: 20px; border: 1px solid #334155;">
        <div style="font-size: 0.82rem; font-weight: 700; margin-bottom: 12px; color: #38bdf8; text-transform: uppercase; letter-spacing: 0.05em; display: flex; align-items: center; gap: 6px;">
          <span>⚙️</span> Generator Parameters & Configuration
        </div>
        <div id="gen-params-fields" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px;"></div>
      </div>

      <!-- Live Stream Preview -->
      <div style="margin-bottom: 20px;">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
          <span style="font-size: 0.82rem; font-weight: 600; color: #94a3b8;">Live Stream Preview (First 128 Bytes)</span>
          <button id="gen-preview-refresh" type="button" style="background: none; border: none; color: #38bdf8; font-size: 0.8rem; cursor: pointer; font-weight: 600;">↺ Refresh Preview</button>
        </div>
        <pre id="gen-preview-box" style="margin: 0; padding: 12px 16px; background: #090d16; color: #38bdf8; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; font-size: 0.8rem; border-radius: 8px; border: 1px solid #1e293b; overflow-x: auto; max-height: 140px; line-height: 1.5;"></pre>
      </div>

      <!-- Action Buttons -->
      <div style="display: flex; gap: 12px; flex-wrap: wrap; align-items: center;">
        <button id="gen-download-btn" type="button" style="background: #0284c7; color: #fff; border: 1px solid #38bdf844; padding: 9px 20px; border-radius: 6px; font-weight: 600; font-size: 0.9rem; cursor: pointer; display: flex; align-items: center; gap: 8px; transition: background 0.2s;">
          <span>💾</span> Download File
        </button>

        ${onRunSuite ? `
          <button id="gen-run-suite-btn" type="button" style="background: #10b981; color: #fff; border: 1px solid #10b98144; padding: 9px 20px; border-radius: 6px; font-weight: 600; font-size: 0.9rem; cursor: pointer; display: flex; align-items: center; gap: 8px;">
            <span>▶</span> Run in Test Suite
          </button>
        ` : ''}

        <span id="gen-status-text" style="font-size: 0.85rem; color: #94a3b8; font-weight: 500;"></span>
      </div>
    </div>
  `;

  const typeSelect = container.querySelector('#gen-type-select');
  const formatSelect = container.querySelector('#gen-format-select');
  const sizeSelect = container.querySelector('#gen-size-select');
  const customSizeWrap = container.querySelector('#gen-custom-size-wrap');
  const customBytesInput = container.querySelector('#gen-custom-bytes');
  const descEl = container.querySelector('#gen-desc');
  const paramsFields = container.querySelector('#gen-params-fields');
  const previewBox = container.querySelector('#gen-preview-box');
  const downloadBtn = container.querySelector('#gen-download-btn');
  const runSuiteBtn = container.querySelector('#gen-run-suite-btn');
  const statusText = container.querySelector('#gen-status-text');

  let currentTargetBytes = 1048576;

  // Handle Size Dropdown
  sizeSelect.addEventListener('change', () => {
    if (sizeSelect.value === 'custom') {
      customSizeWrap.style.display = 'block';
      currentTargetBytes = Math.max(1, parseInt(customBytesInput.value, 10) || 1048576);
    } else {
      customSizeWrap.style.display = 'none';
      currentTargetBytes = parseInt(sizeSelect.value, 10);
      customBytesInput.value = currentTargetBytes;
    }
  });

  customBytesInput.addEventListener('input', () => {
    currentTargetBytes = Math.max(1, parseInt(customBytesInput.value, 10) || 1024);
  });

  function updateParamFields() {
    const typeId = parseInt(typeSelect.value, 10);
    descEl.textContent = generatorOptions[typeId].desc;

    let html = '';
    if (typeId === 0) { // Sequential Ramp
      html = `
        <div>
          <label for="p-seq-start" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">Start Byte (0–255)</label>
          <input id="p-seq-start" type="number" value="0" min="0" max="255" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Initial byte emitted at sequence index 0</div>
        </div>
        <div>
          <label for="p-seq-step" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">Step Increment (1–255)</label>
          <input id="p-seq-step" type="number" value="1" min="0" max="255" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Value added per byte: (start + i × step) % 256</div>
        </div>
      `;
    } else if (typeId === 1) { // Fixed Constant
      html = `
        <div>
          <label for="p-fixed-val" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">Constant Byte Value (0–255)</label>
          <input id="p-fixed-val" type="number" value="0" min="0" max="255" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">0x00 = 0 (all zeros), 0xFF = 255 (all ones), 0xAA = 170</div>
        </div>
      `;
    } else if (typeId === 2) { // LFSR
      html = `
        <div>
          <label for="p-lfsr-order" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">Polynomial Order (Shift Register Size)</label>
          <select id="p-lfsr-order" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; outline: none;">
            ${[3, 4, 8, 9, 11, 13, 19, 27, 31, 32, 64, 96, 128].map(o => `<option value="${o}" ${o === 19 ? 'selected' : ''}>${o}-bit Order (Period 2^${o} - 1)</option>`).join('')}
          </select>
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Primitive polynomial feedback taps</div>
        </div>
        <div>
          <label for="p-lfsr-seed" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">Initial State Seed (Non-Zero)</label>
          <input id="p-lfsr-seed" type="number" value="1" min="1" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Initial shift register state (must be non-zero)</div>
        </div>
      `;
    } else if (typeId === 3) { // De Bruijn
      html = `
        <div>
          <label for="p-debruijn-order" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">De Bruijn Sequence Order (n)</label>
          <select id="p-debruijn-order" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; outline: none;">
            ${[4, 9, 13, 19, 31].map(o => `<option value="${o}" ${o === 19 ? 'selected' : ''}>Order ${o} (Cycle length 2^${o} bits)</option>`).join('')}
          </select>
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Maximal cycle containing all 2ⁿ n-bit patterns exactly once</div>
        </div>
      `;
    } else if (typeId === 4) { // LCG
      html = `
        <div>
          <label for="p-lcg-preset" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">LCG Parameter Standard Preset</label>
          <select id="p-lcg-preset" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; outline: none;">
            <option value="0">ANSI C rand() (a = 1103515245, c = 12345, m = 2³¹)</option>
            <option value="1">MINSTD Park & Miller (a = 16807, c = 0, m = 2³¹ - 1)</option>
            <option value="2">Knuth 64-bit MMIX (a = 6364136223846793005, c = 1442695040888963407)</option>
          </select>
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Recurrence formula: Xₙ₊₁ = (a × Xₙ + c) mod m</div>
        </div>
        <div>
          <label for="p-lcg-seed" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">Initial Seed Value (X₀)</label>
          <input id="p-lcg-seed" type="number" value="12345" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Starting seed value for recurrence relation</div>
        </div>
      `;
    } else if (typeId === 5) { // Xoshiro256**
      html = `
        <div>
          <label for="p-crypto-seed" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">PRNG Seed State (64-bit Integer)</label>
          <input id="p-crypto-seed" type="number" value="123456789" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Seeds SplitMix64 to initialize the 4-word (256-bit) generator state</div>
        </div>
      `;
    } else if (typeId === 6) { // Gaussian
      html = `
        <div>
          <label for="p-dist-seed" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">Box-Muller PRNG Seed</label>
          <input id="p-dist-seed" type="number" value="42" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Fixed distribution profile: Mean μ = 127.5, Std Dev σ = 32.0</div>
        </div>
      `;
    } else if (typeId === 7) { // Poisson
      html = `
        <div>
          <label for="p-dist-seed" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">Poisson Inversion PRNG Seed</label>
          <input id="p-dist-seed" type="number" value="42" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Fixed distribution profile: Rate parameter λ = 127.0</div>
        </div>
      `;
    } else if (typeId === 8) { // AES CTR_DRBG
      html = `
        <div>
          <label for="p-crypto-seed" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">AES-128 Entropy Seed (64-bit Nonce)</label>
          <input id="p-crypto-seed" type="number" value="987654321" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Entropy input initializing AES-128 cipher key and counter state V</div>
        </div>
      `;
    } else if (typeId === 9) { // SHA Hash_DRBG
      html = `
        <div>
          <label for="p-crypto-seed" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">SHA-256 Entropy Seed (64-bit Input)</label>
          <input id="p-crypto-seed" type="number" value="987654321" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Entropy input deriving initial internal state V and constant C via SHA-256</div>
        </div>
      `;
    } else { // ChaCha20
      html = `
        <div>
          <label for="p-crypto-seed" style="display: block; font-size: 0.8rem; font-weight: 600; color: #f8fafc; margin-bottom: 4px;">ChaCha20 Key/Nonce Seed (64-bit)</label>
          <input id="p-crypto-seed" type="number" value="987654321" style="width: 100%; padding: 7px 10px; background: #090d16; border: 1px solid #334155; border-radius: 6px; color: #f8fafc; font-size: 0.85rem; font-family: ui-monospace, monospace;">
          <div style="font-size: 0.72rem; color: #94a3b8; margin-top: 4px;">Seeds 256-bit key expansion and 96-bit nonce for RFC 8439 cipher</div>
        </div>
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
  paramsFields.addEventListener('input', updatePreview);
  paramsFields.addEventListener('change', updatePreview);
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
