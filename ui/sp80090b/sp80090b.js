/**
 * WASM NIST SP 800-90B Min-Entropy Suite runner.
 * Evaluates streaming byte chunks against the 10 min-entropy estimators.
 */
class Sp80090bRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-sp80090b.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new Sp80090bRunner(instance);
  }

  reset() {
    this.exports.sp80090b_reset();
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
      this.exports.sp80090b_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize() {
    const evalPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    this.exports.sp80090b_finalize(evalPtr);
    const view = new DataView(this.memory.buffer);

    const testNames = [
      { id: "§6.3.1", name: "Most Common Value", track: "Non-IID §6.3.1" },
      { id: "§6.3.2", name: "Collision Test", track: "Non-IID §6.3.2" },
      { id: "§6.3.3", name: "Markov Test", track: "Non-IID §6.3.3" },
      { id: "§6.3.4", name: "Compression Test", track: "Non-IID §6.3.4" },
      { id: "§6.3.5", name: "t-Tuple Test", track: "Non-IID §6.3.5" },
      { id: "§6.3.6", name: "Longest Repeated Substring (LRS)", track: "Non-IID §6.3.6" },
      { id: "§6.3.7", name: "Multi Most Common in Window (MMCW)", track: "Predictor §6.3.7" },
      { id: "§6.3.8", name: "Lag Prediction Test", track: "Predictor §6.3.8" },
      { id: "§6.3.9", name: "MultiMMC Prediction Test", track: "Predictor §6.3.9" },
      { id: "§6.3.10", name: "LZ78Y Prediction Test", track: "Predictor §6.3.10" },
    ];

    const statusMap = ["NOT IMPLEMENTED", "PASS", "FAIL", "INSUFFICIENT DATA"];
    const entries = [];

    let offset = evalPtr;
    for (let i = 0; i < 10; i++) {
      const stat = view.getFloat64(offset + 16, true);
      const pval = view.getFloat64(offset + 24, true);
      const passed = view.getUint8(offset + 32) !== 0;
      const statusIdx = view.getUint8(offset + 33);
      const status = statusMap[statusIdx] || "NOT IMPLEMENTED";

      entries.push({
        id: testNames[i].id,
        name: testNames[i].name,
        track: testNames[i].track,
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
      totalTests: totalTests || 10,
      implementedCount,
      passedCount,
      failedCount,
      skippedCount,
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { Sp80090bRunner };
}
