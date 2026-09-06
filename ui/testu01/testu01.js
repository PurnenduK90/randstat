/**
 * WASM TestU01 Academic PRNG Benchmark Suite runner.
 * Evaluates streaming byte chunks against the TestU01 SmallCrush battery.
 */
class TestU01Runner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-testu01.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new TestU01Runner(instance);
  }

  reset() {
    this.exports.testu01_reset();
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
      this.exports.testu01_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize() {
    const evalPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    this.exports.testu01_finalize(evalPtr);
    const view = new DataView(this.memory.buffer);

    const testNames = [
      { id: "TU01", name: "smarsa_BirthdaySpacings", battery: "SmallCrush" },
      { id: "TU02", name: "sknuth_Collision", battery: "SmallCrush" },
      { id: "TU03", name: "sknuth_Gap", battery: "SmallCrush" },
      { id: "TU04", name: "sknuth_SimpPoker", battery: "SmallCrush" },
      { id: "TU05", name: "sknuth_CouponCollector", battery: "SmallCrush" },
      { id: "TU06", name: "sknuth_MaxOft", battery: "SmallCrush" },
      { id: "TU07", name: "svar_WeightDistrib", battery: "SmallCrush" },
      { id: "TU08", name: "smarsa_MatrixRank", battery: "SmallCrush" },
      { id: "TU09", name: "sstring_HammingIndep", battery: "SmallCrush" },
      { id: "TU10", name: "sstring_Run", battery: "SmallCrush" },
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
        battery: testNames[i].battery,
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
  module.exports = { TestU01Runner };
}
