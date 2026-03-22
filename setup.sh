#!/usr/bin/env bash
set -e

echo "==> Installing Rust..."
if ! command -v rustup &>/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
else
  echo "    rustup already installed, skipping"
fi

echo "==> Installing wasm-pack..."
if ! command -v wasm-pack &>/dev/null; then
  cargo install wasm-pack
else
  echo "    wasm-pack already installed, skipping"
fi

echo "==> Installing just..."
if ! command -v just &>/dev/null; then
  cargo install just
else
  echo "    just already installed, skipping"
fi

echo "==> Installing pnpm..."
if ! command -v pnpm &>/dev/null; then
  curl -fsSL https://get.pnpm.io/install.sh | sh -
  source "$HOME/.local/share/pnpm/env" 2>/dev/null || true
else
  echo "    pnpm already installed, skipping"
fi

echo ""
echo "All prerequisites installed. Run 'pnpm install' next to install frontend deps."
