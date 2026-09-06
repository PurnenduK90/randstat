/**
 * WASM Full Meta-Suite Runner.
 * Coordinates simultaneous multi-suite evaluation across all 8 batteries in a single streaming pass.
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
    if (this.exports.ent_reset) this.exports.ent_reset();
    if (this.exports.nist_reset) this.exports.nist_reset();
    if (this.exports.ais31_reset) this.exports.ais31_reset();
    if (this.exports.dieharder_reset) this.exports.dieharder_reset();
    if (this.exports.sp80090b_reset) this.exports.sp80090b_reset();
    if (this.exports.testu01_reset) this.exports.testu01_reset();
    if (this.exports.practrand_reset) this.exports.practrand_reset();
    if (this.exports.gjrand_reset) this.exports.gjrand_reset();
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

      if (this.exports.ent_update) this.exports.ent_update(bufPtr, sliceLen);
      if (this.exports.nist_update) this.exports.nist_update(bufPtr, sliceLen);
      if (this.exports.ais31_update) this.exports.ais31_update(bufPtr, sliceLen);
      if (this.exports.dieharder_update) this.exports.dieharder_update(bufPtr, sliceLen);
      if (this.exports.sp80090b_update) this.exports.sp80090b_update(bufPtr, sliceLen);
      if (this.exports.testu01_update) this.exports.testu01_update(bufPtr, sliceLen);
      if (this.exports.practrand_update) this.exports.practrand_update(bufPtr, sliceLen);
      if (this.exports.gjrand_update) this.exports.gjrand_update(bufPtr, sliceLen);
      if (this.exports.sha256_update) this.exports.sha256_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize(alpha = 0.05) {
    const baseEvalPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    const view = new DataView(this.memory.buffer);
    const statusMap = ["NOT IMPLEMENTED", "PASS", "FAIL", "INSUFFICIENT DATA"];

    // 1. SHA-256 Digest
    const shaPtr = baseEvalPtr;
    let sha256Hex = '';
    if (this.exports.sha256_finalize) {
      this.exports.sha256_finalize(shaPtr);
      const shaBytes = new Uint8Array(this.memory.buffer, shaPtr, 32);
      for (let i = 0; i < 32; i++) {
        sha256Hex += shaBytes[i].toString(16).padStart(2, '0');
      }
    }

    // 2. ENT Results & Validation
    const entResPtr = baseEvalPtr + 64;
    const entEvalPtr = baseEvalPtr + 160;
    if (this.exports.ent_finalize) this.exports.ent_finalize(entResPtr);
    if (this.exports.ent_validate) this.exports.ent_validate(alpha, entEvalPtr);

    const entStatusMap = ['PASS', 'WARN', 'FAIL'];
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
      overallStatus: entStatusMap[view.getUint8(entEvalPtr)] || 'UNKNOWN',
      entropyStatus: entStatusMap[view.getUint8(entEvalPtr + 1)] || 'UNKNOWN',
      chiSquareStatus: entStatusMap[view.getUint8(entEvalPtr + 2)] || 'UNKNOWN',
      meanStatus: entStatusMap[view.getUint8(entEvalPtr + 3)] || 'UNKNOWN',
      piStatus: entStatusMap[view.getUint8(entEvalPtr + 4)] || 'UNKNOWN',
      serialCorrelationStatus: entStatusMap[view.getUint8(entEvalPtr + 5)] || 'UNKNOWN',
      chiSquareLowerBound: view.getFloat64(entEvalPtr + 8, true),
      chiSquareUpperBound: view.getFloat64(entEvalPtr + 16, true),
      pochisqExceedProb: view.getFloat64(entEvalPtr + 24, true),
      piErrorPercent: view.getFloat64(entEvalPtr + 32, true),
    };

    // Helper function to parse 40-byte test entries
    function parseEntries(ptr, count, testDefs) {
      const entries = [];
      let off = ptr;
      for (let i = 0; i < count; i++) {
        const stat = view.getFloat64(off + 16, true);
        const pval = view.getFloat64(off + 24, true);
        const passed = view.getUint8(off + 32) !== 0;
        const statusIdx = view.getUint8(off + 33);
        const def = testDefs[i] || {};
        entries.push({
          ...def,
          statistic: isNaN(stat) ? null : stat,
          pValue: isNaN(pval) ? null : pval,
          passed,
          status: statusMap[statusIdx] || "NOT IMPLEMENTED",
        });
        off += 40;
      }
      return {
        entries,
        totalTests: view.getUint32(off, true) || count,
        implementedCount: view.getUint32(off + 4, true),
        passedCount: view.getUint32(off + 8, true),
        failedCount: view.getUint32(off + 12, true),
        skippedCount: view.getUint32(off + 16, true),
      };
    }

    // 3. NIST Results
    const nistPtr = baseEvalPtr + 256;
    if (this.exports.nist_finalize) this.exports.nist_finalize(nistPtr);
    const nistDefs = [
      { id: "2.1", name: 'Frequency (Monobit)', section: '2.1' },
      { id: "2.2", name: 'Block Frequency', section: '2.2' },
      { id: "2.3", name: 'Runs', section: '2.3' },
      { id: "2.4", name: 'Longest Run of Ones', section: '2.4' },
      { id: "2.5", name: 'Binary Matrix Rank', section: '2.5' },
      { id: "2.6", name: 'DFT / Spectral', section: '2.6' },
      { id: "2.7", name: 'Non-overlapping Template', section: '2.7' },
      { id: "2.8", name: 'Overlapping Template', section: '2.8' },
      { id: "2.9", name: 'Maurer\'s Universal', section: '2.9' },
      { id: "2.10", name: 'Linear Complexity', section: '2.10' },
      { id: "2.11", name: 'Serial Test', section: '2.11' },
      { id: "2.12", name: 'Approximate Entropy', section: '2.12' },
      { id: "2.13", name: 'Cumulative Sums', section: '2.13' },
      { id: "2.14", name: 'Random Excursions', section: '2.14' },
      { id: "2.15", name: 'Random Excursions Variant', section: '2.15' },
    ];
    const nistEvaluation = parseEntries(nistPtr, 15, nistDefs);

    // 4. AIS 31 Results
    const aisPtr = baseEvalPtr + 1000;
    if (this.exports.ais31_finalize) this.exports.ais31_finalize(aisPtr);
    const aisDefs = [
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
    const ais31Evaluation = parseEntries(aisPtr, 9, aisDefs);

    // 5. Dieharder Results
    const dieharderPtr = baseEvalPtr + 1500;
    if (this.exports.dieharder_finalize) this.exports.dieharder_finalize(dieharderPtr);
    const dieharderDefs = [
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
    const dieharderEvaluation = parseEntries(dieharderPtr, 12, dieharderDefs);

    // 6. SP 800-90B Results
    const spPtr = baseEvalPtr + 2100;
    if (this.exports.sp80090b_finalize) this.exports.sp80090b_finalize(spPtr);
    const spDefs = [
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
    const sp80090bEvaluation = parseEntries(spPtr, 10, spDefs);

    // 7. TestU01 Results
    const tu01Ptr = baseEvalPtr + 2600;
    if (this.exports.testu01_finalize) this.exports.testu01_finalize(tu01Ptr);
    const tu01Defs = [
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
    const testu01Evaluation = parseEntries(tu01Ptr, 10, tu01Defs);

    // 8. PractRand Results
    const prPtr = baseEvalPtr + 3100;
    if (this.exports.practrand_finalize) this.exports.practrand_finalize(prPtr);
    const prDefs = [
      { id: "PR01", name: "Gap-16:B", testType: "[Low1/8]" },
      { id: "PR02", name: "FPF-16:B", testType: "[Low1/8]" },
      { id: "PR03", name: "BCFN(2+0,13/64)", testType: "[Low1/8]" },
      { id: "PR04", name: "BCFN(2+1,13/64)", testType: "[Low1/8]" },
      { id: "PR05", name: "DC6-9x1Bytes-1", testType: "[Low4/8]" },
      { id: "PR06", name: "BRank(12)", testType: "[Low4/8]" },
      { id: "PR07", name: "FPF-8:all64k", testType: "[Low8/8]" },
      { id: "PR08", name: "Dist-64x2:g", testType: "[Low8/8]" },
      { id: "PR09", name: "Gap-8:all64k", testType: "[Low8/8]" },
      { id: "PR10", name: "AutoCor-64", testType: "[Low8/8]" },
    ];
    const practrandEvaluation = parseEntries(prPtr, 10, prDefs);

    // 9. gjrand Results
    const gjPtr = baseEvalPtr + 3600;
    if (this.exports.gjrand_finalize) this.exports.gjrand_finalize(gjPtr);
    const gjDefs = [
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
    const gjrandEvaluation = parseEntries(gjPtr, 10, gjDefs);

    return {
      totalBytes: entResult.totalBytes,
      sha256: sha256Hex,
      ent: { result: entResult, evaluation: entEvaluation },
      nist: nistEvaluation,
      ais31: ais31Evaluation,
      dieharder: dieharderEvaluation,
      sp80090b: sp80090bEvaluation,
      testu01: testu01Evaluation,
      practrand: practrandEvaluation,
      gjrand: gjrandEvaluation,
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { FullSuiteRunner };
}
