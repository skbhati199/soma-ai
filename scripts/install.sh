#!/usr/bin/env bash
# install.sh - Universal installer for Soma AI
# Usage: curl -fsSL https://raw.githubusercontent.com/skbhati199/soma-ai/main/scripts/install.sh | bash

set -euo pipefail

REPO="skbhati199/soma-ai"
VERSION="${SOM_VERSION:-latest}"
INSTALL_DIR="${SOM_INSTALL_DIR:-$HOME/.local/bin}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

detect_platform() {
  local os=$(uname -s | tr '[:upper:]' '[:lower:]')
  local arch=$(uname -m)
  
  case "$os" in
    linux)
      case "$arch" in
        x86_64) echo "x86_64-unknown-linux-gnu" ;;
        aarch64) echo "aarch64-unknown-linux-gnu" ;;
        *) log_error "Unsupported architecture: $arch"; exit 1 ;;
      esac
      ;;
    darwin)
      case "$arch" in
        x86_64) echo "x86_64-apple-darwin" ;;
        arm64) echo "aarch64-apple-darwin" ;;
        *) log_error "Unsupported architecture: $arch"; exit 1 ;;
      esac
      ;;
    *)
      log_error "Unsupported OS: $os"
      exit 1
      ;;
  esac
}

get_latest_version() {
  curl -s "https://api.github.com/repos/$REPO/releases/latest" | \
    grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

download_and_install() {
  local platform=$1
  local version=$2
  
  local asset_name="soma-${platform}.tar.gz"
  local url="https://github.com/${REPO}/releases/download/${version}/${asset_name}"
  
  log_info "Downloading ${asset_name}..."
  log_info "From: ${url}"
  
  local tmpdir=$(mktemp -d)
  trap "rm -rf ${tmpdir}" EXIT
  
  if ! curl -fsSL -o "${tmpdir}/${asset_name}" "${url}"; then
    log_error "Failed to download ${asset_name}"
    log_error "Check if version ${version} exists for platform ${platform}"
    exit 1
  fi
  
  log_info "Extracting..."
  tar -xzf "${tmpdir}/${asset_name}" -C "${tmpdir}"
  
  log_info "Installing to ${INSTALL_DIR}..."
  mkdir -p "${INSTALL_DIR}"
  cp "${tmpdir}/soma" "${INSTALL_DIR}/soma"
  chmod +x "${INSTALL_DIR}/soma"
  
  log_info "✓ Installed soma to ${INSTALL_DIR}/soma"
}

main() {
  log_info "Installing Soma AI..."
  
  # Detect platform
  local platform=$(detect_platform)
  log_info "Detected platform: ${platform}"
  
  # Get version
  if [ "$VERSION" = "latest" ]; then
    VERSION=$(get_latest_version)
    log_info "Latest version: ${VERSION}"
  fi
  
  # Download and install
  download_and_install "${platform}" "${VERSION}"
  
  # Verify
  if command -v soma &>/dev/null; then
    log_info "✓ Installation verified: $(soma --version)"
  else
    log_warn "soma not in PATH. Add ${INSTALL_DIR} to your PATH:"
    log_warn "  export PATH=\"\$PATH:${INSTALL_DIR}\""
  fi
  
  log_info ""
  log_info "Next steps:"
  log_info "  1. Restart your shell or run: source ~/.bashrc"
  log_info "  2. Install shell integration: soma install"
  log_info "  3. Try: soma ask \"list all files\""
}

main
