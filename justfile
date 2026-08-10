# randstat workspace — convenience build recipes
# Install: cargo install just
# Usage:   just build-wasm-all

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
    cargo check -p randstat-suite-quick   --target {{WASM_TARGET}}
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

# Build Quick WASM (~12-15 KB) — 4-test fast screening suite
build-wasm-quick:
    cargo build -p randstat-wasm \
        --no-default-features --features quick \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build NIST WASM (~30 KB) — all 15 NIST SP800-22 tests
build-wasm-nist:
    cargo build -p randstat-wasm \
        --no-default-features --features nist \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build Full WASM (~35 KB) — ENT + NIST combined
build-wasm-full:
    cargo build -p randstat-wasm \
        --no-default-features --features full \
        --target {{WASM_TARGET}} --release
    @echo "Output: {{WASM_OUT}}/randstat_wasm.wasm"

# Build ALL WASM variants in sequence
build-wasm-all: build-wasm-math build-wasm-ent build-wasm-quick build-wasm-nist build-wasm-full
    @echo "All WASM variants built."

# ── CLI ────────────────────────────────────────────────────────────────────────

# Run the CLI in development mode (pass args after --)
run *ARGS:
    cargo run -p randstat-cli -- {{ARGS}}

# Install CLI binary to ~/.cargo/bin/randstat
install:
    cargo install --path apps/randstat-cli

# ── CI ─────────────────────────────────────────────────────────────────────────

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

# Concatenate and minify WASM and JS files into dist/
build-dist:
    python scripts/build_dist.py
