/**
 * WASM BSI AIS 20 / AIS 31 Physical TRNG Suite runner.
 * Evaluates streaming byte chunks against the 9 AIS 31 tests (T0-T8).
 */
class Ais31Runner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-ais31.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new Ais31Runner(instance);
  }

  reset() {
    this.exports.ais31_reset();
  }

  update(chunk) {
    if (!chunk || chunk.length === 0) return;
    const len = chunk.length;
    const bufferPtr = 1024;
    const wasmView = new Uint8Array(this.memory.buffer, bufferPtr, len);
    wasmView.set(chunk);
    this.exports.ais31_update(bufferPtr, len);
  }

  finalize() {
    const evalPtr = 4096;
    this.exports.ais31_finalize(evalPtr);
    const view = new DataView(this.memory.buffer, evalPtr);

    const testNames = [
      { id: "T0", name: "Test T0: Disjointness" },
      { id: "T1", name: "Test T1: Monobit" },
      { id: "T2", name: "Test T2: Poker Test" },
      { id: "T3", name: "Test T3: Runs" },
      { id: "T4", name: "Test T4: Long Runs" },
      { id: "T5", name: "Test T5: Autocorrelation" },
      { id: "T6", name: "Test T6: Uniform Distribution" },
      { id: "T7", name: "Test T7: Comparative Test" },
      { id: "T8", name: "Test T8: Shannon Entropy" },
    ];

    const statusMap = ["NOT IMPLEMENTED", "PASS", "FAIL", "INSUFFICIENT DATA"];
    const entries = [];
    
    let offset = evalPtr;
    for (let i = 0; i < 9; i++) {
      const stat = view.getFloat64(offset + 16, true);
      const pval = view.getFloat64(offset + 24, true);
      const passed = view.getUint8(offset + 32) !== 0;
      const statusIdx = view.getUint8(offset + 33);
      const status = statusMap[statusIdx] || "NOT IMPLEMENTED";

      entries.push({
        id: testNames[i].id,
        name: testNames[i].name,
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
      totalTests: totalTests || 9,
      implementedCount,
      passedCount,
      failedCount,
      skippedCount,
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { Ais31Runner };
}
