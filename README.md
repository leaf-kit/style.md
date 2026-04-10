<p align="center">
  <img src="images/logo.png" alt="stylemd logo" width="300" />
</p>

<h1 align="center">stylemd</h1>
<p align="center"><strong>마크다운 스타일링 툴킷 — 린트, 포맷, 수정, 그리고 다듬기</strong></p>

[![Release](https://img.shields.io/github/v/release/leaf-kit/style.md?label=release)](https://github.com/leaf-kit/style.md/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub Stars](https://img.shields.io/github/stars/leaf-kit/style.md?style=social)](https://github.com/leaf-kit/style.md/stargazers)
[![GitHub Downloads](https://img.shields.io/github/downloads/leaf-kit/style.md/total?label=downloads)](https://github.com/leaf-kit/style.md/releases)
[![Homebrew](https://img.shields.io/badge/homebrew-leaf--kit%2Fstylemd-yellow.svg)](https://github.com/leaf-kit/homebrew-stylemd)

> **v0.2.0 "First Stroke"** — [GitHub Release](https://github.com/leaf-kit/style.md/releases/tag/v0.2.0) | [Homebrew Tap](https://github.com/leaf-kit/homebrew-stylemd)

**stylemd**는 마크다운 파일의 스타일을 관리하는 CLI 도구입니다. 스타일 위반을 감지하고, 자동으로 수정할 수 있는 것은 고치고, 나머지는 리포트로 알려줍니다 — 모든 `.md` 파일이 읽기 좋고 보기 좋도록.

Rust로 빌드. LTO 최적화. 단일 바이너리 배포.

[English Documentation](README_en.md)

---

## 브랜드 스토리

> *모든 훌륭한 글은 한 획에서 시작됩니다.*

**stylemd**는 모든 마크다운 파일을 하나의 캔버스로 바라봅니다. 스타일은 단순한 포매팅이 아닙니다 — 명확함, 일관성, 그리고 정성입니다. 좋은 스타일은 글을 읽기 쉽게, 유지보수하기 쉽게, 그리고 신뢰할 수 있게 만듭니다.

이름이 모든 것을 말합니다: 당신의 **.md**에 **style**을.

### v0.2.0 — "First Stroke" (첫 획)

붓의 첫 획이 글자의 성격을 결정합니다. 색보다, 구도보다 먼저 — 방향을 정하는 한 획이 있습니다.

**"First Stroke"**는 stylemd의 기초를 다지는 첫 릴리스입니다. 모든 마크다운 프로젝트에 필요한 핵심 스타일링 규칙을 제공합니다:

- **6가지 핵심 린트 규칙** — 헤딩, 공백, 빈 줄, 코드 블록, 목록, 이미지
- **Light / Deep 이중 모드** — 속도와 정밀도의 균형
- **자동 수정** — 하나의 명령으로 일반적인 위반 사항 정리
- **다중 출력 형식** (텍스트, JSON, YAML, HTML, Markdown)
- **다국어 지원** — 첫날부터 영어와 한국어 지원

이것은 시작입니다. 이 첫 획 위에 새로운 규칙, 새로운 연동, 마크다운을 더 날카롭게 다듬는 기능들이 쌓여갈 것입니다.

---

## 용어 정리

| 용어 | 설명 |
|------|------|
| **Light 모드** | 기본 모드 — 빠른 문법 및 구조 검사 |
| **Deep 모드** | `--deep` 플래그 — 링크 검증, TOC, 코드 블록 언어 등 전체 검사 |
| **린트(Lint)** | 스타일 이슈 정적 분석 |
| **수정(Fix)** | 수정 가능한 린트 위반 자동 교정 |
| **수정가능(Fixable)** | `stylemd fix`로 자동 교정 가능한 이슈 |

---

## Light vs Deep 모드

속도와 정밀도의 균형을 위한 두 가지 검사 모드:

| | Light (기본) | Deep (`--deep`) |
|---|---|---|
| **속도** | 빠름 | 느림 (HTTP 요청) |
| **헤딩 구조** | H1 중복, 레벨 스킵 | 동일 |
| **후행 공백** | 공백, 탭 | 동일 |
| **빈 줄** | 헤딩 전후만 | + 연속 빈 줄 제한 |
| **코드 블록** | 미닫힘만 | + 언어 지정 필수 |
| **목록 기호** | 일관성 검사 | 동일 |
| **이미지 참조** | 파일 존재, alt 텍스트 | 동일 |
| **앵커 링크** | - | 내부 `#section` 검증 |
| **상대 링크** | - | 파일 존재 검사 |
| **외부 링크** | - | HTTP 상태 검사 |
| **목차(TOC)** | - | 헤딩 vs TOC 구조 |

```bash
stylemd check .              # Light 모드 (빠름)
stylemd check --deep .       # Deep 모드 (정밀)
```

---

## 설치

### Homebrew (macOS / Linux)

플랫폼에 맞는 바이너리를 자동으로 다운로드합니다. Rust 설치 불필요:

```bash
brew tap leaf-kit/stylemd
brew install stylemd
```

| 플랫폼 | 자동 선택 |
|--------|----------|
| macOS (Apple Silicon) | `stylemd-aarch64-apple-darwin` |
| macOS (Intel) | `stylemd-x86_64-apple-darwin` |
| Linux (x86_64) | `stylemd-x86_64-unknown-linux-gnu` |
| Linux (aarch64) | `stylemd-aarch64-unknown-linux-gnu` |

### 소스 빌드

```bash
git clone https://github.com/leaf-kit/style.md.git
cd style.md
cargo build --release
cp target/release/stylemd /usr/local/bin/
```

### 업데이트 / 삭제

| 방법 | 업데이트 | 삭제 |
|------|---------|------|
| Homebrew | `brew upgrade stylemd` | `brew uninstall stylemd && brew untap leaf-kit/stylemd` |
| 수동 | `git pull && cargo build --release` | `rm /usr/local/bin/stylemd` |

---

## 핵심 명령어

```bash
stylemd check  [path]          # 문제 탐지 (light)
stylemd check --deep [path]    # 문제 탐지 (deep)
stylemd fmt    [path]          # 자동 포매팅
stylemd fix    [path]          # 린트 오류 자동 수정
stylemd stats  [path]          # 품질 점수 리포트
stylemd links  [path]          # 링크 전수 검사 (항상 deep)
stylemd init                   # .stylemd.toml 생성
```

---

## 실행 결과 예시

모든 예시는 `playground/` 테스트 파일의 **실제 출력**입니다.

### 1. Light 검사 (기본)

```
% stylemd --lang ko check playground/trailing_whitespace.md

playground/trailing_whitespace.md
  playground/trailing_whitespace.md:1:27: 경고 후행 공백 발견 (3자) [수정가능]
  playground/trailing_whitespace.md:3:31: 경고 후행 공백 발견 (3자) [수정가능]
  playground/trailing_whitespace.md:4:29: 경고 후행 공백(탭 포함) 발견 (1자) [수정가능]
  playground/trailing_whitespace.md:7:1: 경고 헤딩 뒤에 빈 줄 누락 [수정가능]
  playground/trailing_whitespace.md:7:19: 경고 후행 공백 발견 (3자) [수정가능]
  playground/trailing_whitespace.md:8:41: 경고 후행 공백(탭 포함) 발견 (3자) [수정가능]

요약: 1 파일 검사 완료, 1 이슈 있음
  0 오류, 6 경고
  6 자동 수정 가능 (`stylemd fix`로 수정하세요)
```

### 2. 이미지 참조 검사 (light 모드)

```
% stylemd check playground/image_refs.md

playground/image_refs.md
  playground/image_refs.md:9:1: warning Image file not found: images/nonexistent.png
  playground/image_refs.md:13:1: warning Image missing alt text: ../images/logo.svg
  playground/image_refs.md:17:1: warning Image file not found: images/missing.jpg

Summary: 1 files checked, 1 with issues
  0 errors, 3 warnings
```

### 3. Deep 검사

```
% stylemd check --deep playground/broken_links.md

playground/broken_links.md
  playground/broken_links.md:7:3: error Broken anchor link: #nonexistent-section (heading not found)
  playground/broken_links.md:8:3: error Broken anchor link: #does-not-exist (heading not found)
  playground/broken_links.md:13:3: error Broken relative link: ./does_not_exist.md (file not found)

Summary: 1 files checked, 1 with issues
  3 errors, 0 warnings
```

### 4. 품질 통계

```
% stylemd --lang ko stats playground/

품질 리포트
  검사한 파일:             9
  깨끗한 파일:             2
  전체 이슈:              31
  오류:                 3
  경고:                 28
  자동 수정 가능:           23
  품질 점수:              22%

  규칙별 이슈:
    blank-line                     11
    trailing-whitespace            7
    list-style                     5
    heading-structure              4
    image-ref                      3
    code-block                     1
```

### 5. 파이프 / 출력 형식

```bash
echo "# Bad doc   " | stylemd check -           # stdin 린트
stylemd check --format json playground/          # JSON 출력
stylemd check --format yaml playground/          # YAML 출력
stylemd stats --report md playground/            # Markdown 리포트
```

---

## 옵션 참조

### 전역 옵션

| 옵션 | 설명 |
|------|------|
| `--lang <LANG>` | 출력 언어: `en` (기본), `ko` (한국어) |
| `-v, --verbose` | 상세 출력 |
| `-q, --quiet` | 진행 출력 억제 |

### `stylemd check`

| 옵션 | 설명 |
|------|------|
| `--deep` | Deep 모드 활성화 |
| `-c, --config <FILE>` | 설정 파일 경로 |
| `-f, --format <FMT>` | 출력 형식: `pretty`, `json`, `yaml` |
| `--watch` | 파일 변경 감지 후 자동 재검사 |
| `--ignore-pattern <GLOB>` | 패턴에 맞는 파일/폴더 무시 |
| `--git-staged` | Git staged 파일만 검사 |

### `stylemd fix`

| 옵션 | 설명 |
|------|------|
| `--deep` | 모든 규칙으로 수정 |
| `--diff` | 수정 전후 diff 출력 |
| `--dry-run` | 실제 수정 없이 미리보기 |

### `stylemd stats`

| 옵션 | 설명 |
|------|------|
| `--deep` | 링크, TOC 분석 포함 |
| `--report <FMT>` | `pretty`, `json`, `yaml`, `md`, `html` |

---

## 설정

```bash
stylemd init    # .stylemd.toml 생성
```

설정 파일 탐색 순서: `./.stylemd.toml` → `~/.config/stylemd/config.toml`

### 기본 설정 파일

```toml
# ── Light 모드 규칙 (기본) ─────────────────────────────────
[light]
heading_structure = true           # H1 중복, 레벨 스킵 감지
trailing_whitespace = true         # 후행 공백, 탭 감지
blank_lines_around_headings = true # 헤딩 전후 빈 줄 필수
unclosed_code_block = true         # 미닫힌 코드 블록 감지
list_marker_consistency = true     # 목록 기호 일관성
image_file_exists = true           # 참조된 이미지 파일 존재 여부
list_marker = "-"                  # 선호 목록 기호

# ── Deep 모드 규칙 (--deep) ────────────────────────────────
[deep]
code_block_language = true         # 코드 블록 언어 지정 필수
consecutive_blank_lines = true     # 연속 빈 줄 제한
max_consecutive_blank_lines = 1    # 최대 연속 빈 줄 수
anchor_links = true                # 내부 앵커 링크 검증
relative_links = true              # 상대 파일 링크 존재 검사
external_links = true              # 외부 URL HTTP 검사 (느림)
toc_check = true                   # 목차 구조 검증
link_timeout_seconds = 10          # HTTP 링크 검사 타임아웃

# ── 포매팅 규칙 (stylemd fmt) ──────────────────────────────
[format]
ensure_final_newline = true
trim_trailing_whitespace = true
max_consecutive_blank_lines = 1
list_marker = "-"

# ignore = ["node_modules/**", "target/**"]
```

### 설정 예시

```toml
# 이미지 검사 비활성화
[light]
image_file_exists = false

# 외부 링크 HTTP 검사 비활성화 (deep 모드)
[deep]
external_links = false
```

---

## 린트 규칙

### Light 모드 (기본)

| 규칙 | 심각도 | 자동 수정 | 설명 |
|------|--------|-----------|------|
| `heading-structure` | error/warning | 불가 | H1 중복, 헤딩 레벨 스킵 |
| `trailing-whitespace` | warning | 가능 | 후행 공백 및 탭 |
| `blank-line` | warning | 가능 | 헤딩 전후 빈 줄 누락 |
| `code-block` | error | 가능 | 미닫힌 코드 블록 |
| `list-style` | warning | 부분 | 목록 기호 혼용, 들여쓰기 |
| `image-ref` | warning | 불가 | 이미지 파일 누락, alt 텍스트 누락 |

### Deep 모드 추가 규칙

| 규칙 | 심각도 | 자동 수정 | 설명 |
|------|--------|-----------|------|
| `code-block` | warning | 불가 | 코드 블록 언어 지정 누락 |
| `blank-line` | warning | 가능 | 연속 빈 줄 제한 |
| `broken-link` | error | 불가 | 앵커/상대/외부 링크 검증 |
| TOC 검사 | info | 불가 | 헤딩 수 대비 목차 존재 |

---

## Git 훅 연동

```bash
# .git/hooks/pre-commit
#!/bin/sh
stylemd check --git-staged           # 빠른 검사
# stylemd check --deep --git-staged  # 정밀 검사
```

---

## Playground

```
playground/
├── all_issues.md           # 모든 이슈 유형 통합
├── blank_lines.md          # 빈 줄 규칙 위반
├── broken_links.md         # 깨진 앵커/상대 링크
├── clean.md                # 이슈 없는 파일
├── code_blocks.md          # 언어 누락, 미닫힌 블록
├── heading_structure.md    # H1 중복, 레벨 스킵
├── image_refs.md           # 이미지 파일 누락, alt 텍스트 누락
├── list_style.md           # 기호 혼용, 들여쓰기 오류
└── trailing_whitespace.md  # 후행 공백 및 탭
```

---

## 활용 팁

실무에서 자주 쓰는 명령어를 상황별로 정리했습니다.

### 일상 작업

```bash
# 현재 디렉터리의 모든 마크다운을 빠르게 검사
stylemd check

# 특정 파일 하나만 검사
stylemd check docs/guide.md

# docs 디렉터리 전체 검사
stylemd check docs/

# 여러 디렉터리를 한 번에 검사하고 싶을 때
stylemd check docs/ && stylemd check wiki/

# 프로젝트 전체 품질 점수를 한눈에 확인
stylemd stats
```

### 자동 수정

```bash
# 문제가 있는 파일을 한 번에 자동 수정
# 자동 수정 항목:
#   - 후행 공백/탭 제거
#   - 헤딩 앞뒤 빈 줄 자동 삽입
#   - 연속 빈 줄 정리 (--deep 모드)
#   - 목록 기호 통일 (*/+ → -)
#   - 미닫힌 코드 블록 닫기 (``` 추가)
stylemd fix

# 특정 디렉터리만 수정
stylemd fix docs/

# 수정 전에 어떤 부분이 바뀌는지 diff로 미리 확인
stylemd fix --diff --dry-run

# 특정 파일의 변경 내용만 미리보기
stylemd fix --diff --dry-run docs/api.md

# deep 모드로 수정 (연속 빈 줄 정리까지 포함)
stylemd fix --deep

# 포매팅만 적용 (빈 줄 정리, 후행 공백 제거, 목록 기호 통일)
stylemd fmt
stylemd fmt docs/
```

### 문서 작성 중 활용

```bash
# 글 쓰면서 실시간으로 검사 결과 확인
stylemd check --watch docs/

# README 수정 중에 계속 검사
stylemd check --watch README.md

# 글 다 쓴 후 한 번에 정리
stylemd fix docs/writing-guide.md

# 정리 후 깨끗한지 확인
stylemd check docs/writing-guide.md
```

### 프로젝트 구조 관리

```bash
# 프로젝트 전체에서 이미지 참조가 깨진 파일 찾기
stylemd check . 2>&1 | grep "image"

# 특정 폴더는 제외하고 검사
stylemd check --ignore-pattern "vendor/**"

# node_modules, build 산출물 제외
stylemd check --ignore-pattern "node_modules/**"

# 복잡한 제외 패턴은 설정 파일 사용
cat > .stylemd.toml << 'EOF'
ignore = ["node_modules/**", "vendor/**", "dist/**", "build/**"]
EOF
stylemd check
```

### 커밋 전 검사

```bash
# git add한 파일만 빠르게 검사
stylemd check --git-staged

# 커밋 전 자동 검사 (pre-commit hook)
echo '#!/bin/sh
stylemd check --git-staged' > .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### 프로젝트 품질 확인

```bash
# 전체 프로젝트 품질 점수 확인
stylemd stats

# 어떤 규칙이 가장 많이 위반되는지 확인
stylemd stats --deep

# JSON으로 뽑아서 대시보드에 연동
stylemd stats --report json > quality.json
```

### 링크 검사 (deep)

```bash
# 문서 내 모든 링크가 유효한지 검사 (앵커, 상대 경로, 외부 URL)
stylemd links

# 깨진 링크만 빠르게 필터링
stylemd links docs/ 2>&1 | grep "error"
```

### CI/CD 파이프라인

```bash
# CI에서 검사 실패 시 빌드 중단 (exit code 1)
stylemd check --deep

# JSON 결과를 CI 아티팩트로 저장
stylemd check --deep --format json > lint-results.json

# YAML로 뽑아서 다른 도구와 연동
stylemd check --format yaml > lint-results.yaml

# Markdown 리포트 생성
stylemd stats --report md > LINT_REPORT.md

# HTML 리포트 생성 (PR 코멘트나 Slack 공유용)
stylemd stats --deep --report html > report.html
```

### 파이프 조합

```bash
# stdin으로 마크다운 검사
cat README.md | stylemd check -

# 에러만 필터링
stylemd check docs/ 2>&1 | grep "error"

# 이슈 개수 세기
stylemd check . 2>&1 | grep -c "warning\|error"

# 특정 규칙 위반만 보기
stylemd check . 2>&1 | grep "trailing-whitespace"

# JSON 출력 + jq로 에러만 추출
stylemd check --format json . | jq '.[] | select(.severity == "error")'

# 수정 가능한 이슈만 보기
stylemd check --format json . | jq '.[] | select(.fixable == true)'
```

### 실시간 감시

```bash
# 파일 변경 시 자동으로 재검사 (글쓰기 중 사용)
stylemd check --watch docs/

# deep 모드로 실시간 감시
stylemd check --deep --watch .
```

### 특정 파일/폴더 제외

```bash
# node_modules 제외
stylemd check --ignore-pattern "node_modules/**"

# 여러 패턴 제외는 설정 파일 사용
cat > .stylemd.toml << 'EOF'
ignore = ["node_modules/**", "vendor/**", "dist/**"]
EOF
stylemd check
```

### 한국어 출력

```bash
# 모든 메시지를 한국어로 출력
stylemd --lang ko check
stylemd --lang ko stats
stylemd --lang ko fix --dry-run
```

### 설정 파일

```bash
# 기본 설정 파일 생성
stylemd init

# 커스텀 설정으로 검사
stylemd check -c my-config.toml docs/

# 프로젝트별 설정 예시: 이미지 검사 끄기, 목록 기호를 *로 변경
cat > .stylemd.toml << 'EOF'
[light]
image_file_exists = false
list_marker = "*"

[deep]
external_links = false
EOF
```

---

## 라이선스

[MIT](LICENSE)
