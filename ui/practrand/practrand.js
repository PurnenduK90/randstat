/**
 * WASM PractRand PRNG Test Suite runner.
 * Evaluates streaming byte chunks against the PractRand tests.
 */
class PractRandRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-practrand.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new PractRandRunner(instance);
  }

  reset() {
    this.exports.practrand_reset();
  }

  update(chunk) {
    if (!chunk || chunk.length === 0) return;
    const bufPtr = this.exports.get_input_buffer_ptr ? this.exports.get_input_buffer_ptr() : 65536;
    const capacity = this.exports.get_input_buffer_capacity ? this.exports.get_input_buffer_capacity() : 65536;

    let offset = 0;
    while (offset < chunk.length) {
      const sliceLen = Math.min(chunk.length - offset, capacity);
      const subChunk = chunk.subarray ? chunk.subarray(offset, offset + sliceLen) : chunk.slice(offset, offset + sliceLen);
      const wasmView = new Uint8Array(this.memory.buffer, bufPtr, sliceLen);
      wasmView.set(subChunk);
      this.exports.practrand_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize() {
    const evalPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    this.exports.practrand_finalize(evalPtr);
    const view = new DataView(this.memory.buffer);

    const testNames = [
      { id: "PR01", name: "BCFN (Binary Cell Finite Number)", testType: "Linear / Folding" },
      { id: "PR02", name: "Gap-16", testType: "Frequency / Distance" },
      { id: "PR03", name: "FPFT (Fourier Transform)", testType: "Spectral" },
      { id: "PR04", name: "BRank (Binary Matrix Rank)", testType: "Matrix" },
      { id: "PR05", name: "DC6 (Distance to Cube)", testType: "Spatial" },
      { id: "PR06", name: "Dist-1to4", testType: "Distribution" },
    ];

    const statusMap = ["NOT IMPLEMENTED", "PASS", "FAIL", "INSUFFICIENT DATA"];
    const entries = [];

    let offset = evalPtr;
    for (let i = 0; i < 6; i++) {
      const stat = view.getFloat64(offset + 16, true);
      const pval = view.getFloat64(offset + 24, true);
      const passed = view.getUint8(offset + 32) !== 0;
      const statusIdx = view.getUint8(offset + 33);
      const status = statusMap[statusIdx] || "NOT IMPLEMENTED";

      entries.push({
        id: testNames[i].id,
        name: testNames[i].name,
        testType: testNames[i].testType,
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
      totalTests: totalTests || 6,
      implementedCount,
      passedCount,
      failedCount,
      skippedCount,
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { PractRandRunner };
}
