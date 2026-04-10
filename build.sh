#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")" && pwd)"
BINARY_NAME="stylemd"
INSTALL_DIR="/usr/local/bin"
GITHUB_REPO="leaf-kit/style.md"
HOMEBREW_TAP_REPO="leaf-kit/homebrew-stylemd"

cd "$PROJECT_ROOT"

get_version() {
    grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/'
}

bump_patch_version() {
    local current
    current=$(get_version)
    local major minor patch
    IFS='.' read -r major minor patch <<< "$current"
    patch=$((patch + 1))
    local new_version="${major}.${minor}.${patch}"
    sed -i '' "s/^version = \"${current}\"/version = \"${new_version}\"/" Cargo.toml
    echo ">> Version bumped: ${current} -> ${new_version}"
}

show_menu() {
    local ver
    ver=$(get_version)
    echo "=================================="
    echo "  stylemd v${ver} — Build & Distribution"
    echo "=================================="
    echo ""
    echo "  1) Build (debug, clean)"
    echo "  2) Build (release)"
    echo "  3) Build & Install locally"
    echo "  4) Run tests"
    echo "  5) Run clippy (lint)"
    echo "  6) Clean build artifacts"
    echo "  7) Build all platforms (cross)"
    echo "  8) Create release tarball"
    echo "  9) Deploy to Homebrew (full pipeline)"
    echo "  0) Exit"
    echo ""
    echo -n "  Select: "
}

build_debug() {
    echo ">> Cleaning previous build..."
    cargo clean
    echo ">> Building debug (clean)..."
    cargo build
    echo ">> Done: target/debug/$BINARY_NAME"
}

require_tests() {
    echo ">> Running tests (required before release)..."
    if ! cargo test; then
        echo "!! Tests failed. Aborting."
        return 1
    fi
    echo ">> Running clippy..."
    if ! cargo clippy -- -D warnings; then
        echo "!! Clippy found issues. Aborting."
        return 1
    fi
    echo ">> All checks passed."
}

build_release() {
    require_tests || return 1
    bump_patch_version
    echo ">> Building release..."
    cargo build --release
    echo ">> Done: target/release/$BINARY_NAME (v$(get_version))"
}

install_local() {
    build_release || return 1
    echo ">> Installing to $INSTALL_DIR/$BINARY_NAME"
    if [ -w "$INSTALL_DIR" ]; then
        cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    else
        echo ">> Requires sudo for $INSTALL_DIR"
        sudo cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    fi
    echo ">> Installed. Run: $BINARY_NAME --help"
}

run_tests() {
    echo ">> Running tests..."
    cargo test
    echo ">> Tests complete."
}

run_clippy() {
    echo ">> Running clippy..."
    cargo clippy -- -W clippy::all
    echo ">> Clippy complete."
}

clean() {
    echo ">> Cleaning cargo build artifacts..."
    cargo clean
    echo ">> Clearing Homebrew cache for $BINARY_NAME..."
    brew cleanup -s "$BINARY_NAME" 2>/dev/null || true
    brew cleanup -s 2>/dev/null || true
    echo ">> Clean complete."
}

# ══════════════════════════════════════════════════════════════
#  Cross-platform build
#
#  Builds for all available targets on the current machine.
#  macOS: aarch64-apple-darwin, x86_64-apple-darwin
#  Linux: x86_64-unknown-linux-gnu (+ aarch64 if cross tools available)
# ══════════════════════════════════════════════════════════════
build_cross() {
    require_tests || return 1

    local targets=()
    local os
    os="$(uname -s)"

    if [[ "$os" == "Darwin" ]]; then
        targets+=("aarch64-apple-darwin" "x86_64-apple-darwin")
    elif [[ "$os" == "Linux" ]]; then
        targets+=("x86_64-unknown-linux-gnu")
        if command -v aarch64-linux-gnu-gcc &>/dev/null; then
            targets+=("aarch64-unknown-linux-gnu")
        fi
    fi

    echo ""
    echo ">> Building for ${#targets[@]} target(s)..."
    echo ""

    for target in "${targets[@]}"; do
        echo "── $target ──"

        # Ensure target is installed
        if ! rustup target list --installed | grep -q "$target"; then
            echo "   Installing target..."
            rustup target add "$target"
        fi

        echo "   Building release..."
        cargo build --release --target "$target"

        local binary="target/$target/release/$BINARY_NAME"
        if [ -f "$binary" ]; then
            local size
            size=$(du -h "$binary" | awk '{print $1}')
            echo "   Done: $binary ($size)"
        else
            echo "   !! Build failed for $target"
        fi
    done

    echo ""
    echo ">> All builds complete."
    echo ""

    # Create tarballs
    for target in "${targets[@]}"; do
        local binary="target/$target/release/$BINARY_NAME"
        if [ -f "$binary" ]; then
            local tarball="$BINARY_NAME-$target.tar.gz"
            local staging
            staging=$(mktemp -d)
            cp "$binary" "$staging/"
            cp README.md LICENSE "$staging/" 2>/dev/null || true
            tar -czf "$tarball" -C "$staging" .
            rm -rf "$staging"
            local sha
            sha=$(shasum -a 256 "$tarball" | awk '{print $1}')
            echo "  $tarball"
            echo "    SHA256: $sha"
        fi
    done
}

create_tarball() {
    build_release || return 1

    VERSION=$(get_version)
    ARCH="$(uname -m)"
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    TARBALL="$BINARY_NAME-$VERSION-$ARCH-$OS.tar.gz"

    echo ">> Creating release tarball: $TARBALL"

    STAGING=$(mktemp -d)
    cp "target/release/$BINARY_NAME" "$STAGING/"
    cp README.md LICENSE "$STAGING/" 2>/dev/null || true

    tar -czf "$TARBALL" -C "$STAGING" .
    rm -rf "$STAGING"

    echo ">> Done: $TARBALL"
    echo ">> SHA256: $(shasum -a 256 "$TARBALL" | awk '{print $1}')"
}

# ══════════════════════════════════════════════════════════════
#  Deploy to Homebrew (Pre-built Binary Formula)
#
#  Steps:
#    1. Run tests + clippy
#    2. Bump version + tag
#    3. Cross-compile for all platforms
#    4. Create tarballs + upload to GitHub Release
#    5. Download from CDN + compute SHA256 per platform
#    6. Generate Formula/stylemd.rb (binary download)
#    7. Push Formula to homebrew-stylemd tap
#    8. Verify: untap → tap → install → version check
# ══════════════════════════════════════════════════════════════
deploy_homebrew() {
    require_tests || return 1

    bump_patch_version
    VERSION=$(get_version)
    TAG="v${VERSION}"

    echo ""
    echo "══════════════════════════════════════"
    echo "  Deploying stylemd ${VERSION} to Homebrew"
    echo "  (pre-built binary Formula)"
    echo "══════════════════════════════════════"
    echo ""

    # ── Step 1: Cross-compile ──
    echo "[1/8] Building for all available platforms..."

    local targets=()
    local os
    os="$(uname -s)"

    if [[ "$os" == "Darwin" ]]; then
        for t in aarch64-apple-darwin x86_64-apple-darwin; do
            rustup target list --installed | grep -q "$t" || rustup target add "$t"
            targets+=("$t")
        done
    elif [[ "$os" == "Linux" ]]; then
        targets+=("x86_64-unknown-linux-gnu")
        if command -v aarch64-linux-gnu-gcc &>/dev/null; then
            rustup target list --installed | grep -q aarch64-unknown-linux-gnu || rustup target add aarch64-unknown-linux-gnu
            targets+=("aarch64-unknown-linux-gnu")
        fi
    fi

    for target in "${targets[@]}"; do
        echo "   Building $target..."
        cargo build --release --target "$target"
    done
    echo "        Done."

    # ── Step 2: Create tarballs ──
    echo "[2/8] Creating release tarballs..."
    local assets=""
    for target in "${targets[@]}"; do
        local binary="target/$target/release/$BINARY_NAME"
        if [ -f "$binary" ]; then
            local tarball="$BINARY_NAME-$target.tar.gz"
            rm -f "$tarball"
            local staging
            staging=$(mktemp -d)
            cp "$binary" "$staging/"
            cp README.md README_en.md LICENSE "$staging/" 2>/dev/null || true
            tar -czf "$tarball" -C "$staging" .
            rm -rf "$staging"
            assets="$assets $tarball"
            echo "   $tarball ($(shasum -a 256 "$tarball" | awk '{print $1}'))"
        fi
    done
    echo "        Done."

    # ── Step 3: Create GitHub release ──
    echo "[3/8] Creating GitHub release ${TAG}..."
    gh release delete "$TAG" --yes --cleanup-tag 2>/dev/null || true
    git tag -d "$TAG" 2>/dev/null || true

    git tag "$TAG"
    git push origin "$TAG" 2>/dev/null

    # shellcheck disable=SC2086
    gh release create "$TAG" $assets \
        --title "$TAG" \
        --notes "stylemd $VERSION — \`brew tap leaf-kit/stylemd && brew install stylemd\`"
    echo "        Done."

    # ── Step 4: Download from CDN + compute SHA256 ──
    echo "[4/8] Downloading from CDN to verify SHA256..."
    sleep 5

    VERIFY_DIR=$(mktemp -d)
    declare -A SHAS

    for target in "${targets[@]}"; do
        local tarball="$BINARY_NAME-$target.tar.gz"
        local url="https://github.com/${GITHUB_REPO}/releases/download/${TAG}/${tarball}"

        for attempt in 1 2 3; do
            curl -sL "$url" -o "$VERIFY_DIR/$tarball"
            local size
            size=$(wc -c < "$VERIFY_DIR/$tarball" | tr -d ' ')
            if [ "$size" -gt 1000 ]; then
                SHAS[$target]=$(shasum -a 256 "$VERIFY_DIR/$tarball" | awk '{print $1}')
                echo "   $target: ${SHAS[$target]}"
                break
            fi
            echo "   $target: attempt $attempt failed ($size bytes), retrying..."
            sleep 5
        done

        if [ -z "${SHAS[$target]:-}" ]; then
            echo "!! Failed to get SHA for $target. Aborting."
            rm -rf "$VERIFY_DIR"
            return 1
        fi
    done
    rm -rf "$VERIFY_DIR"
    echo "        Done."

    # ── Step 5: Generate Formula (binary) ──
    echo "[5/8] Generating Formula/stylemd.rb (binary download)..."
    mkdir -p Formula

    local BASE="https://github.com/${GITHUB_REPO}/releases/download/${TAG}"

    local MAC_ARM_SHA="${SHAS[aarch64-apple-darwin]:-TODO}"
    local MAC_X86_SHA="${SHAS[x86_64-apple-darwin]:-TODO}"
    local LINUX_X86_SHA="${SHAS[x86_64-unknown-linux-gnu]:-TODO}"
    local LINUX_ARM_SHA="${SHAS[aarch64-unknown-linux-gnu]:-TODO}"

    cat > Formula/stylemd.rb << FORMULA
class Stylemd < Formula
  desc "Markdown styling toolkit — lint, format, fix, and polish your prose"
  homepage "https://github.com/${GITHUB_REPO}"
  version "${VERSION}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "${BASE}/stylemd-aarch64-apple-darwin.tar.gz"
      sha256 "${MAC_ARM_SHA}"
    else
      url "${BASE}/stylemd-x86_64-apple-darwin.tar.gz"
      sha256 "${MAC_X86_SHA}"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "${BASE}/stylemd-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "${LINUX_ARM_SHA}"
    else
      url "${BASE}/stylemd-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "${LINUX_X86_SHA}"
    end
  end

  def install
    bin.install "stylemd"
  end

  test do
    assert_match "stylemd ${VERSION}", shell_output("\#{bin}/stylemd --version")

    (testpath/"test.md").write("# Title\ntext   \n")
    output = shell_output("\#{bin}/stylemd check \#{testpath}/test.md 2>&1")
    assert_match "trailing", output.downcase

    system bin/"stylemd", "init"
    assert_predicate testpath/".stylemd.toml", :exist?
  end
end
FORMULA
    echo "        Done."

    # ── Step 6: Commit Formula ──
    echo "[6/8] Committing Formula..."
    git add Formula/stylemd.rb
    if ! git diff --cached --quiet; then
        git commit -m "Formula: update to ${TAG} (binary)"
        git push origin main 2>/dev/null || true
    fi
    echo "        Done."

    # ── Step 7: Push to homebrew tap ──
    echo "[7/8] Pushing Formula to homebrew-stylemd tap..."
    TAP_DIR=$(mktemp -d)
    if gh repo clone "$HOMEBREW_TAP_REPO" "$TAP_DIR/hb" 2>/dev/null; then
        mkdir -p "$TAP_DIR/hb/Formula"
        cp Formula/stylemd.rb "$TAP_DIR/hb/Formula/stylemd.rb"
        (
            cd "$TAP_DIR/hb"
            git add Formula/stylemd.rb
            if ! git diff --cached --quiet; then
                git commit -m "Update stylemd to ${TAG}"
                git push origin main
            else
                echo "        Tap formula unchanged."
            fi
        )
    else
        echo "        !! Could not clone tap repo. Push Formula manually."
    fi
    rm -rf "$TAP_DIR"
    echo "        Done."

    # ── Step 8: Verify ──
    echo "[8/8] Verifying brew installation..."
    brew uninstall "$BINARY_NAME" 2>/dev/null || true
    brew untap leaf-kit/stylemd 2>/dev/null || true
    brew cleanup -s 2>/dev/null || true
    brew tap leaf-kit/stylemd

    if brew install "$BINARY_NAME"; then
        INSTALLED_VERSION=$("$BINARY_NAME" --version 2>/dev/null || echo "unknown")
        if echo "$INSTALLED_VERSION" | grep -q "$VERSION"; then
            echo ""
            echo "══════════════════════════════════════"
            echo "  Deploy successful!"
            echo "  Version:  ${INSTALLED_VERSION}"
            echo "  Formula:  pre-built binary"
            echo "  Platforms:"
            for target in "${targets[@]}"; do
                echo "    - $target (${SHAS[$target]})"
            done
            echo ""
            echo "  brew tap leaf-kit/stylemd"
            echo "  brew install stylemd"
            echo "══════════════════════════════════════"
        else
            echo ""
            echo "!! WARNING: Installed version mismatch!"
            echo "   Expected: ${VERSION}"
            echo "   Got:      ${INSTALLED_VERSION}"
        fi
    else
        echo ""
        echo "!! brew install failed."
        echo "   brew untap leaf-kit/stylemd && brew tap leaf-kit/stylemd && brew install stylemd"
    fi

    # Clean up tarballs
    rm -f $BINARY_NAME-*.tar.gz
}

while true; do
    show_menu
    read -r choice
    echo ""

    case $choice in
        1) build_debug ;;
        2) build_release ;;
        3) install_local ;;
        4) run_tests ;;
        5) run_clippy ;;
        6) clean ;;
        7) build_cross ;;
        8) create_tarball ;;
        9) deploy_homebrew ;;
        0) echo "Bye."; exit 0 ;;
        *) echo "Invalid selection." ;;
    esac

    echo ""
done
