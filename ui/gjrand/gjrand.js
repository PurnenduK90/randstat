/**
 * WASM gjrand Lightweight PRNG Test Suite runner.
 * Evaluates streaming byte chunks against the 10 gjrand tests.
 */
class GjrandRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-gjrand.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new GjrandRunner(instance);
  }

  reset() {
    this.exports.gjrand_reset();
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
      this.exports.gjrand_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize() {
    const evalPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    this.exports.gjrand_finalize(evalPtr);
    const view = new DataView(this.memory.buffer);

    const testNames = [
      { id: "GJ01", name: "mcoll16", profile: "16-bit" },
      { id: "GJ02", name: "mcoll32", profile: "32-bit" },
      { id: "GJ03", name: "mprob16", profile: "16-bit" },
      { id: "GJ04", name: "mprob32", profile: "32-bit" },
      { id: "GJ05", name: "mdist16", profile: "16-bit" },
      { id: "GJ06", name: "mdist32", profile: "32-bit" },
      { id: "GJ07", name: "mgap16", profile: "16-bit" },
      { id: "GJ08", name: "mgap32", profile: "32-bit" },
      { id: "GJ09", name: "mrun16", profile: "16-bit" },
      { id: "GJ10", name: "mrun32", profile: "32-bit" },
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
        profile: testNames[i].profile,
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
  module.exports = { GjrandRunner };
}
