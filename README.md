# snacc

**snacc** is a lightweight CLI tool that automatically copies cells from recently downloaded Kaggle notebooks — formatted for LLM-friendly pasting into tools like ChatGPT, Copilot, or your own prompts.

It’s built to eliminate the repetitive copy-paste work from `.ipynb` files and streamline your workflow.

---

## Features

- Automatically watches your Downloads folder for new Kaggle notebooks
- One-shot mode to copy the most recent notebook manually
- Deletes notebooks after copying (enabled by default — can be disabled)
- Choose which cells to copy:
  - `--code` (default)
  - `--markdown`
  - `--all`
- Automatically formats content in a clean, copy-ready way
- Cross-platform support (Linux, macOS, Windows — note: `watch` does not work on WSL for now)

---

## Installation

### Option 1: Prebuilt release (recommended)

Download the latest version from the [Releases page](https://github.com/yourusername/snacc/releases):

- **Installers available**:
  - Windows (`.msi`)
  - Linux (`.deb`)
- **Manual binaries**:
  - `.zip` files for Linux, macOS, and Windows

Unzip or install as appropriate, then run `snacc --help` to get started.

### Option 2: Build from source

```bash
git clone https://github.com/yourusername/snacc.git
cd snacc
cargo build --release
./target/release/snacc --help
