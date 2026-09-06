/**
 * WASM Fourmilab ENT runner.
 * Evaluates streaming byte chunks via WASM linear memory state.
 */
class EntRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-ent.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new EntRunner(instance);
  }

  reset() {
    this.exports.ent_reset();
  }

  update(chunk) {
    if (!chunk || chunk.length === 0) return;
    const len = chunk.length;
    const bufferPtr = 1024;
    const wasmView = new Uint8Array(this.memory.buffer, bufferPtr, len);
    wasmView.set(chunk);
    this.exports.ent_update(bufferPtr, len);
  }

  finalize() {
    const resultPtr = 4096;
    this.exports.ent_finalize(resultPtr);
    const view = new DataView(this.memory.buffer, resultPtr);

    const sha256Bytes = new Uint8Array(this.memory.buffer, resultPtr + 56, 32);
    let sha256Hex = '';
    for (let i = 0; i < 32; i++) {
      sha256Hex += sha256Bytes[i].toString(16).padStart(2, '0');
    }

    return {
      totalBytes: Number(view.getBigUint64(0, true)),
      entropy: view.getFloat64(8, true),
      compressionPercent: view.getFloat64(16, true),
      chiSquare: view.getFloat64(24, true),
      mean: view.getFloat64(32, true),
      monteCarloPi: view.getFloat64(40, true),
      serialCorrelation: view.getFloat64(48, true),
      sha256: sha256Hex,
    };
  }

  validate(alpha = 0.05) {
    const evalPtr = 5120;
    this.exports.ent_validate(alpha, evalPtr);
    const view = new DataView(this.memory.buffer, evalPtr);

    const statusMap = ['PASS', 'WARN', 'FAIL'];
    return {
      overallStatus: statusMap[view.getUint8(0)] || 'UNKNOWN',
      entropyStatus: statusMap[view.getUint8(1)] || 'UNKNOWN',
      chiSquareStatus: statusMap[view.getUint8(2)] || 'UNKNOWN',
      meanStatus: statusMap[view.getUint8(3)] || 'UNKNOWN',
      piStatus: statusMap[view.getUint8(4)] || 'UNKNOWN',
      serialCorrelationStatus: statusMap[view.getUint8(5)] || 'UNKNOWN',
      chiSquareLowerBound: view.getFloat64(8, true),
      chiSquareUpperBound: view.getFloat64(16, true),
      pochisqExceedProb: view.getFloat64(24, true),
      piErrorPercent: view.getFloat64(32, true),
    };
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { EntRunner };
}
