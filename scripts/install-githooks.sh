#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "${repo_root}"

git config core.hooksPath "scripts/githooks"
chmod +x "scripts/githooks/pre-commit"

echo "Installed git hooks via core.hooksPath=scripts/githooks"

