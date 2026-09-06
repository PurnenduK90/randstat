/**
 * Generic WASM Instance Loader and Memory Bridge.
 * Loads a WASM module from URL or buffer, manages linear memory chunk transfers.
 */
class WasmBridge {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  /**
   * Instantiates WASM binary module from URL or ArrayBuffer.
   */
  static async load(wasmSource) {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new WasmBridge(instance);
  }

  /** Write a Uint8Array chunk into WASM linear memory at `bufferPtr` */
  writeChunk(chunk, bufferPtr = 1024) {
    if (!chunk || chunk.length === 0) return 0;
    const len = chunk.length;
    const wasmView = new Uint8Array(this.memory.buffer, bufferPtr, len);
    wasmView.set(chunk);
    return len;
  }

  /** Read a 32-byte hex string from `ptr` */
  readSha256Hex(ptr) {
    const bytes = new Uint8Array(this.memory.buffer, ptr, 32);
    let hex = '';
    for (let i = 0; i < 32; i++) {
      hex += bytes[i].toString(16).padStart(2, '0');
    }
    return hex;
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { WasmBridge };
}
