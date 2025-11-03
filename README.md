# rff – Rust Fuzzy Finder

> A fast, beautiful, `fzf`-like fuzzy finder written in **pure Rust**.

![rff in action](https://i.imgur.com/placeholder.gif)  
*Live fuzzy search with `ratatui`, `Ctrl+j/k` navigation, and `$EDITOR` integration.*

---

## Feature Roadmap

- [x] Live fuzzy search with `fuzzy-matcher`
- [x] Smooth TUI with `ratatui` (zero flicker)
- [x] Smart file walking (`.gitignore`, `--all`, hidden files)
- [x] Parallel traversal using `ignore::WalkParallel`
- [x] `Ctrl+j` / `Ctrl+k` navigation
- [x] Free typing of `j`, `k`, `mod`, `joker` in query
- [x] `Enter` opens file in `$EDITOR`
- [x] Clean exit with `Esc`
- [x] Filter `target/` by default
- [x] Production-grade error handling (`thiserror`)
- [x] Unit tests with `tempfile`
- [x] Semantic commit history
- [ ] `--multi` mode (select multiple files)
- [ ] Preview pane (show file content)
- [ ] Syntax highlighting in preview (`bat`-style)
- [ ] Publish to [crates.io](https://crates.io)
- [ ] Config file (`~/.config/rff/config.toml`)
- [ ] Custom keybindings
- [ ] Pipe mode (`git ls-files | rff`)

---

## Installation

```bash
cargo install --git https://github.com/crabbylab/rff.git
```
## Usage
```bash
# Basic: search current directory
rff

# Include hidden files and .gitignore
rff --all

# Start in a specific directory
rff --all /path/to/project

# Use a specific editor
EDITOR=hx rff

# Combine
rff --all . hx
```


