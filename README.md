# snacc

**snacc** is a lightweight CLI tool that automatically copies cells from recently downloaded Kaggle notebooks — formatted for LLM-friendly pasting into tools like ChatGPT, Copilot, or your own prompts.
It’s built to eliminate the repetitive copy-paste work from .ipynb files and streamline your workflow.

---

## Features

- Watches your Downloads folder and automatically copies the latest Kaggle notebook  
  *(watch mode is the default; does not currently work on WSL)*
- One-shot command to copy the most recently downloaded notebook
- Deletes the notebook after copying (enabled by default; can be disabled)
- Choose which cell types to copy:
  - `--code` (default)
  - `--markdown`
  - `--all`
- Outputs cells in a clean, LLM-friendly format for copy-paste
- Works on Windows, macOS, and Linux

---

## Installation

### Option 1: Prebuilt release (recommended)

Download from the [Releases page](https://github.com/yourusername/snacc/releases):

- Installers:
  - Windows (`.msi`)
  - Linux (`.deb`)
- Binaries:
  - `.zip` archives for Windows, Linux, and macOS

Then run:

```bash
snacc --help
```

### Option 2: Build from source

```bash
git clone https://github.com/yourusername/snacc.git
cd snacc
cargo build --release
./target/release/snacc --help
```

Requires [Rust](https://www.rust-lang.org/tools/install) and Cargo.

---

## Coming soon

- GUI version (`snacc x`) with:
  - Keyboard shortcuts
  - Clipboard preview
- Option to send directly into a local LLM or copilot interface

---

## Why?

If you often have to copy-paste code cells from Kaggle (or any) notebooks into ChatGPT, `snacc` automates that process. Just activate the watcher, download the notebook, and the cells are copied to your clipboard automatically.

---

## Feedback

Open an issue on GitHub or comment on the [Fast.ai forum post](https://forums.fast.ai/t/) to share thoughts or suggestions.

---

## License

MIT
