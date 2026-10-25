# Soma AI

<div align="center">

![Soma AI](https://img.shields.io/badge/Soma-AI%20Terminal%20Assistant-blue?style=for-the-badge)
![Version](https://img.shields.io/github/v/release/skbhati199/soma-ai?style=for-the-badge)
![License](https://img.shields.io/github/license/skbhati199/soma-ai?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)

**A single-binary, local AI command assistant for your terminal.**

Works offline after first model download. Supports PowerShell, Bash, Zsh, and Fish.

</div>

---

## 🎯 Features

| Feature | Description |
|---------|-------------|
| **Single Binary** | No runtime dependencies, just download and run |
| **Local LLM** | Uses GGUF models (qwen2.5-coder, phi3, etc.) via llama.cpp |
| **Offline-First** | Works completely offline after model download |
| **Multi-Shell** | PowerShell 7+, Bash, Zsh, Fish, cmd.exe |
| **Keybinding** | `Ctrl+Space` inserts AI command on next line |
| **Alias** | `?? "query"` for quick suggestions |
| **Auto-Install** | `soma install` sets up keybindings, aliases, completions |
| **Configurable** | TOML config with env var overrides |

---

## 🚀 Quick Start

### Install (Binary)

```bash
# Download latest release for your platform
# Linux
curl -L https://github.com/skbhati199/soma-ai/releases/latest/download/soma-x86_64-unknown-linux-gnu.tar.gz | tar xz

# macOS (Intel)
curl -L https://github.com/skbhati199/soma-ai/releases/latest/download/soma-x86_64-apple-darwin.tar.gz | tar xz

# macOS (Apple Silicon)
curl -L https://github.com/skbhati199/soma-ai/releases/latest/download/soma-aarch64-apple-darwin.tar.gz | tar xz

# Windows (PowerShell)
iwr -useb https://github.com/skbhati199/soma-ai/releases/latest/download/soma-x86_64-pc-windows-msvc.zip -outfile soma.zip
Expand-Archive soma.zip
```

### Install (Package Managers)

```bash
# Scoop (Windows)
scoop install soma

# Homebrew (macOS/Linux)
brew install soma

# Cargo (from source)
cargo install --git https://github.com/skbhati199/soma-ai --features inference,clipboard,completions
```

### Setup Shell Integration

```bash
# Auto-detect shell and install
soma install

# Or specify shell
soma install --shell pwsh    # PowerShell 7+
soma install --shell bash    # Bash
soma install --shell zsh     # Zsh
soma install --shell fish    # Fish
```

This adds:
- `Ctrl+Space` → Insert AI command on next line
- `?? "query"` → Quick ask (prints + copies to clipboard)
- Shell completions

Restart your shell or run `source ~/.bashrc` (or `. $PROFILE` in PowerShell).

---

## 💡 Usage

### Ask for Commands

```bash
# Print suggestion + copy to clipboard
soma ask "list all txt files recursively"
# → Get-ChildItem -Recurse -File -Filter '*.txt'

# Quick alias
?? "find all .txt files modified today"
```

### Insert on Next Line (Keybinding)

Press `Ctrl+Space`, type your query, press Enter:

```
PS C:\projects> █
# Ctrl+Space → "disk usage of current directory" → Enter

PS C:\projects>
Get-ChildItem -Recurse -File | Measure-Object -Property Length -Sum | Select-Object -ExpandProperty Sum
█
```

### Manage Models

```bash
# List downloaded models
soma model list

# Download a model
soma model pull qwen2.5-coder-1.5b-instruct-q4_k_m

# Remove a model
soma model remove qwen2.5-coder-1.5b-instruct-q4_k_m

# Show model info
soma model info
```

### Configuration

```bash
# Show current config
soma config show

# Edit in $EDITOR
soma config edit

# Get/set specific values
soma config get model.name
soma config set model.name "phi3-mini-4k-instruct-q4.gguf"
```

---

## 🔧 Configuration

Config file: `~/.config/soma/config.toml` (or `%APPDATA%\soma\config.toml` on Windows)

```toml
[model]
name = "qwen2.5-coder-1.5b-instruct-q4_k_m.gguf"
auto_download = true
context_size = 4096
threads = 0        # 0 = auto
gpu_layers = 0     # 0 = CPU only

[shell]
default = "auto"
keybind = "ctrl+space"
alias = "??"

[prompt]
system = "powershell"  # powershell, bash, generic
max_tokens = 512
temperature = 0.1

[output]
insert_mode = "next_line"
show_confirmation = true
copy_to_clipboard = true
```

Environment variables override config (prefix with `SOMA_`):
```bash
SOMA_MODEL_NAME="phi3-mini-4k-instruct-q4.gguf"
SOMA_SHELL_KEYBIND="alt+enter"
SOMA_PROMPT_TEMPERATURE=0.2
```

---

## 📦 Available Models

| Model | Size | Description |
|-------|------|-------------|
| `qwen2.5-coder-1.5b-instruct-q4_k_m` | ~1 GB | **Recommended** - Code-specialized, fast |
| `qwen2.5-coder-7b-instruct-q4_k_m` | ~4 GB | Better quality, slower |
| `phi3-mini-4k-instruct-q4` | ~2 GB | Microsoft's compact model |
| `codellama-7b-instruct-q4` | ~4 GB | Meta's code model |

Download: `soma model pull <model-name>`

---

## 🏗 Building from Source

### Prerequisites

- Rust 1.75+
- For `inference` feature: CMake, C++ compiler

```bash
# Clone
git clone https://github.com/skbhati199/soma-ai
cd soma-ai

# Build with all features
cargo build --release --features inference,clipboard,completions

# Binary at target/release/soma
```

### Features

| Feature | Description | Default |
|---------|-------------|---------|
| `inference` | Enable llama.cpp inference | ✅ |
| `clipboard` | Clipboard support (arboard) | ✅ |
| `completions` | Shell completions (clap_complete) | ✅ |
| `embedded-model` | Embed model in binary | ❌ |

### Cross-Compilation

```bash
# Linux (static musl)
cargo build --release --target x86_64-unknown-linux-musl

# Windows (from Linux)
cargo build --release --target x86_64-pc-windows-msvc

# macOS (universal)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create -output soma target/x86_64-apple-darwin/release/soma target/aarch64-apple-darwin/release/soma
```

---

## 📁 Project Structure

```
soma-ai/
├── src/
│   ├── main.rs                 # Entry point
│   ├── cli/                    # CLI args & completions
│   ├── config/                 # Configuration (figment)
│   ├── shell/                  # Shell detection & installer
│   ├── model/                  # Model management (download/verify)
│   ├── inference/              # LLM inference (llama.cpp)
│   ├── output/                 # Output handlers (insert/clipboard)
│   ├── commands/               # CLI subcommands
│   └── utils/                  # Logging, errors
├── assets/
│   ├── prompts/                # System prompt templates
│   └── default_config.toml
├── .github/workflows/          # CI/CD
└── Cargo.toml
```

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo fmt`, `cargo clippy`, `cargo test`
5. Submit a PR

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- [llama.cpp](https://github.com/ggerganov/llama.cpp) - Inference engine
- [Qwen](https://github.com/QwenLM/Qwen) - Code models
- [Microsoft Phi-3](https://github.com/microsoft/phi-3) - Compact models
- [clap](https://github.com/clap-rs/clap) - CLI framework
- [figment](https://github.com/figment-rs/figment) - Configuration

---

<div align="center">

**Made with ❤️ in Rust**

[Report Bug](https://github.com/skbhati199/soma-ai/issues) • [Request Feature](https://github.com/skbhati199/soma-ai/issues) • [Discussions](https://github.com/skbhati199/soma-ai/discussions)

</div>