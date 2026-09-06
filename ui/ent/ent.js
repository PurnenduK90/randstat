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
    const bufPtr = this.exports.get_input_buffer_ptr ? this.exports.get_input_buffer_ptr() : 65536;
    const capacity = this.exports.get_input_buffer_capacity ? this.exports.get_input_buffer_capacity() : 65536;

    let offset = 0;
    while (offset < chunk.length) {
      const sliceLen = Math.min(chunk.length - offset, capacity);
      const subChunk = chunk.subarray ? chunk.subarray(offset, offset + sliceLen) : chunk.slice(offset, offset + sliceLen);
      const wasmView = new Uint8Array(this.memory.buffer, bufPtr, sliceLen);
      wasmView.set(subChunk);
      this.exports.ent_update(bufPtr, sliceLen);
      offset += sliceLen;
    }
  }

  finalize() {
    const resultPtr = this.exports.get_eval_buffer_ptr ? this.exports.get_eval_buffer_ptr() : 131072;
    this.exports.ent_finalize(resultPtr);
    const view = new DataView(this.memory.buffer);

    const sha256Bytes = new Uint8Array(this.memory.buffer, resultPtr + 56, 32);
    let sha256Hex = '';
    for (let i = 0; i < 32; i++) {
      sha256Hex += sha256Bytes[i].toString(16).padStart(2, '0');
    }

    return {
      totalBytes: Number(view.getBigUint64(resultPtr, true)),
      entropy: view.getFloat64(resultPtr + 8, true),
      compressionPercent: view.getFloat64(resultPtr + 16, true),
      chiSquare: view.getFloat64(resultPtr + 24, true),
      mean: view.getFloat64(resultPtr + 32, true),
      monteCarloPi: view.getFloat64(resultPtr + 40, true),
      serialCorrelation: view.getFloat64(resultPtr + 48, true),
      sha256: sha256Hex,
    };
  }

  validate(alpha = 0.05) {
    const evalPtr = this.exports.get_eval_buffer_ptr ? (this.exports.get_eval_buffer_ptr() + 1024) : 132096;
    this.exports.ent_validate(alpha, evalPtr);
    const view = new DataView(this.memory.buffer);

    const statusMap = ['PASS', 'WARN', 'FAIL'];
    return {
      overallStatus: statusMap[view.getUint8(evalPtr)] || 'UNKNOWN',
      entropyStatus: statusMap[view.getUint8(evalPtr + 1)] || 'UNKNOWN',
      chiSquareStatus: statusMap[view.getUint8(evalPtr + 2)] || 'UNKNOWN',
      meanStatus: statusMap[view.getUint8(evalPtr + 3)] || 'UNKNOWN',
      piStatus: statusMap[view.getUint8(evalPtr + 4)] || 'UNKNOWN',
      serialCorrelationStatus: statusMap[view.getUint8(evalPtr + 5)] || 'UNKNOWN',
      chiSquareLowerBound: view.getFloat64(evalPtr + 8, true),
      chiSquareUpperBound: view.getFloat64(evalPtr + 16, true),
      pochisqExceedProb: view.getFloat64(evalPtr + 24, true),
      piErrorPercent: view.getFloat64(evalPtr + 32, true),
    };
  }
}

const EntWasmRunner = EntRunner;

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { EntRunner, EntWasmRunner };
}
