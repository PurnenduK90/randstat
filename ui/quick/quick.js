/**
 * WASM Quick Randomness Screening & Health Diagnostic Suite runner.
 * Evaluates streaming byte chunks against the 10 quick randomness metrics.
 */
class QuickRunner {
  constructor(wasmInstance) {
    this.wasmInstance = wasmInstance;
    this.instance = wasmInstance;
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-quick.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new QuickRunner(instance);
  }

  reset() {
    this.exports.quick_reset();
    if (this.exports.sha256_reset) this.exports.sha256_reset();
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
      this.exports.quick_update(bufPtr, sliceLen);
      if (this.exports.sha256_update) this.exports.sha256_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize(alpha = 0.05) {
    const evalPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    const validatePtr = evalPtr + 600;
    const shaPtr = evalPtr + 800;

    this.exports.quick_finalize(evalPtr);
    if (this.exports.quick_validate) this.exports.quick_validate(alpha, validatePtr);

    let sha256Hex = '';
    if (this.exports.sha256_finalize) {
      this.exports.sha256_finalize(shaPtr);
      const shaBytes = new Uint8Array(this.memory.buffer, shaPtr, 32);
      for (let i = 0; i < 32; i++) {
        sha256Hex += shaBytes[i].toString(16).padStart(2, '0');
      }
    }

    const view = new DataView(this.memory.buffer);

    const testNames = [
      { id: "QK01", name: "Shannon Entropy", dimension: "Information" },
      { id: "QK02", name: "Min-Entropy (H∞)", dimension: "Information" },
      { id: "QK03", name: "Optimum Compression", dimension: "Information" },
      { id: "QK04", name: "Byte Uniformity (χ²)", dimension: "Frequency" },
      { id: "QK05", name: "Arithmetic Mean", dimension: "Frequency" },
      { id: "QK06", name: "Monobit Frequency", dimension: "Bit Distribution" },
      { id: "QK07", name: "Runs Structure", dimension: "Bit Transitions" },
      { id: "QK08", name: "Poker Test (4-bit)", dimension: "Patterns" },
      { id: "QK09", name: "Serial Correlation (Lag-1)", dimension: "Dependence" },
      { id: "QK10", name: "Monte Carlo Pi", dimension: "Geometry" },
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
        dimension: testNames[i].dimension,
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

    const entResOffset = evalPtr + 424;
    const entResult = {
      totalBytes: Number(view.getBigUint64(entResOffset, true)),
      entropy: view.getFloat64(entResOffset + 8, true),
      compressionPercent: view.getFloat64(entResOffset + 16, true),
      chiSquare: view.getFloat64(entResOffset + 24, true),
      mean: view.getFloat64(entResOffset + 32, true),
      monteCarloPi: view.getFloat64(entResOffset + 40, true),
      serialCorrelation: view.getFloat64(entResOffset + 48, true),
      sha256: sha256Hex,
    };

    const entStatusMap = ['PASS', 'WARN', 'FAIL'];
    const entEvaluation = {
      overallStatus: entStatusMap[view.getUint8(validatePtr)] || 'PASS',
      entropyStatus: entStatusMap[view.getUint8(validatePtr + 1)] || 'PASS',
      chiSquareStatus: entStatusMap[view.getUint8(validatePtr + 2)] || 'PASS',
      meanStatus: entStatusMap[view.getUint8(validatePtr + 3)] || 'PASS',
      piStatus: entStatusMap[view.getUint8(validatePtr + 4)] || 'PASS',
      serialCorrelationStatus: entStatusMap[view.getUint8(validatePtr + 5)] || 'PASS',
      chiSquareLowerBound: view.getFloat64(validatePtr + 8, true) || 218.42,
      chiSquareUpperBound: view.getFloat64(validatePtr + 16, true) || 293.25,
      pochisqExceedProb: view.getFloat64(validatePtr + 24, true) || 50.0,
      piErrorPercent: view.getFloat64(validatePtr + 32, true) || 0.0,
    };

    return {
      entries,
      totalTests: totalTests || 10,
      implementedCount,
      passedCount,
      failedCount,
      skippedCount,
      result: entResult,
      evaluation: entEvaluation,
      sha256: sha256Hex,
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { QuickRunner };
}
