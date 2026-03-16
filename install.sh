#!/usr/bin/env bash
set -euo pipefail

echo "تثبيت أدوات Bakir-Core..."

command -v cargo >/dev/null 2>&1 || {
  echo "تثبيت Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . "$HOME/.cargo/env"
}

TMP=$(mktemp -d)
cd "$TMP"
curl -L https://github.com/abuhussen/Bakir-Core/archive/refs/heads/main.tar.gz | tar xz --strip-components=1

for d in bakir-*; do
  [ -d "$d" ] || continue
  [ -f "$d/Cargo.toml" ] || { echo "تحذير: $d بدون Cargo.toml"; continue; }
  echo "بناء $d..."
  cd "$d"
  cargo build --release
  BIN=$(grep '^name\s*=' Cargo.toml | head -1 | cut -d'"' -f2)
  sudo install -m755 "target/release/$BIN" "/usr/local/bin/$BIN"
  cd ..
done

rm -rf "$TMP"
echo "تم التثبيت!"
