# Soma AI - Single Binary Terminal Assistant

## Project Vision
A **single binary** (`soma`) that provides local AI-powered command suggestions for PowerShell/Bash/Zsh, with zero external dependencies at runtime. Built in Rust, embeds a small LLM (via ONNX/GGUF), and installs/configures itself completely.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        soma (single binary)                 │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   CLI       │  │   Shell     │  │   Embedded LLM      │  │
│  │   Parser    │──▶│   Detector  │──▶│   (GGUF/ONNX)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│         │                │                   │               │
│         ▼                ▼                   ▼               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Config    │  │   Prompt    │  │   Inference Engine  │  │
│  │   Manager   │  │   Builder   │  │   (llama.cpp/ORT)   │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│         │                │                   │               │
│         └────────────────┴───────────────────┘               │
│                              │                                │
│                              ▼                                │
│                    ┌─────────────────────┐                   │
│                    │   Output Handler    │                   │
│                    │   (Insert/Clipboard)│                   │
│                    └─────────────────────┘                   │
└─────────────────────────────────────────────────────────────┘
```

---

## Technology Choices

| Component | Choice | Rationale |
|-----------|--------|-----------|
| **Language** | Rust 2021 Edition | Single binary, memory safety, performance |
| **LLM Runtime** | `llama.cpp` via `llama-cpp-2` crate | GGUF support, CPU-optimized, no GPU needed |
| **Multi-Model Support** | `qwen2.5-coder-1.5b-instruct-q4_k_m.gguf` (~1GB) and more | Code-specialized, small, fast |
| **Config** | `figment` + TOML | Layered config (defaults/env/file) |
| **CLI** | `clap` v4 | Derive-based, completions, help |
| **Shell Detection** | Custom + `which` crate | Cross-platform |
| **Embedding** | `include_bytes!` + `tempfile` | Model embedded or auto-download |
| **Installer** | Self-contained `install` subcommand | `soma install --shell pwsh --keybind ctrl+space` |

---

## Project Structure

```
soma-ai/
├── Cargo.toml
├── Cargo.lock
├── LICENSE (MIT)
├── README.md
├── .gitignore
├── .github/
│   ├── workflows/
│   │   ├── ci.yml          # Build, test, release
│   │   └── release.yml     # Auto-release on tag
│   └── dependabot.yml
├── src/
│   ├── main.rs             # Entry point
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── args.rs         # Clap derive args
│   │   └── completions.rs  # Shell completions generator
│   ├── config/
│   │   ├── mod.rs
│   │   ├── schema.rs       # Config struct + defaults
│   │   └── loader.rs       # Figment layered config
│   ├── shell/
│   │   ├── mod.rs
│   │   ├── detector.rs     # Detect pwsh/bash/zsh/fish
│   │   ├── installer.rs    # Install keybindings, aliases
│   │   └── profiles.rs     # Profile paths per shell
│   ├── model/
│   │   ├── mod.rs
│   │   ├── manager.rs      # Download/verify/load model
│   │   ├── embedded.rs     # Include_bytes! fallback
│   │   └── downloader.rs   # HTTP with progress + resume
│   ├── inference/
│   │   ├── mod.rs
│   │   ├── engine.rs       # llama-cpp-2 wrapper
│   │   ├── prompt.rs       # Prompt templates per task
│   │   └── parser.rs       # Clean output (strip markdown)
│   ├── output/
│   │   ├── mod.rs
│   │   ├── inserter.rs     # PSReadLine / readline insert
│   │   └── clipboard.rs    # Cross-platform clipboard
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── ask.rs          # `soma ask "query"`
│   │   ├── insert.rs       # `soma insert "query"` (Ctrl+Space)
│   │   ├── install.rs      # `soma install`
│   │   ├── config.rs       # `soma config show/edit`
│   │   ├── model.rs        # `soma model pull/list/remove`
│   │   └── completions.rs  # `soma completions <shell>`
│   └── utils/
│       ├── mod.rs
│       ├── logging.rs      # tracing + env_filter
│       └── error.rs        # anyhow + thiserror
├── assets/
│   ├── prompts/
│   │   ├── system_powershell.txt
│   │   ├── system_bash.txt
│   │   └── system_generic.txt
│   └── default_config.toml
├── tests/
│   ├── integration_tests.rs
│   └── prompt_tests.rs
├── scripts/
│   ├── build-release.sh
│   └── download-model.sh
├── build.rs                # Embed model hash, version
└── xtask/                  # Custom build tasks (optional)
    └── Cargo.toml
```

---

## Core Features (MVP)

### 1. **Single Binary Distribution**
- `cargo build --release` → `target/release/soma` (~15MB + embedded model ~1GB)
- Or: Binary downloads model on first run (user choice)
- Statically linked where possible (`musl` for Linux)

### 2. **Shell Integration**
```bash
# Auto-detect shell and install
soma install                    # Interactive
soma install --shell pwsh       # PowerShell 7+
soma install --shell bash       # Bash
soma install --shell zsh        # Zsh
soma install --keybind "ctrl+space" --no-aliases
```

Installs:
- Key binding (`Ctrl+Space` → `soma insert`)
- Alias: `??` → `soma ask`
- Shell function: `soma()` wrapper
- Completions

### 3. **Commands**
```bash
soma ask "list all txt files"           # Print suggestion, copy to clipboard
soma insert "list all txt files"        # Insert on next line (for keybind)
soma install                            # Setup shell integration
soma config show                        # Show current config
soma config edit                        # Open $EDITOR
soma model pull qwen2.5-coder-1.5b      # Download model
soma model list                         # List local models
soma completions pwsh > _soma.ps1       # Generate completions
```

### 4. **Configuration** (`~/.config/soma/config.toml`)
```toml
[model]
name = "qwen2.5-coder-1.5b-instruct-q4_k_m.gguf"
auto_download = true
context_size = 4096
threads = 4
gpu_layers = 0

[shell]
default = "auto"  # pwsh, bash, zsh, fish
keybind = "ctrl+space"
alias = "??"

[prompt]
system = "powershell"  # powershell, bash, generic
max_tokens = 512
temperature = 0.1

[output]
insert_mode = "next_line"  # next_line, at_cursor, clipboard
show_confirmation = true
```

---

## Implementation Phases

### Phase 1: Foundation (Week 1)
- [ ] Cargo workspace setup
- [ ] CLI structure with `clap` (all subcommands)
- [ ] Config system with `figment` (defaults + file + env)
- [ ] Error handling with `thiserror`/`anyhow`
- [ ] Logging with `tracing`
- [ ] Basic `soma --version`, `soma --help`

### Phase 2: Model Management (Week 2)
- [ ] Model downloader (resume, progress, checksum verify)
- [ ] Embedded model support (`include_bytes!` for tiny models)
- [ ] `llama-cpp-2` integration (feature-gated)
- [ ] Model loading with params (context, threads, gpu_layers)
- [ ] `soma model pull/list/remove` commands

### Phase 3: Inference Engine (Week 3)
- [ ] Prompt templates (system + user)
- [ ] Inference loop with streaming
- [ ] Output parser (strip markdown, extract command)
- [ ] Token budget management
- [ ] Benchmark: <3s cold, <1s warm

### Phase 4: Shell Integration (Week 4)
- [ ] Shell detector (pwsh, bash, zsh, fish, cmd)
- [ ] Profile path resolver per shell
- [ ] Installer: keybindings, aliases, functions
- [ ] Completions generator
- [ ] `soma install` interactive + flags

### Phase 5: Output Handlers (Week 5)
- [ ] PSReadLine inserter (PowerShell)
- [ ] Readline/ZLE inserter (Bash/Zsh)
- [ ] Clipboard fallback (all platforms)
- [ ] `soma ask` (print + clipboard)
- [ ] `soma insert` (for keybindings)

### Phase 6: Polish & Release (Week 6)
- [ ] Cross-compilation (Windows/macOS/Linux)
- [ ] GitHub Actions CI/CD
- [ ] Auto-release on tag
- [ ] Homebrew/Scoop/Chocolatey formulas
- [ ] Documentation, README, examples
- [ ] Performance optimization

---

## Embedded Model Strategy

### Option A: Embed Tiny Model (~50MB)
```rust
// build.rs downloads and includes at compile time
const MODEL_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model.gguf"));
```
- Pros: Truly single file, works offline
- Cons: Binary size, compile time, memory

### Option B: Auto-Download on First Run (Recommended)
```rust
// Check ~/.cache/soma/models/, download if missing
// Show progress bar, verify sha256
```
- Pros: Small binary (~15MB), user chooses model
- Cons: Needs internet first run

### Option C: Both
- Default: auto-download
- `--embed-model` feature for air-gapped builds

---

## Key Technical Details

### Prompt Template (PowerShell)
```text
<|system|>
You are a PowerShell 7+ command expert. Output ONLY the command.
Rules: Use Get-ChildItem, Where-Object, Select-Object, Measure-Object.
Never use: ls, grep, find, awk, sed, tar.
Use -Filter for patterns. Pipeline | for chaining.
STOP after one command. No explanations.

<|user|>
Request: {{query}}
PowerShell Command:
<|assistant|>
```

### Output Cleaning
```rust
fn clean_output(raw: &str) -> String {
    raw
        .trim()
        .strip_prefix("```powershell").unwrap_or(raw)
        .strip_suffix("```").unwrap_or(raw)
        .trim()
        .lines()
        .next()
        .unwrap_or("")
        .to_string()
}
```

### PSReadLine Insertion
```powershell
# Installed function
function soma-insert {
    $q = Read-Host "soma"
    if ($q) { $cmd = soma insert $q; [Microsoft.PowerShell.PSConsoleReadLine]::Insert("`n$cmd") }
}
Set-PSReadLineKeyHandler -Key 'Ctrl+Space' -Function soma-insert
```

---

## CI/CD Pipeline

```yaml
# .github/workflows/release.yml
on:
  push:
    tags: ['v*']
jobs:
  release:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest, target: x86_64-unknown-linux-gnu
          - os: ubuntu-latest, target: x86_64-unknown-linux-musl
          - os: windows-latest, target: x86_64-pc-windows-msvc
          - os: macos-latest, target: x86_64-apple-darwin
          - os: macos-latest, target: aarch64-apple-darwin
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release --target ${{ matrix.target }}
      - uses: taiki-e/upload-release-asset@v1
        with: asset_path: target/${{ matrix.target }}/release/soma*
```

---

## Distribution Targets

| Platform | Target | Package Manager |
|----------|--------|-----------------|
| Linux (glibc) | `x86_64-unknown-linux-gnu` | Homebrew, AUR, binary |
| Linux (musl) | `x86_64-unknown-linux-musl` | Static, Alpine |
| Windows | `x86_64-pc-windows-msvc` | Scoop, Chocolatey, winget |
| macOS Intel | `x86_64-apple-darwin` | Homebrew |
| macOS Apple Silicon | `aarch64-apple-darwin` | Homebrew |

---

## Configuration Files to Create

1. **`Cargo.toml`** - Dependencies, features, metadata
2. **`src/main.rs`** - Entry, CLI dispatch
3. **`src/config/schema.rs`** - Config struct with `serde` + defaults
4. **`src/shell/detector.rs`** - `Shell::detect()` → `ShellType`
5. **`src/model/manager.rs`** - Download, verify, load
6. **`src/inference/engine.rs`** - `llama-cpp-2` wrapper
7. **`src/commands/install.rs`** - Full installer logic
8. **`assets/prompts/system_powershell.txt`** - System prompt
9. **`assets/default_config.toml`** - Embedded defaults
10. **`build.rs`** - Version, git hash, model hash embedding

---

## Dependencies (Cargo.toml)

```toml
[package]
name = "soma"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Local AI terminal command assistant - single binary"

[features]
default = ["inference", "clipboard", "completions"]
inference = ["llama-cpp-2"]
clipboard = ["arboard"]
completions = ["clap_complete"]
embedded-model = []  # For air-gapped builds

[dependencies]
clap = { version = "4.5", features = ["derive", "env", "wrap_help"] }
figment = { version = "0.11", features = ["toml", "env", "serde"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dirs = "5.0"
which = "5.0"
reqwest = { version = "0.12", features = ["blocking", "rustls-tls"] }
sha2 = "0.10"
indicatif = "0.17"  # Progress bars
arboard = { version = "3.3", optional = true }
clap_complete = { version = "4.5", optional = true }
llama-cpp-2 = { version = "0.8", optional = true, features = ["default", "metal", "cuda"] }

[profile.release]
lto = true
codegen-units = 1
strip = true
panic = "abort"
opt-level = "z"
```

---

## Next Steps

1. **Initialize repo**: `cargo init soma-ai && cd soma-ai`
2. **Add dependencies** to `Cargo.toml`
3. **Create module structure** per `src/` tree
4. **Implement Phase 1** (CLI + Config + Logging)
5. **Test locally** with `cargo run -- ask "test"`
6. **Add CI/CD** and iterate

---

## Success Criteria

- [ ] `cargo build --release` → single `soma` binary (<20MB without model)
- [ ] `soma install` works on PowerShell 7+, Bash, Zsh
- [ ] `Ctrl+Space` inserts command on next line
- [ ] Model auto-downloads on first use
- [ ] Inference <3s cold, <1s warm (CPU)
- [ ] Publishes to GitHub Releases automatically
- [ ] Installable via `scoop install soma`, `brew install soma`