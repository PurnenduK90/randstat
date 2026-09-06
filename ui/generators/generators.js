/**
 * WASM Test Stream Generator & Formatter Runner.
 * High-performance, zero-allocation generation of test vectors directly in browser memory.
 */
class GeneratorRunner {
  constructor(wasmInstance) {
    this.exports = wasmInstance.exports;
    this.memory = this.exports.memory;
  }

  static async load(wasmSource = 'randstat-generators.wasm') {
    let bytes;
    if (typeof wasmSource === 'string') {
      const response = await fetch(wasmSource);
      bytes = await response.arrayBuffer();
    } else {
      bytes = wasmSource;
    }
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new GeneratorRunner(instance);
  }

  /**
   * Initializes generator.
   * @param {number} genType - 0:Seq, 1:Fixed, 2:LFSR, 3:DeBruijn, 4:LCG, 5:Xoshiro, 6:Gaussian, 7:Poisson, 8:AES, 9:SHA, 10:ChaCha
   * @param {number} p1 - First param
   * @param {number} p2 - Second param
   * @param {bigint|number} seedLo - Low 64 bits of seed
   * @param {bigint|number} seedHi - High 64 bits of seed
   */
  init(genType = 0, p1 = 0, p2 = 1, seedLo = 0n, seedHi = 0n) {
    const sLo = BigInt(seedLo);
    const sHi = BigInt(seedHi);
    return this.exports.generator_init(genType, p1, p2, sLo, sHi) === 1;
  }

  reset() {
    this.exports.generator_reset();
  }

  /**
   * Generates a raw byte chunk up to 65536 bytes in WASM memory.
   * Returns a copy of the Uint8Array.
   */
  generateChunk(size = 65536) {
    const chunkPtr = this.exports.generator_get_chunk_ptr();
    const capacity = this.exports.generator_get_chunk_capacity();
    const len = Math.min(size, capacity);
    const written = this.exports.generator_fill_chunk(len);
    return new Uint8Array(this.memory.buffer, chunkPtr, written).slice();
  }

  /**
   * Formats a raw chunk of size `rawLen` into `formatCode` (0:Binary, 1:AsciiInt, 2:AsciiBits, 3:Hex, 4:Float).
   * Returns a copy of the formatted Uint8Array.
   */
  formatChunk(rawLen, formatCode = 0) {
    if (formatCode === 0) {
      return this.generateChunk(rawLen);
    }
    const chunkPtr = this.exports.generator_get_chunk_ptr();
    const formattedPtr = this.exports.generator_get_formatted_ptr();
    const capacity = this.exports.generator_get_chunk_capacity();
    const len = Math.min(rawLen, capacity);
    this.exports.generator_fill_chunk(len);
    const formattedLen = this.exports.generator_format_chunk(len, formatCode);
    return new Uint8Array(this.memory.buffer, formattedPtr, formattedLen).slice();
  }

  /**
   * Generates a complete Blob for downloading or processing.
   * Uses chunked streaming to support large files without memory exhaustion.
   */
  async createDownloadBlob(totalBytes, formatCode = 0, onProgress = null) {
    const chunkSize = 65536;
    const parts = [];
    let remaining = totalBytes;
    let generated = 0;

    while (remaining > 0) {
      const take = Math.min(remaining, chunkSize);
      const chunk = this.formatChunk(take, formatCode);
      parts.push(chunk);
      remaining -= take;
      generated += take;

      if (onProgress) {
        onProgress(generated, totalBytes);
      }
      if (remaining > 0 && generated % (chunkSize * 16) === 0) {
        // Yield to event loop
        await new Promise((r) => setTimeout(r, 0));
      }
    }

    const mimeType = formatCode === 0 ? 'application/octet-stream' : 'text/plain;charset=utf-8';
    return new Blob(parts, { type: mimeType });
  }
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { GeneratorRunner };
}
