#!/usr/bin/env bash
# Build release binaries for all targets

set -euo pipefail

VERSION="${1:-$(cargo metadata --format-version=1 | jq -r '.packages[] | select(.name=="soma") | .version')}"

echo "Building Soma AI v${VERSION}"

# Build for all targets
targets=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "x86_64-pc-windows-msvc"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

mkdir -p release

for target in "${targets[@]}"; do
    echo "Building for ${target}..."
    
    if [[ "${target}" == *"musl"* ]]; then
        # Need musl tools
        if ! command -v musl-gcc &>/dev/null; then
            echo "Skipping ${target}: musl-gcc not found"
            continue
        fi
    fi
    
    if [[ "${target}" == *"windows"* ]]; then
        if ! rustup target list --installed | grep -q "${target}"; then
            echo "Skipping ${target}: target not installed"
            continue
        fi
    fi
    
    cargo build --release --target "${target}" --features inference,clipboard,completions
    
    # Copy artifact
    artifact="soma"
    if [[ "${target}" == *"windows"* ]]; then
        artifact="soma.exe"
    fi
    
    cp "target/${target}/release/${artifact}" "release/${artifact}-${target}"
    
    echo "Built: release/${artifact}-${target}"
done

# Create archives
cd release
for file in soma*; do
    if [[ "${file}" == *.exe ]]; then
        zip -9 "${file%.exe}.zip" "${file}"
    else
        tar -czf "${file}.tar.gz" "${file}"
    fi
done

# Generate checksums
sha256sum * > checksums.txt

echo ""
echo "Release artifacts in release/:"
ls -la
