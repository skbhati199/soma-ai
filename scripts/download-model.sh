#!/usr/bin/env bash
# Download model for development/testing

set -euo pipefail

MODEL_NAME="${1:-qwen2.5-coder-1.5b-instruct-q4_k_m.gguf}"
CACHE_DIR="${HOME}/.cache/soma/models"

mkdir -p "${CACHE_DIR}"

MODEL_URL="https://huggingface.co/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/${MODEL_NAME}"

echo "Downloading ${MODEL_NAME} to ${CACHE_DIR}..."

if command -v wget &>/dev/null; then
    wget -c -O "${CACHE_DIR}/${MODEL_NAME}" "${MODEL_URL}"
elif command -v curl &>/dev/null; then
    curl -L -C - -o "${CACHE_DIR}/${MODEL_NAME}" "${MODEL_URL}"
else
    echo "Error: wget or curl required"
    exit 1
fi

echo "Download complete: ${CACHE_DIR}/${MODEL_NAME}"