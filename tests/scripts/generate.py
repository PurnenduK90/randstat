import os
import sys
import random
import secrets
import argparse
import math

CHUNK_SIZE = 16 * 1024 * 1024  # 16 MB streaming chunk size

# Primitive polynomials for LFSR and De Bruijn generators
TAPS = {
    3: [3, 1],
    4: [4, 1],
    8: [8, 6, 5, 1],
    9: [9, 4],
    11: [11, 2],
    13: [13, 4, 3, 1],
    19: [19, 5, 2, 1],
    27: [27, 5, 2, 1]
}

def format_size(num_bytes):
    """Formats bytes into human readable string (e.g. 10MB, 1GB)."""
    if num_bytes >= 1024 * 1024 * 1024:
        return f"{num_bytes / (1024 * 1024 * 1024):.1f}GB".replace(".0GB", "GB")
    elif num_bytes >= 1024 * 1024:
        return f"{num_bytes / (1024 * 1024):.1f}MB".replace(".0MB", "MB")
    elif num_bytes >= 1024:
        return f"{num_bytes / 1024:.1f}KB".replace(".0KB", "KB")
    return f"{num_bytes}B"

def parse_size_string(size_str, default_to_mb=False):
    """Parses size string like '100M', '1G', '1024' into byte integer."""
    if size_str is None:
        return None
    if isinstance(size_str, int):
        return size_str
    size_str = str(size_str).strip().upper()
    units = {'K': 1024, 'M': 1024**2, 'G': 1024**3, 'T': 1024**4}
    
    if size_str.endswith("KB"):
        return int(float(size_str[:-2]) * 1024)
    elif size_str.endswith("MB"):
        return int(float(size_str[:-2]) * 1024**2)
    elif size_str.endswith("GB"):
        return int(float(size_str[:-2]) * 1024**3)
    elif size_str.endswith("TB"):
        return int(float(size_str[:-2]) * 1024**4)
    elif size_str[-1] in units:
        unit = units[size_str[-1]]
        return int(float(size_str[:-1]) * unit)
    else:
        try:
            val = float(size_str)
            if default_to_mb:
                return int(val * 1024 * 1024)
            return int(val)
        except ValueError:
            return None

def generate_zeroes_file(size, output_dir, verbose=True):
    """Generate a file filled with zeroes in 16MB streaming chunks."""
    filename = f"zeroes_{format_size(size)}.bin"
    file_path = os.path.join(output_dir, filename)
    if os.path.exists(file_path):
        if verbose:
            print(f"Skipping {filename} ({format_size(size)}) - already exists.")
        return file_path
    if verbose:
        print(f"Creating {filename} ({format_size(size)})...", end="", flush=True)

    zero_chunk = b"\x00" * CHUNK_SIZE
    bytes_remaining = size
    with open(file_path, "wb") as f:
        while bytes_remaining > 0:
            write_size = min(bytes_remaining, CHUNK_SIZE)
            f.write(zero_chunk[:write_size])
            bytes_remaining -= write_size

    if verbose:
        print(" Done!")
    return file_path

def generate_sequential_file(size, output_dir, verbose=True):
    """Generate a file with sequential byte values (0-255) in 16MB streaming chunks."""
    filename = f"sequential_{format_size(size)}.bin"
    file_path = os.path.join(output_dir, filename)
    if os.path.exists(file_path):
        if verbose:
            print(f"Skipping {filename} ({format_size(size)}) - already exists.")
        return file_path
    if verbose:
        print(f"Creating {filename} ({format_size(size)})...", end="", flush=True)

    base_pattern = bytes(range(256))
    pattern_chunk = base_pattern * (CHUNK_SIZE // 256)

    bytes_remaining = size
    with open(file_path, "wb") as f:
        while bytes_remaining > 0:
            write_size = min(bytes_remaining, len(pattern_chunk))
            f.write(pattern_chunk[:write_size])
            bytes_remaining -= write_size

    if verbose:
        print(" Done!")
    return file_path

def generate_pseudorandom_file(size, output_dir, verbose=True):
    """Generate a file with pseudo-random bytes in 16MB streaming chunks."""
    filename = f"pseudorandom_{format_size(size)}.bin"
    file_path = os.path.join(output_dir, filename)
    if os.path.exists(file_path):
        if verbose:
            print(f"Skipping {filename} ({format_size(size)}) - already exists.")
        return file_path
    if verbose:
        print(f"Creating {filename} ({format_size(size)})...", end="", flush=True)

    bytes_remaining = size
    with open(file_path, "wb") as f:
        while bytes_remaining > 0:
            write_size = min(bytes_remaining, CHUNK_SIZE)
            if hasattr(random, 'randbytes'):
                chunk = random.randbytes(write_size)
            else:
                chunk = os.urandom(write_size)
            f.write(chunk)
            bytes_remaining -= write_size

    if verbose:
        print(" Done!")
    return file_path

def generate_cryptorandom_file(size, output_dir, verbose=True):
    """Generate a file with cryptographically secure random bytes in 16MB streaming chunks."""
    filename = f"cryptorandom_{format_size(size)}.bin"
    file_path = os.path.join(output_dir, filename)
    if os.path.exists(file_path):
        if verbose:
            print(f"Skipping {filename} ({format_size(size)}) - already exists.")
        return file_path
    if verbose:
        print(f"Creating {filename} ({format_size(size)})...", end="", flush=True)

    bytes_remaining = size
    with open(file_path, "wb") as f:
        while bytes_remaining > 0:
            write_size = min(bytes_remaining, CHUNK_SIZE)
            chunk = secrets.token_bytes(write_size)
            f.write(chunk)
            bytes_remaining -= write_size

    if verbose:
        print(" Done!")
    return file_path

def generate_lfsr_bytes_chunk(size_bytes, order):
    """Generate raw LFSR bytes for a given size and order."""
    buf = bytearray(size_bytes)
    state = 1
    if order == 3:
        for i in range(size_bytes):
            b = 0
            for shift in (7, 6, 5, 4, 3, 2, 1, 0):
                fb = (state ^ (state >> 2)) & 1
                b |= ((state & 1) << shift)
                state = (state >> 1) | (fb << 2)
            buf[i] = b
    elif order == 8:
        for i in range(size_bytes):
            b = 0
            for shift in (7, 6, 5, 4, 3, 2, 1, 0):
                fb = (state ^ (state >> 2) ^ (state >> 3) ^ (state >> 7)) & 1
                b |= ((state & 1) << shift)
                state = (state >> 1) | (fb << 7)
            buf[i] = b
    elif order == 11:
        for i in range(size_bytes):
            b = 0
            for shift in (7, 6, 5, 4, 3, 2, 1, 0):
                fb = (state ^ (state >> 9)) & 1
                b |= ((state & 1) << shift)
                state = (state >> 1) | (fb << 10)
            buf[i] = b
    elif order == 27:
        for i in range(size_bytes):
            b = 0
            for shift in (7, 6, 5, 4, 3, 2, 1, 0):
                fb = (state ^ (state >> 22) ^ (state >> 25) ^ (state >> 26)) & 1
                b |= ((state & 1) << shift)
                state = (state >> 1) | (fb << 26)
            buf[i] = b
    return bytes(buf)

def generate_lfsr_file(size, order, output_dir, verbose=True):
    """Generate an LFSR sequence file, streaming or tiling based on order period."""
    filename = f"lfsr_order{order}_{format_size(size)}.bin"
    file_path = os.path.join(output_dir, filename)
    if os.path.exists(file_path):
        if verbose:
            print(f"Skipping {filename} ({format_size(size)}) - already exists.")
        return file_path
    if verbose:
        print(f"Creating {filename} ({format_size(size)})...", end="", flush=True)

    period_bits = (1 << order) - 1
    period_bytes = period_bits

    if period_bytes < 4 * 1024 * 1024:
        # For small periods, tile the full period to preserve exact periodicity
        period_data = generate_lfsr_bytes_chunk(period_bytes, order)
        bytes_remaining = size
        with open(file_path, "wb") as f:
            while bytes_remaining > 0:
                chunk_periods = max(1, CHUNK_SIZE // len(period_data))
                chunk_size = chunk_periods * len(period_data)
                write_size = min(bytes_remaining, chunk_size)
                chunk = period_data * chunk_periods
                f.write(chunk[:write_size])
                bytes_remaining -= write_size
    else:
        # For large periods (like order 27), stream sequentially using O(1) memory
        state = 1
        bytes_remaining = size
        chunk_size = 1024 * 1024  # 1MB chunks
        with open(file_path, "wb") as f:
            while bytes_remaining > 0:
                write_size = min(bytes_remaining, chunk_size)
                chunk = bytearray(write_size)
                for i in range(write_size):
                    b = 0
                    for shift in (7, 6, 5, 4, 3, 2, 1, 0):
                        fb = (state ^ (state >> 22) ^ (state >> 25) ^ (state >> 26)) & 1
                        b |= ((state & 1) << shift)
                        state = (state >> 1) | (fb << 26)
                    chunk[i] = b
                f.write(chunk)
                bytes_remaining -= write_size

    if verbose:
        print(" Done!")
    return file_path

def generate_debruijn_period_bytes(order):
    """Generate the full De Bruijn sequence period for a given order, packed into bytes."""
    taps = TAPS[order]
    state = 1
    period = (1 << order) - 1
    bits = []

    if order == 4:
        for _ in range(period):
            bits.append(state & 1)
            fb = (state ^ (state >> 3)) & 1
            state = (state >> 1) | (fb << 3)
    elif order == 9:
        for _ in range(period):
            bits.append(state & 1)
            fb = (state ^ (state >> 5)) & 1
            state = (state >> 1) | (fb << 8)
    elif order == 13:
        for _ in range(period):
            bits.append(state & 1)
            fb = (state ^ (state >> 9) ^ (state >> 10) ^ (state >> 12)) & 1
            state = (state >> 1) | (fb << 12)
    elif order == 19:
        for _ in range(period):
            bits.append(state & 1)
            fb = (state ^ (state >> 14) ^ (state >> 17) ^ (state >> 18)) & 1
            state = (state >> 1) | (fb << 18)

    # Insert extra 0 into the run of (order-1) zeros to complete the De Bruijn sequence
    pattern = [1] + [0] * (order - 1) + [1]
    double_list = bits + bits
    found_idx = -1
    p_len = len(pattern)
    for i in range(len(bits)):
        if double_list[i:i+p_len] == pattern:
            found_idx = i
            break

    if found_idx != -1:
        insert_pos = (found_idx + 1 + (order - 1)) % len(bits)
        bits.insert(insert_pos, 0)
    else:
        bits.append(0)

    # Pack to bytes
    out_bytes = bytearray()
    for i in range(0, len(bits), 8):
        b = 0
        for j in range(8):
            if i + j < len(bits):
                b |= (bits[i + j] << (7 - j))
        out_bytes.append(b)
    return bytes(out_bytes)

def generate_debruijn_file(size, order, output_dir, verbose=True):
    """Generate a De Bruijn sequence file by tiling the pre-generated period."""
    filename = f"debruijn_order{order}_{format_size(size)}.bin"
    file_path = os.path.join(output_dir, filename)
    if os.path.exists(file_path):
        if verbose:
            print(f"Skipping {filename} ({format_size(size)}) - already exists.")
        return file_path
    if verbose:
        print(f"Creating {filename} ({format_size(size)})...", end="", flush=True)

    period_bytes = generate_debruijn_period_bytes(order)
    bytes_remaining = size
    with open(file_path, "wb") as f:
        while bytes_remaining > 0:
            chunk_periods = max(1, CHUNK_SIZE // len(period_bytes))
            chunk_size = chunk_periods * len(period_bytes)
            write_size = min(bytes_remaining, chunk_size)
            chunk = period_bytes * chunk_periods
            f.write(chunk[:write_size])
            bytes_remaining -= write_size

    if verbose:
        print(" Done!")
    return file_path

def generate_gaussian_buffer(buffer_size):
    """Generate Gaussian distributed bytes using the Box-Muller transform."""
    buf = bytearray(buffer_size)
    for i in range(0, buffer_size, 2):
        u1 = random.random()
        u2 = random.random()
        if u1 < 1e-9:
            u1 = 1e-9
        r = math.sqrt(-2.0 * math.log(u1))
        theta = 2.0 * math.pi * u2
        z0 = r * math.cos(theta)
        z1 = r * math.sin(theta)

        val0 = int(z0 * 30.0 + 127.5)
        val0 = max(0, min(255, val0))
        buf[i] = val0

        if i + 1 < buffer_size:
            val1 = int(z1 * 30.0 + 127.5)
            val1 = max(0, min(255, val1))
            buf[i+1] = val1
    return bytes(buf)

def generate_gaussian_file(size, output_dir, verbose=True):
    """Generate a Gaussian distribution file by repeating a pre-generated buffer."""
    filename = f"gaussian_{format_size(size)}.bin"
    file_path = os.path.join(output_dir, filename)
    if os.path.exists(file_path):
        if verbose:
            print(f"Skipping {filename} ({format_size(size)}) - already exists.")
        return file_path
    if verbose:
        print(f"Creating {filename} ({format_size(size)})...", end="", flush=True)

    ref_size = min(size, 4 * 1024 * 1024)
    ref_buffer = generate_gaussian_buffer(ref_size)

    bytes_remaining = size
    with open(file_path, "wb") as f:
        while bytes_remaining > 0:
            write_size = min(bytes_remaining, len(ref_buffer))
            f.write(ref_buffer[:write_size])
            bytes_remaining -= write_size

    if verbose:
        print(" Done!")
    return file_path

def generate_poisson_buffer(buffer_size):
    """Generate Poisson distributed bytes using Gaussian approximation (lambda = 127)."""
    buf = bytearray(buffer_size)
    mean = 127.0
    std_dev = math.sqrt(mean)  # ~11.269
    for i in range(0, buffer_size, 2):
        u1 = random.random()
        u2 = random.random()
        if u1 < 1e-9:
            u1 = 1e-9
        r = math.sqrt(-2.0 * math.log(u1))
        theta = 2.0 * math.pi * u2
        z0 = r * math.cos(theta)
        z1 = r * math.sin(theta)

        val0 = int(z0 * std_dev + mean)
        val0 = max(0, min(255, val0))
        buf[i] = val0

        if i + 1 < buffer_size:
            val1 = int(z1 * std_dev + mean)
            val1 = max(0, min(255, val1))
            buf[i+1] = val1
    return bytes(buf)

def generate_poisson_file(size, output_dir, verbose=True):
    """Generate a Poisson distribution file by repeating a pre-generated buffer."""
    filename = f"poisson_{format_size(size)}.bin"
    file_path = os.path.join(output_dir, filename)
    if os.path.exists(file_path):
        if verbose:
            print(f"Skipping {filename} ({format_size(size)}) - already exists.")
        return file_path
    if verbose:
        print(f"Creating {filename} ({format_size(size)})...", end="", flush=True)

    ref_size = min(size, 4 * 1024 * 1024)
    ref_buffer = generate_poisson_buffer(ref_size)

    bytes_remaining = size
    with open(file_path, "wb") as f:
        while bytes_remaining > 0:
            write_size = min(bytes_remaining, len(ref_buffer))
            f.write(ref_buffer[:write_size])
            bytes_remaining -= write_size

    if verbose:
        print(" Done!")
    return file_path

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    default_dir = os.path.abspath(os.path.join(script_dir, "..", "..", "testfiles"))

    parser = argparse.ArgumentParser(description="Streamed generator for entropy benchmark test files.")
    parser.add_argument("--size", type=str, default=None, help="Size of files to generate (e.g. 1kB, 1MB, 10MB, 1GB).")
    parser.add_argument("--min-size", "--minsize", type=str, default=None, help="Generate all preset sizes starting from this limit (e.g. 10, 10MB).")
    parser.add_argument("--max-size", "--maxsize", type=str, default=None, help="Generate all preset sizes up to this limit (e.g. 200, 200MB, 1GB).")
    parser.add_argument("--output-dir", type=str, default=default_dir, help=f"Directory to save generated files (default: {default_dir})")
    parser.add_argument("--verbose", action="store_true", default=True, help="Enable verbose progress output")
    parser.add_argument("--num-files", type=int, default=1, help="Number of files to generate per type (default: 1)")

    file_type_choices = ["zeroes", "sequential", "pseudorandom", "cryptorandom", "lfsr", "debruijn", "gaussian", "poisson", "poison"]
    default_file_types = ["zeroes", "sequential", "pseudorandom", "cryptorandom", "lfsr", "debruijn", "gaussian", "poisson"]

    parser.add_argument("--file-types", type=str, nargs="+", choices=file_type_choices, default=default_file_types, help="Types of files to generate")
    parser.add_argument("--size-min", type=str, default=None, help="Minimum size for random size range (e.g. 10K, 1M)")
    parser.add_argument("--size-max", type=str, default=None, help="Maximum size for random size range (e.g. 10M, 500M)")

    args = parser.parse_args()

    out_dir = args.output_dir
    os.makedirs(out_dir, exist_ok=True)

    # Fixed presets: 1kB, 1MB, 10MB, 50MB, 100MB, 200MB, 500MB, 1GB
    PRESETS = [
        (1024, "1kB"),
        (1024 * 1024, "1MB"),
        (10 * 1024 * 1024, "10MB"),
        (50 * 1024 * 1024, "50MB"),
        (100 * 1024 * 1024, "100MB"),
        (200 * 1024 * 1024, "200MB"),
        (500 * 1024 * 1024, "500MB"),
        (1024 * 1024 * 1024, "1GB")
    ]

    sizes_to_generate = []

    size_min_bytes = parse_size_string(args.size_min, default_to_mb=False)
    size_max_bytes = parse_size_string(args.size_max, default_to_mb=False)

    min_bytes = parse_size_string(args.min_size, default_to_mb=True)
    max_bytes = parse_size_string(args.max_size, default_to_mb=True)

    if size_min_bytes is not None and size_max_bytes is not None:
        for _ in range(args.num_files):
            curr_size = random.randint(size_min_bytes, size_max_bytes)
            sizes_to_generate.append((curr_size, format_size(curr_size)))
    elif min_bytes is not None or max_bytes is not None:
        effective_min = min_bytes if min_bytes is not None else 0
        effective_max = max_bytes if max_bytes is not None else 1024 * 1024 * 1024
        
        for p_bytes, p_label in PRESETS:
            if effective_min <= p_bytes <= effective_max:
                sizes_to_generate.append((p_bytes, p_label))
        
        if min_bytes is not None:
            is_min_preset = any(abs(p_bytes - min_bytes) < 8 for p_bytes, _ in PRESETS)
            if not is_min_preset and min_bytes <= effective_max:
                sizes_to_generate.append((min_bytes, format_size(min_bytes)))
                
        if max_bytes is not None:
            is_max_preset = any(abs(p_bytes - max_bytes) < 8 for p_bytes, _ in PRESETS)
            if not is_max_preset and max_bytes >= effective_min:
                sizes_to_generate.append((max_bytes, format_size(max_bytes)))
                
        # Sort by size to keep them in order and remove duplicates
        sizes_to_generate = sorted(list(set(sizes_to_generate)), key=lambda x: x[0])
    elif args.size:
        size_bytes = parse_size_string(args.size, default_to_mb=False)
        sizes_to_generate.append((size_bytes, format_size(size_bytes)))
    else:
        # Fallback to 1MB if no size or limit is specified
        sizes_to_generate.append((1024 * 1024, "1MB"))

    for file_type in args.file_types:
        for size_bytes, _ in sizes_to_generate:
            if file_type == "zeroes":
                generate_zeroes_file(size_bytes, out_dir, args.verbose)
            elif file_type == "sequential":
                generate_sequential_file(size_bytes, out_dir, args.verbose)
            elif file_type == "pseudorandom":
                generate_pseudorandom_file(size_bytes, out_dir, args.verbose)
            elif file_type == "cryptorandom":
                generate_cryptorandom_file(size_bytes, out_dir, args.verbose)
            elif file_type == "lfsr":
                for order in [3, 8, 11, 27]:
                    generate_lfsr_file(size_bytes, order, out_dir, args.verbose)
            elif file_type == "debruijn":
                for order in [4, 9, 13, 19]:
                    generate_debruijn_file(size_bytes, order, out_dir, args.verbose)
            elif file_type == "gaussian":
                generate_gaussian_file(size_bytes, out_dir, args.verbose)
            elif file_type in ("poisson", "poison"):
                generate_poisson_file(size_bytes, out_dir, args.verbose)

if __name__ == "__main__":
    main()