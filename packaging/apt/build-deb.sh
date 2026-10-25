#!/bin/bash
# build-deb.sh - Build .deb package for APT

set -euo pipefail

VERSION="0.1.0"
PACKAGE="soma-ai"
ARCH="amd64"
MAINTAINER="skbhati199 <skbhati199@gmail.com>"

BUILD_DIR="build-deb"
DEB_DIR="${BUILD_DIR}/${PACKAGE}_${VERSION}-1_${ARCH}"
DEBIAN_DIR="${DEB_DIR}/DEBIAN"
BIN_DIR="${DEB_DIR}/usr/local/bin"
DOC_DIR="${DEB_DIR}/usr/share/doc/${PACKAGE}"

# Clean and create directories
rm -rf "${BUILD_DIR}"
mkdir -p "${DEBIAN_DIR}" "${BIN_DIR}" "${DOC_DIR}"

# Copy binary
cp target/x86_64-unknown-linux-gnu/release/soma "${BIN_DIR}/"
chmod 755 "${BIN_DIR}/soma"

# Generate control file
cat > "${DEBIAN_DIR}/control" <<EOF
Package: ${PACKAGE}
Version: ${VERSION}-1
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: ${MAINTAINER}
Description: Local AI terminal command assistant
 Soma AI is a single-binary, local AI command assistant for your terminal.
 Works offline after first model download. Supports PowerShell, Bash, Zsh, and Fish.
Homepage: https://github.com/skbhati199/soma-ai
License: MIT
EOF

# Generate postinst
cat > "${DEBIAN_DIR}/postinst" <<'EOF'
#!/bin/bash
set -e
echo "Soma AI installed successfully."
echo "Run 'soma install' to set up shell integration."
EOF
chmod 755 "${DEBIAN_DIR}/postinst"

# Copy docs
cp README.md "${DOC_DIR}/"
cp LICENSE "${DOC_DIR}/copyright"

# Build .deb
dpkg-deb --build "${DEB_DIR}"
mv "${DEB_DIR}.deb" "${PACKAGE}_${VERSION}-1_${ARCH}.deb"

echo "Built: ${PACKAGE}_${VERSION}-1_${ARCH}.deb"
