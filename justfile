# randstat workspace — convenience build recipes
# Install: cargo install just
# Usage:   just build-dist

WASM_TARGET := "wasm32-unknown-unknown"
WASM_OUT    := "target/" + WASM_TARGET + "/release"

# List all available recipes
default:
    @just --list

# ── Workspace ──────────────────────────────────────────────────────────────────

# Check all crates compile cleanly
check:
    cargo check --workspace

# Run all tests in the workspace
test:
    cargo test --workspace

# Check all no_std crates on wasm32 target
check-wasm:
    cargo check -p randstat-core          --target {{WASM_TARGET}}
    cargo check -p randstat-tests         --target {{WASM_TARGET}}
    cargo check -p randstat-suite-ent     --target {{WASM_TARGET}}
    cargo check -p randstat-suite-nist    --target {{WASM_TARGET}}
    cargo check -p randstat-suite-sp800-90b --target {{WASM_TARGET}}
    cargo check -p randstat-suite-ais31   --target {{WASM_TARGET}}
    cargo check -p randstat-suite-dieharder --target {{WASM_TARGET}}
    cargo check -p randstat-suite-testu01 --target {{WASM_TARGET}}
    cargo check -p randstat-suite-practrand --target {{WASM_TARGET}}
    cargo check -p randstat-suite-gjrand  --target {{WASM_TARGET}}
    cargo check -p randstat-suite-full    --target {{WASM_TARGET}}

# ── WASM builds ────────────────────────────────────────────────────────────────

# Build math-only WASM (~3-5 KB) — PDF/CDF/plot generators only
build-wasm-math:
    cargo build -p randstat-wasm \
        --no-default-features --features math-only \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build ENT WASM (~15-18 KB) — Shannon + Monte Carlo + Serial Corr
build-wasm-ent:
    cargo build -p randstat-wasm \
        --no-default-features --features ent \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build NIST WASM (~30 KB) — all 15 NIST SP800-22 tests
build-wasm-nist:
    cargo build -p randstat-wasm \
        --no-default-features --features nist \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build AIS 31 WASM (~20 KB) — BSI AIS 20 / AIS 31 TRNG test battery
build-wasm-ais31:
    cargo build -p randstat-wasm \
        --no-default-features --features ais31 \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build SP800-90B WASM (~20 KB) — NIST SP800-90B min-entropy battery
build-wasm-sp800-90b:
    cargo build -p randstat-wasm \
        --no-default-features --features sp800-90b \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build Dieharder WASM (~25 KB) — Dieharder battery
build-wasm-dieharder:
    cargo build -p randstat-wasm \
        --no-default-features --features dieharder \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build Generators WASM (~10-15 KB) — 11 generator families + formatters
build-wasm-generators:
    cargo build -p randstat-wasm \
        --no-default-features --features generators \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build Full WASM (~35 KB) — all suites combined
build-wasm-full:
    cargo build -p randstat-wasm \
        --no-default-features --features full \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build ALL WASM variants in sequence
build-wasm-all: build-wasm-math build-wasm-ent build-wasm-nist build-wasm-ais31 build-wasm-sp800-90b build-wasm-dieharder build-wasm-generators build-wasm-full
    @echo "All WASM variants built."

# ── CLI ────────────────────────────────────────────────────────────────────────

# Run the CLI in development mode (pass args after --)
run *ARGS:
    cargo run -p randstat-cli -- {{ARGS}}

# Run the CLI stream generator (pass generator args)
generate *ARGS:
    cargo run -p randstat-cli -- generate {{ARGS}}

# Install CLI binary to ~/.cargo/bin/randstat
install:
    cargo install --path apps/randstat-cli

# ── CI & Distribution ──────────────────────────────────────────────────────────

# Check all source files are formatted
fmt:
    cargo fmt --all --check

# Run clippy on all workspace packages
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Run all tests and measure line coverage (min 60%)
cov:
    cargo llvm-cov --workspace --all-features --fail-under-lines 60

# Full CI gate: check format + lint + check + wasm check + test coverage
ci: fmt clippy check check-wasm cov
    @echo "CI passed."

# Package independent WASM binaries, minified JS SDKs, and READMEs into dist/
build-dist:
    python scripts/build_dist.py
