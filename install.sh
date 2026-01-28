#!/usr/bin/env sh
set -eu

OWNER="${PRISMCTL_GITHUB_OWNER:-thornboo}"
REPO="${PRISMCTL_GITHUB_REPO:-prismctl}"
VERSION="${PRISMCTL_VERSION:-latest}"
INSTALL_DIR="${PRISMCTL_INSTALL_DIR:-"$HOME/.local/bin"}"
NO_VERIFY="${PRISMCTL_NO_VERIFY:-0}"

fail() {
  echo "error: $*" >&2
  exit 1
}

command_exists() {
  command -v "$1" >/dev/null 2>&1
}

download() {
  url="$1"
  out="$2"

  if command_exists curl; then
    curl -fsSL "$url" -o "$out"
    return 0
  fi
  if command_exists wget; then
    wget -qO "$out" "$url"
    return 0
  fi
  fail "missing downloader: install curl or wget"
}

download_text() {
  url="$1"
  tmp="$2"
  download "$url" "$tmp"
  cat "$tmp"
}

file_sha256() {
  f="$1"
  if command_exists sha256sum; then
    sha256sum "$f" | awk '{print $1}'
    return 0
  fi
  if command_exists shasum; then
    shasum -a 256 "$f" | awk '{print $1}'
    return 0
  fi
  return 1
}

detect_target() {
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Darwin)
      case "$arch" in
        x86_64) echo "x86_64-apple-darwin" ;;
        arm64) echo "aarch64-apple-darwin" ;;
        *) fail "unsupported macOS arch: $arch" ;;
      esac
      ;;
    Linux)
      case "$arch" in
        x86_64) echo "x86_64-unknown-linux-gnu" ;;
        *)
          fail "unsupported linux arch: $arch (try: cargo install prismctl)"
          ;;
      esac
      ;;
    *)
      fail "unsupported OS: $os"
      ;;
  esac
}

resolve_tag() {
  v="$1"
  if [ "$v" = "latest" ]; then
    api="https://api.github.com/repos/$OWNER/$REPO/releases/latest"
    tag="$(
      download_text "$api" "$TMPDIR/release.json" \
        | sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' \
        | head -n 1
    )"
    [ -n "$tag" ] || fail "failed to resolve latest release tag from GitHub API"
    echo "$tag"
    return 0
  fi

  case "$v" in
    v*) echo "$v" ;;
    *) echo "v$v" ;;
  esac
}

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT INT TERM

TARGET="$(detect_target)"
TAG="$(resolve_tag "$VERSION")"

ARCHIVE="prismctl-$TAG-$TARGET.tar.gz"
BASE_URL="https://github.com/$OWNER/$REPO/releases/download/$TAG"
ARCHIVE_URL="$BASE_URL/$ARCHIVE"

echo "Installing prismctl $TAG ($TARGET) to $INSTALL_DIR"

mkdir -p "$INSTALL_DIR"

download "$ARCHIVE_URL" "$TMPDIR/$ARCHIVE"

if [ "$NO_VERIFY" != "1" ]; then
  if download "$ARCHIVE_URL.sha256" "$TMPDIR/$ARCHIVE.sha256" 2>/dev/null; then
    if expected="$(awk '{print $1}' "$TMPDIR/$ARCHIVE.sha256")" && [ -n "$expected" ]; then
      if actual="$(file_sha256 "$TMPDIR/$ARCHIVE")"; then
        if [ "$expected" != "$actual" ]; then
          fail "sha256 mismatch for $ARCHIVE (expected $expected, got $actual)"
        fi
      else
        echo "warning: missing sha256 tool (sha256sum/shasum); skipping verification" >&2
      fi
    fi
  else
    echo "warning: checksum file not found; skipping verification" >&2
  fi
fi

tar -xzf "$TMPDIR/$ARCHIVE" -C "$TMPDIR"
[ -f "$TMPDIR/prismctl" ] || fail "archive did not contain expected binary: prismctl"

if command_exists install; then
  install -m 0755 "$TMPDIR/prismctl" "$INSTALL_DIR/prismctl"
else
  cp "$TMPDIR/prismctl" "$INSTALL_DIR/prismctl"
  chmod 0755 "$INSTALL_DIR/prismctl"
fi

echo "Done: $INSTALL_DIR/prismctl"
if ! command_exists prismctl; then
  echo "Note: '$INSTALL_DIR' may not be on your PATH." >&2
fi
