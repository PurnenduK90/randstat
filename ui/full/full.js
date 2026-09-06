/**
 * WASM Full Meta-Suite Runner.
 * Coordinates simultaneous multi-suite evaluation (ENT, NIST, AIS 31) in a single streaming pass.
 */
class FullSuiteRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-full.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new FullSuiteRunner(instance);
  }

  reset() {
    this.exports.ent_reset();
    this.exports.nist_reset();
    this.exports.ais31_reset();
    this.exports.sha256_reset();
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

      this.exports.ent_update(bufPtr, sliceLen);
      this.exports.nist_update(bufPtr, sliceLen);
      this.exports.ais31_update(bufPtr, sliceLen);
      this.exports.sha256_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize(alpha = 0.05) {
    const baseEvalPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    const view = new DataView(this.memory.buffer);

    // 1. SHA-256 Digest
    const shaPtr = baseEvalPtr;
    this.exports.sha256_finalize(shaPtr);
    const shaBytes = new Uint8Array(this.memory.buffer, shaPtr, 32);
    let sha256Hex = '';
    for (let i = 0; i < 32; i++) {
      sha256Hex += shaBytes[i].toString(16).padStart(2, '0');
    }

    // 2. ENT Results & Validation
    const entResPtr = baseEvalPtr + 64;
    this.exports.ent_finalize(entResPtr);

    const entEvalPtr = baseEvalPtr + 256;
    this.exports.ent_validate(alpha, entEvalPtr);

    const statusMap = ['PASS', 'WARN', 'FAIL'];
    const entResult = {
      totalBytes: Number(view.getBigUint64(entResPtr, true)),
      entropy: view.getFloat64(entResPtr + 8, true),
      compressionPercent: view.getFloat64(entResPtr + 16, true),
      chiSquare: view.getFloat64(entResPtr + 24, true),
      mean: view.getFloat64(entResPtr + 32, true),
      monteCarloPi: view.getFloat64(entResPtr + 40, true),
      serialCorrelation: view.getFloat64(entResPtr + 48, true),
      sha256: sha256Hex,
    };
    const entEvaluation = {
      overallStatus: statusMap[view.getUint8(entEvalPtr)] || 'UNKNOWN',
      entropyStatus: statusMap[view.getUint8(entEvalPtr + 1)] || 'UNKNOWN',
      chiSquareStatus: statusMap[view.getUint8(entEvalPtr + 2)] || 'UNKNOWN',
      meanStatus: statusMap[view.getUint8(entEvalPtr + 3)] || 'UNKNOWN',
      piStatus: statusMap[view.getUint8(entEvalPtr + 4)] || 'UNKNOWN',
      serialCorrelationStatus: statusMap[view.getUint8(entEvalPtr + 5)] || 'UNKNOWN',
      chiSquareLowerBound: view.getFloat64(entEvalPtr + 8, true),
      chiSquareUpperBound: view.getFloat64(entEvalPtr + 16, true),
      pochisqExceedProb: view.getFloat64(entEvalPtr + 24, true),
      piErrorPercent: view.getFloat64(entEvalPtr + 32, true),
    };

    // 3. NIST Results (NistEvaluation struct: 15 entries * 40 bytes = 600 bytes)
    const nistPtr = baseEvalPtr + 512;
    this.exports.nist_finalize(nistPtr);
    const nistTestNames = [
      { name: 'Frequency (Monobit)', section: '2.1' },
      { name: 'Block Frequency', section: '2.2' },
      { name: 'Runs', section: '2.3' },
      { name: 'Longest Run of Ones', section: '2.4' },
      { name: 'Binary Matrix Rank', section: '2.5' },
      { name: 'DFT / Spectral', section: '2.6' },
      { name: 'Non-overlapping Template', section: '2.7' },
      { name: 'Overlapping Template', section: '2.8' },
      { name: 'Maurer\'s Universal', section: '2.9' },
      { name: 'Linear Complexity', section: '2.10' },
      { name: 'Serial Test', section: '2.11' },
      { name: 'Approximate Entropy', section: '2.12' },
      { name: 'Cumulative Sums', section: '2.13' },
      { name: 'Random Excursions', section: '2.14' },
      { name: 'Random Excursions Variant', section: '2.15' },
    ];
    const nistStatusMap = ['NOT IMPLEMENTED', 'PASS', 'FAIL', 'INSUFFICIENT DATA'];
    const nistEntries = [];
    let nistOffset = nistPtr;
    for (let i = 0; i < 15; i++) {
      const stat = view.getFloat64(nistOffset + 16, true);
      const pval = view.getFloat64(nistOffset + 24, true);
      const passed = view.getUint8(nistOffset + 32) !== 0;
      const statusIdx = view.getUint8(nistOffset + 33);
      nistEntries.push({
        name: nistTestNames[i].name,
        section: nistTestNames[i].section,
        statistic: isNaN(stat) ? null : stat,
        pValue: isNaN(pval) ? null : pval,
        passed,
        status: nistStatusMap[statusIdx] || 'NOT IMPLEMENTED',
      });
      nistOffset += 40;
    }
    const nistTotalTests = view.getUint32(nistOffset, true) || 15;
    const nistImplementedCount = view.getUint32(nistOffset + 4, true);
    const nistPassedCount = view.getUint32(nistOffset + 8, true);
    const nistFailedCount = view.getUint32(nistOffset + 12, true);
    const nistSkippedCount = view.getUint32(nistOffset + 16, true);
    const nistEvaluation = {
      entries: nistEntries,
      totalTests: nistTotalTests,
      implementedCount: nistImplementedCount,
      passedCount: nistPassedCount,
      failedCount: nistFailedCount,
      skippedCount: nistSkippedCount,
    };

    // 4. AIS 31 Results (Ais31Evaluation struct: 9 entries * 40 bytes = 360 bytes)
    const aisPtr = baseEvalPtr + 1500;
    this.exports.ais31_finalize(aisPtr);
    const aisNames = [
      { id: 'T0', name: 'Test T0: Disjointness' },
      { id: 'T1', name: 'Test T1: Monobit' },
      { id: 'T2', name: 'Test T2: Poker Test' },
      { id: 'T3', name: 'Test T3: Runs' },
      { id: 'T4', name: 'Test T4: Long Runs' },
      { id: 'T5', name: 'Test T5: Autocorrelation' },
      { id: 'T6', name: 'Test T6: Uniform Distribution' },
      { id: 'T7', name: 'Test T7: Comparative Test' },
      { id: 'T8', name: 'Test T8: Shannon Entropy' },
    ];
    const aisEntries = [];
    let aisOffset = aisPtr;
    for (let i = 0; i < 9; i++) {
      const stat = view.getFloat64(aisOffset + 16, true);
      const pval = view.getFloat64(aisOffset + 24, true);
      const passed = view.getUint8(aisOffset + 32) !== 0;
      const statusIdx = view.getUint8(aisOffset + 33);
      aisEntries.push({
        id: aisNames[i].id,
        name: aisNames[i].name,
        statistic: isNaN(stat) ? null : stat,
        pValue: isNaN(pval) ? null : pval,
        passed,
        status: nistStatusMap[statusIdx] || 'NOT IMPLEMENTED',
      });
      aisOffset += 40;
    }
    const aisTotalTests = view.getUint32(aisOffset, true) || 9;
    const aisImplementedCount = view.getUint32(aisOffset + 4, true);
    const aisPassedCount = view.getUint32(aisOffset + 8, true);
    const aisFailedCount = view.getUint32(aisOffset + 12, true);
    const aisSkippedCount = view.getUint32(aisOffset + 16, true);
    const ais31Evaluation = {
      entries: aisEntries,
      totalTests: aisTotalTests,
      implementedCount: aisImplementedCount,
      passedCount: aisPassedCount,
      failedCount: aisFailedCount,
      skippedCount: aisSkippedCount,
    };

    return {
      totalBytes: entResult.totalBytes,
      sha256: sha256Hex,
      ent: { result: entResult, evaluation: entEvaluation },
      nist: nistEvaluation,
      ais31: ais31Evaluation,
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { FullSuiteRunner };
}
