<p align="center">
  <img src="images/logo.png" alt="stylemd logo" width="300" />
</p>

<h1 align="center">stylemd</h1>
<p align="center"><strong>Markdown styling toolkit — lint, format, fix, and polish your prose.</strong></p>

[![Release](https://img.shields.io/github/v/release/leaf-kit/style.md?label=release)](https://github.com/leaf-kit/style.md/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub Stars](https://img.shields.io/github/stars/leaf-kit/style.md?style=social)](https://github.com/leaf-kit/style.md/stargazers)
[![GitHub Downloads](https://img.shields.io/github/downloads/leaf-kit/style.md/total?label=downloads)](https://github.com/leaf-kit/style.md/releases)
[![Homebrew](https://img.shields.io/badge/homebrew-leaf--kit%2Fstylemd-yellow.svg)](https://github.com/leaf-kit/homebrew-stylemd)

> **v0.2.0 "First Stroke"** — [GitHub Release](https://github.com/leaf-kit/style.md/releases/tag/v0.2.0) | [Homebrew Tap](https://github.com/leaf-kit/homebrew-stylemd)

**stylemd** keeps your Markdown files clean, consistent, and well-structured. It catches style violations, auto-fixes what it can, and reports what needs your attention — so every `.md` file reads as good as it looks.

Built with Rust for speed and safety. Optimized with LTO. Ships as a single static binary.

[Korean Documentation (한국어)](README.md)

---

## Brand Story

> *Every great document starts with a single stroke.*

**stylemd** treats every Markdown file as a canvas. Style isn't just formatting — it's clarity, consistency, and craft. Good style makes your writing easier to read, easier to maintain, and easier to trust.

The name says it all: **style** your **.md**.

### v0.2.0 — "First Stroke"

The first stroke of a brush defines the character. Before color, before composition — there is the stroke that sets the direction.

**"First Stroke"** is the foundational release of stylemd. It lays down the essential styling rules that every Markdown project needs:

- **6 core lint rules** covering headings, whitespace, blank lines, code blocks, lists, and images
- **Light / Deep dual modes** balancing speed and thoroughness
- **Auto-fix** for common violations — one command to clean up your files
- **Multi-format output** (text, JSON, YAML, HTML, Markdown) for any workflow
- **i18n** with English and Korean support from day one

This is the foundation. Future releases will build on this stroke — adding new rules, new integrations, and new ways to keep your Markdown sharp.

---

## Terminology

| Term | Definition |
|------|-----------|
| **Light mode** | Default mode — fast syntax and structure checks |
| **Deep mode** | `--deep` flag — all light checks + link validation, TOC, code block language |
| **Lint** | Static analysis to detect style issues |
| **Fix** | Automatically correct fixable lint violations |
| **Fixable** | An issue that `stylemd fix` can auto-correct |

---

## Light vs Deep Mode

stylemd has two check modes to balance speed and thoroughness:

| | Light (default) | Deep (`--deep`) |
|---|---|---|
| **Speed** | Fast | Slower (HTTP requests) |
| **Heading structure** | H1 duplicate, level skips | Same |
| **Trailing whitespace** | Spaces and tabs | Same |
| **Blank lines** | Around headings only | + consecutive blank line limit |
| **Code blocks** | Unclosed blocks only | + language specifier required |
| **List markers** | Consistency check | Same |
| **Image references** | File existence, alt text | Same |
| **Anchor links** | - | Internal `#section` validation |
| **Relative links** | - | File existence check |
| **External links** | - | HTTP status check |
| **TOC** | - | Heading vs TOC structure |

```bash
stylemd check .              # Light mode (fast)
stylemd check --deep .       # Deep mode (thorough)
```

---

## Installation

### Homebrew (macOS / Linux)

Downloads the pre-built binary for your platform. No Rust required:

```bash
brew tap leaf-kit/stylemd
brew install stylemd
```

| Platform | Auto-selected binary |
|----------|---------------------|
| macOS (Apple Silicon) | `stylemd-aarch64-apple-darwin` |
| macOS (Intel) | `stylemd-x86_64-apple-darwin` |
| Linux (x86_64) | `stylemd-x86_64-unknown-linux-gnu` |
| Linux (aarch64) | `stylemd-aarch64-unknown-linux-gnu` |

### Build from Source

```bash
git clone https://github.com/leaf-kit/style.md.git
cd style.md
cargo build --release
cp target/release/stylemd /usr/local/bin/
```

Or use the interactive build script (runs tests before release):

```bash
./build.sh
```

### Update

| Method | Command |
|--------|---------|
| Homebrew | `brew upgrade stylemd` |
| Source | `git pull && cargo build --release && cp target/release/stylemd /usr/local/bin/` |

### Uninstall

| Method | Command |
|--------|---------|
| Homebrew | `brew uninstall stylemd && brew untap leaf-kit/stylemd` |
| Manual | `rm /usr/local/bin/stylemd` |

---

## Commands

```
% stylemd
stylemd — Markdown styling toolkit

Usage: stylemd [OPTIONS] [COMMAND]

Commands:
  check  Check Markdown files for style issues (light mode by default)
  fmt    Auto-format Markdown files
  fix    Auto-fix lint errors in Markdown files
  stats  Show quality statistics and scores
  links  Check all links in Markdown files (always deep)
  lsp    Start Language Server Protocol server
  init   Create a default .stylemd.toml configuration file
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose      Set log level to verbose
  -q, --quiet        Disable progress output
      --lang <LANG>  Output language (en, ko) [default: en]
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

---

## Output Examples

All examples are **actual outputs** from `playground/` test files.

### 1. Light Check (default) — `stylemd check`

```
% stylemd check playground/

playground/all_issues.md
  playground/all_issues.md:1:1: warning Missing blank line after heading [fixable]
  playground/all_issues.md:1:18: warning Trailing whitespace found (3 chars) [fixable]
  playground/all_issues.md:3:1: warning Multiple H1 headings found; only one H1 is recommended
  playground/all_issues.md:5:1: error Heading level skipped: H3 follows H1 (expected H2 or lower)
  playground/all_issues.md:6:1: warning List marker '*' should be '-' for consistency [fixable]
  playground/all_issues.md:16:25: warning Trailing whitespace (including tabs) found (1 chars) [fixable]
  ...

playground/image_refs.md
  playground/image_refs.md:9:1: warning Image file not found: images/nonexistent.png
  playground/image_refs.md:13:1: warning Image missing alt text: ../images/logo.svg
  playground/image_refs.md:17:1: warning Image file not found: images/missing.jpg
  ...

Summary: 9 files checked, 7 with issues
  3 errors, 28 warnings
  23 auto-fixable (use `stylemd fix` to fix)
```

> Light mode: no link checks, no code block language check, no consecutive blank line limit.

### 2. Deep Check — `stylemd check --deep`

```
% stylemd check --deep playground/

playground/all_issues.md
  ...
  playground/all_issues.md:9:1: warning Code block missing language specifier
  playground/all_issues.md:13:1: warning Too many consecutive blank lines (found 2, max 1) [fixable]
  playground/all_issues.md:17:1: error Broken anchor link: #nowhere (heading not found)

playground/broken_links.md
  playground/broken_links.md:7:3: error Broken anchor link: #nonexistent-section (heading not found)
  playground/broken_links.md:8:3: error Broken anchor link: #does-not-exist (heading not found)
  playground/broken_links.md:13:3: error Broken relative link: ./does_not_exist.md (file not found)
  ...

Summary: 9 files checked, 8 with issues
  8 errors, 34 warnings
  27 auto-fixable (use `stylemd fix` to fix)
  info playground/blank_lines.md: Document has 4 headings but no Table of Contents
  info playground/heading_structure.md: Document has 5 headings but no Table of Contents
  ...
```

> Deep mode adds: code block language, consecutive blank lines, broken links, TOC check.

### 3. Fix Preview — `stylemd fix --diff --dry-run`

```
% stylemd fix --diff --dry-run playground/trailing_whitespace.md

--- playground/trailing_whitespace.md
+++ playground/trailing_whitespace.md
-# Trailing Whitespace Test   
+# Trailing Whitespace Test
+This line has trailing spaces.
 ...
  dry-run playground/trailing_whitespace.md (6 issues would be fixed)

Done: 6 issue(s) would be auto-fixed
```

### 4. Quality Stats — `stylemd stats`

```
% stylemd stats playground/

Quality Report
  Files scanned:      9
  Clean files:        2
  Total issues:       31
  Errors:             3
  Warnings:           28
  Auto-fixable:       23
  Quality score:      22%

  Issues by rule:
    blank-line                     11
    trailing-whitespace            7
    list-style                     5
    heading-structure              4
    image-ref                      3
    code-block                     1
```

### 5. Link Check — `stylemd links`

```
% stylemd links playground/

playground/broken_links.md
  playground/broken_links.md:7:3: error Broken anchor link: #nonexistent-section (heading not found)
  playground/broken_links.md:8:3: error Broken anchor link: #does-not-exist (heading not found)
  playground/broken_links.md:13:3: error Broken relative link: ./does_not_exist.md (file not found)

Summary: 9 files checked, 3 with issues
  5 errors, 0 warnings
```

### 6. JSON / YAML Output

```bash
stylemd check --format json playground/   # JSON output
stylemd check --format yaml playground/   # YAML output
stylemd stats --report md playground/     # Markdown report
stylemd stats --report html playground/   # HTML report
```

### 7. Pipe Support

```bash
echo "# Bad doc   " | stylemd check -           # Lint from stdin
stylemd check playground/ 2>&1 | grep "error"    # Filter errors
stylemd check --format json playground/ | jq '.'  # JSON pipe
```

### 8. Korean Output — `--lang ko`

```
% stylemd --lang ko check playground/trailing_whitespace.md

playground/trailing_whitespace.md
  playground/trailing_whitespace.md:1:27: 경고 후행 공백 발견 (3자) [수정가능]
  playground/trailing_whitespace.md:7:1: 경고 헤딩 뒤에 빈 줄 누락 [수정가능]
  ...

요약: 1 파일 검사 완료, 1 이슈 있음
  0 오류, 6 경고
  6 자동 수정 가능 (`stylemd fix`로 수정하세요)
```

---

## Options Reference

### Global Options

| Option | Description |
|--------|-------------|
| `--lang <LANG>` | Output language: `en` (default), `ko` (Korean) |
| `-v, --verbose` | Enable verbose output |
| `-q, --quiet` | Suppress progress output |

### `stylemd check`

| Option | Description |
|--------|-------------|
| `--deep` | Enable deep mode (link validation, TOC, code block language) |
| `-c, --config <FILE>` | Custom configuration file path |
| `-f, --format <FMT>` | Output format: `pretty` (default), `json`, `yaml` |
| `--watch` | Watch for file changes and re-check |
| `--summary` | Show summary statistics |
| `--ignore-pattern <GLOB>` | Ignore files matching pattern |
| `--git-staged` | Only check git staged files |

### `stylemd fix`

| Option | Description |
|--------|-------------|
| `--deep` | Fix with all rules enabled |
| `--diff` | Show colored diff of changes |
| `--dry-run` | Preview changes without writing |
| `-c, --config <FILE>` | Custom configuration file path |

### `stylemd stats`

| Option | Description |
|--------|-------------|
| `--deep` | Include link and TOC analysis |
| `--report <FMT>` | Report format: `pretty`, `json`, `yaml`, `md`, `html` |

---

## Configuration

Create a configuration file:

```bash
stylemd init    # Creates .stylemd.toml
```

Config search order: `./.stylemd.toml` → `~/.config/stylemd/config.toml`

### Default Configuration

```toml
# stylemd configuration
# https://github.com/leaf-kit/style.md
#
# stylemd has two check modes:
#
#   stylemd check [path]         Light mode (default) — fast, essential checks
#   stylemd check --deep [path]  Deep mode — thorough analysis including links

# ── Light mode rules (default) ──────────────────────────────────────────
[light]
heading_structure = true           # Detect duplicate H1, heading level skips
trailing_whitespace = true         # Detect trailing spaces and tabs
blank_lines_around_headings = true # Require blank lines before/after headings
unclosed_code_block = true         # Detect unclosed ``` blocks
list_marker_consistency = true     # Enforce consistent list markers (- * +)
image_file_exists = true           # Check that referenced image files exist
list_marker = "-"                  # Preferred list marker (-, *, +)

# ── Deep mode rules (--deep) ────────────────────────────────────────────
[deep]
code_block_language = true         # Require language specifier on code blocks
consecutive_blank_lines = true     # Limit consecutive blank lines
max_consecutive_blank_lines = 1    # Maximum allowed consecutive blank lines
anchor_links = true                # Validate internal anchor links (#section)
relative_links = true              # Check relative file links exist
external_links = true              # HTTP check external URLs (slow)
toc_check = true                   # Verify TOC matches heading structure
link_timeout_seconds = 10          # Timeout for HTTP link checks

# ── Formatting rules (stylemd fmt) ──────────────────────────────────────
[format]
ensure_final_newline = true
trim_trailing_whitespace = true
max_consecutive_blank_lines = 1
list_marker = "-"

# ── Ignore patterns ────────────────────────────────────────────────────
# ignore = ["node_modules/**", "target/**", ".git/**"]
```

### Configuration Examples

Disable image checks in light mode:

```toml
[light]
image_file_exists = false
```

Skip external HTTP link checks in deep mode:

```toml
[deep]
external_links = false
```

Use `*` as list marker:

```toml
[light]
list_marker = "*"
```

---

## Lint Rules

### Light Mode (default)

| Rule | Severity | Fixable | Description |
|------|----------|---------|-------------|
| `heading-structure` | error/warning | No | Duplicate H1, heading level skips |
| `trailing-whitespace` | warning | Yes | Trailing spaces and tabs |
| `blank-line` | warning | Yes | Missing blank lines around headings |
| `code-block` | error | Yes | Unclosed code blocks |
| `list-style` | warning | Partial | Mixed list markers, bad indentation |
| `image-ref` | warning | No | Missing image files, missing alt text |

### Deep Mode (`--deep`) — adds:

| Rule | Severity | Fixable | Description |
|------|----------|---------|-------------|
| `code-block` | warning | No | Code block missing language specifier |
| `blank-line` | warning | Yes | Consecutive blank line limit |
| `broken-link` | error | No | Broken anchors, relative paths, external URLs |
| TOC check | info | No | Heading count vs TOC presence |

---

## Git Hook Integration

```bash
# .git/hooks/pre-commit
#!/bin/sh
stylemd check --git-staged
```

For thorough pre-commit checks:

```bash
stylemd check --deep --git-staged
```

---

## Playground

```
playground/
├── all_issues.md           # Every issue type combined
├── blank_lines.md          # Blank line rule violations
├── broken_links.md         # Broken anchor/relative links
├── clean.md                # Clean file (no issues)
├── code_blocks.md          # Missing language, unclosed blocks
├── heading_structure.md    # Duplicate H1, level skips
├── image_refs.md           # Missing images, missing alt text
├── list_style.md           # Mixed markers, bad indentation
└── trailing_whitespace.md  # Trailing spaces and tabs
```

---

## Related Projects

| Project | Description |
|---------|-------------|
| [markdownlint](https://github.com/DavidAnson/markdownlint) | Node.js Markdown linter |
| [mdformat](https://github.com/executablebooks/mdformat) | Python Markdown formatter |
| [lsmd](https://github.com/leaf-kit/ls.md) | Markdown-aware directory listing |

---

## Feedback & Contributing

Contributions, issues, and feature requests are welcome.

[github.com/leaf-kit/style.md/issues](https://github.com/leaf-kit/style.md/issues)

## License

[MIT](LICENSE)
