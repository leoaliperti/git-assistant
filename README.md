# 🤖 Git Assistant

**Git Assistant** is a robust, blazingly fast CLI tool written in Rust that automates your Git workflow using **Apple Intelligence** running completely locally. 

Instead of manually writing commit messages, `git-assistant` pipes your staged changes (`git diff`) directly into the `apfel` CLI, generates a clean, conventional commit message, asks for your confirmation, and pushes the code. All in seconds, with zero cloud API costs and maximum privacy.

## ✨ Features

- **🧠 Local AI Powered:** Uses Apple's on-device FoundationModels via the `apfel` wrapper. Your code never leaves your Mac.
- **🛡️ Bulletproof Error Handling:** Intelligently handles AI safety filters, unsupported languages, and unexpected formatting. If the AI fails, it gracefully falls back to manual input.
- **🕹️ Interactive Review:** Review the generated message before committing. You can Accept (`Y`), Regenerate (`r`), or write a custom message (`n`).
- **⚡ Auto-pilot Mode:** Pass the `-y` flag to skip the prompt and instantly commit and push.
- **🪈 Pipe-safe Architecture:** Feeds massive code diffs directly into the AI via standard streams (`stdin`), preventing memory spikes and CLI argument limits.
- **🚀 Seamless Push Forwarding:** Any extra arguments are passed straight to `git push` (e.g., `git-assistant -y origin main --force`).

## 📋 Prerequisites

Before installing, ensure you have the following on your system:
1. **macOS 26+** (Required for the local Apple FoundationModels).
2. **[apfel](https://github.com/Arthur-Ficial/apfel)** installed and available in your system's `PATH`.
3. **Rust & Cargo** installed (via [rustup](https://rustup.rs/)).
4. **Git** configured for your repository.

## 🛠️ Installation

Clone this repository and install it globally using Cargo:

```bash
git clone https://github.com/leoaliperti/git-assistant.git
cd git-assistant
cargo install --path .
