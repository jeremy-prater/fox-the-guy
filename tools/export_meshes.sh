#!/usr/bin/env bash
# Export mesh objects from a .blend to assets/meshes/*.glb (CI / local).
#
# Usage:
#   tools/export_meshes.sh
#   tools/export_meshes.sh path/to/scene.blend
#
# Env:
#   BLENDER         Blender binary (default: blender)
#   BLENDER_BLEND   Override default .blend
#   FOX_MESHES_DIR  Forwarded to the Python export script
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

BLENDER_BIN="${BLENDER:-blender}"
DEFAULT_BLEND="$ROOT/content/blender/city-template.blend"
BLEND="${1:-${BLENDER_BLEND:-$DEFAULT_BLEND}}"

if [[ ! -f "$BLEND" ]]; then
  echo "error: blend file not found: $BLEND" >&2
  exit 1
fi

if ! command -v "$BLENDER_BIN" >/dev/null 2>&1; then
  echo "error: Blender not found as '$BLENDER_BIN' (set BLENDER=...)" >&2
  exit 1
fi

exec "$BLENDER_BIN" "$BLEND" --background --python "$ROOT/tools/export_meshes_to_glb.py"
