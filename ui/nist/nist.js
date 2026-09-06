/**
 * WASM NIST SP 800-22 Rev 1a Suite runner.
 * Evaluates streaming byte chunks against the 15 NIST SP 800-22 tests.
 */
class NistRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-nist.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new NistRunner(instance);
  }

  reset() {
    this.exports.nist_reset();
  }

  update(chunk) {
    if (!chunk || chunk.length === 0) return;
    const len = chunk.length;
    const bufferPtr = 1024;
    const wasmView = new Uint8Array(this.memory.buffer, bufferPtr, len);
    wasmView.set(chunk);
    this.exports.nist_update(bufferPtr, len);
  }

  finalize() {
    const evalPtr = 4096;
    this.exports.nist_finalize(evalPtr);
    const view = new DataView(this.memory.buffer, evalPtr);

    const testNames = [
      { name: "Frequency (Monobit)", section: "2.1" },
      { name: "Block Frequency", section: "2.2" },
      { name: "Runs", section: "2.3" },
      { name: "Longest Run of Ones", section: "2.4" },
      { name: "Binary Matrix Rank", section: "2.5" },
      { name: "DFT / Spectral", section: "2.6" },
      { name: "Non-overlapping Template", section: "2.7" },
      { name: "Overlapping Template", section: "2.8" },
      { name: "Maurer's Universal", section: "2.9" },
      { name: "Linear Complexity", section: "2.10" },
      { name: "Serial Test", section: "2.11" },
      { name: "Approximate Entropy", section: "2.12" },
      { name: "Cumulative Sums", section: "2.13" },
      { name: "Random Excursions", section: "2.14" },
      { name: "Random Excursions Variant", section: "2.15" },
    ];

    const statusMap = ["NOT IMPLEMENTED", "PASS", "FAIL", "INSUFFICIENT DATA"];
    const entries = [];
    
    // In wasm32: each NistTestEntry is 32 or 40 bytes.
    // We can also extract results via pointer strides:
    let offset = evalPtr;
    for (let i = 0; i < 15; i++) {
      // Find TestResult: 2 string fat pointers (16 bytes) then statistic (f64), p_value (f64), passed (bool), status (u8)
      const stat = view.getFloat64(offset + 16, true);
      const pval = view.getFloat64(offset + 24, true);
      const passed = view.getUint8(offset + 32) !== 0;
      const statusIdx = view.getUint8(offset + 33);
      const status = statusMap[statusIdx] || "NOT IMPLEMENTED";

      entries.push({
        name: testNames[i].name,
        section: testNames[i].section,
        statistic: isNaN(stat) ? null : stat,
        pValue: isNaN(pval) ? null : pval,
        passed,
        status,
      });

      offset += 40; // 40 bytes per entry
    }

    const totalTests = view.getUint32(offset, true);
    const implementedCount = view.getUint32(offset + 4, true);
    const passedCount = view.getUint32(offset + 8, true);
    const failedCount = view.getUint32(offset + 12, true);
    const skippedCount = view.getUint32(offset + 16, true);

    return {
      entries,
      totalTests: totalTests || 15,
      implementedCount,
      passedCount,
      failedCount,
      skippedCount,
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { NistRunner };
}
