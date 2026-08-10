/**
 * WASM ENT calculation runner for browser and WebAssembly environments.
 * Stream-evaluates byte arrays or File chunks via WASM linear memory state.
 */
class EntWasmRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  /**
   * Instantiates WASM binary module from URL or ArrayBuffer.
   */
  static async load(wasmSource = 'entropy.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new EntWasmRunner(instance);
  }

  /** Resets state accumulators inside WASM linear memory */
  reset() {
    this.exports.ent_reset();
  }

  /** Passes a chunk of Uint8Array data to WASM for evaluation */
  update(chunk) {
    if (!chunk || chunk.length === 0) return;
    const len = chunk.length;
    // Use bufferPtr inside WASM linear memory
    const bufferPtr = 1024;
    const wasmView = new Uint8Array(this.memory.buffer, bufferPtr, len);
    wasmView.set(chunk);
    this.exports.ent_update(bufferPtr, len);
  }

  /** Finalizes accumulator pass and reads EntResult struct from linear memory */
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
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { EntWasmRunner };
}