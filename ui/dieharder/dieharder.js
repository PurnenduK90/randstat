/**
 * WASM Dieharder Test Suite runner.
 * Evaluates streaming byte chunks against the 12 Dieharder tests.
 */
class DieharderRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-dieharder.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new DieharderRunner(instance);
  }

  reset() {
    this.exports.dieharder_reset();
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
      this.exports.dieharder_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize() {
    const evalPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    this.exports.dieharder_finalize(evalPtr);
    const view = new DataView(this.memory.buffer);

    const testNames = [
      { id: "DH01", name: "Birthday Spacings", category: "spatial" },
      { id: "DH02", name: "Parking Lot", category: "spatial" },
      { id: "DH03", name: "Minimum Distance 2D", category: "spatial" },
      { id: "DH04", name: "3D Spheres", category: "spatial" },
      { id: "DH05", name: "Runs Up/Down", category: "runs" },
      { id: "DH06", name: "OPERM5", category: "runs" },
      { id: "DH07", name: "OQSO", category: "template" },
      { id: "DH08", name: "DNA", category: "template" },
      { id: "DH09", name: "Count Ones in Stream", category: "frequency" },
      { id: "DH10", name: "Squeeze", category: "complexity" },
      { id: "DH11", name: "Overlapping Sums", category: "distribution" },
      { id: "DH12", name: "Craps", category: "distribution" },
    ];

    const statusMap = ["NOT IMPLEMENTED", "PASS", "FAIL", "INSUFFICIENT DATA"];
    const entries = [];

    let offset = evalPtr;
    for (let i = 0; i < 12; i++) {
      const stat = view.getFloat64(offset + 16, true);
      const pval = view.getFloat64(offset + 24, true);
      const passed = view.getUint8(offset + 32) !== 0;
      const statusIdx = view.getUint8(offset + 33);
      const status = statusMap[statusIdx] || "NOT IMPLEMENTED";

      entries.push({
        id: testNames[i].id,
        name: testNames[i].name,
        category: testNames[i].category,
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
      totalTests: totalTests || 12,
      implementedCount,
      passedCount,
      failedCount,
      skippedCount,
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { DieharderRunner };
}
