#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "QuantumEdge bootstrap"
python3 -m venv "$ROOT_DIR/services/python-optimizer/.venv"
source "$ROOT_DIR/services/python-optimizer/.venv/bin/activate"
pip install --upgrade pip
pip install -e "$ROOT_DIR/services/python-optimizer"
deactivate

cargo check --manifest-path "$ROOT_DIR/services/rust-api/Cargo.toml"
cd "$ROOT_DIR/apps/web"
npm install
npm run build
