class Soma < Formula
  desc "Local AI terminal command assistant"
  homepage "https://github.com/skbhati199/soma-ai"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/skbhati199/soma-ai/releases/download/v#{version}/soma-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_INTEL_SHA256"
    else
      url "https://github.com/skbhati199/soma-ai/releases/download/v#{version}/soma-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ARM_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/skbhati199/soma-ai/releases/download/v#{version}/soma-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_LINUX_X64_SHA256"
    elsif Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
      url "https://github.com/skbhati199/soma-ai/releases/download/v#{version}/soma-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_LINUX_ARM64_SHA256"
    end
  end

  def install
    bin.install "soma"
    
    # Generate shell completions
    generate_completions_from_executable(bin/"soma", "completions")
  end

  def caveats
    <<~EOS
      To enable shell integration, run:
        soma install
      
      Then restart your shell.
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/soma --version")
    assert_match "ask", shell_output("#{bin}/soma --help")
  end
end
