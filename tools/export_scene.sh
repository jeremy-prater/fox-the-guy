#!/usr/bin/env bash
# Export an entire .blend scene to a single assets/meshes/*.glb (CI / local).
#
# Usage:
#   tools/export_scene.sh
#   tools/export_scene.sh path/to/scene.blend
#
# Env:
#   BLENDER         Blender binary (default: blender)
#   BLENDER_BLEND   Override default .blend
#   FOX_MESHES_DIR  Forwarded to the Python export script
#   FOX_SCENE_GLB   Forwarded output path override
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

DEFAULT_BLEND="$ROOT/content/blender/cibot-ziggurat.blend"
BLEND="${1:-${BLENDER_BLEND:-$DEFAULT_BLEND}}"

if [[ -n "${BLENDER:-}" ]]; then
  BLENDER_BIN="$BLENDER"
elif command -v blender >/dev/null 2>&1; then
  BLENDER_BIN="$(command -v blender)"
elif [[ -x /Applications/Blender.app/Contents/MacOS/Blender ]]; then
  BLENDER_BIN="/Applications/Blender.app/Contents/MacOS/Blender"
else
  echo "error: Blender not found (set BLENDER=...)" >&2
  exit 1
fi

if [[ ! -f "$BLEND" ]]; then
  echo "error: blend file not found: $BLEND" >&2
  exit 1
fi

exec "$BLENDER_BIN" "$BLEND" --background --python "$ROOT/tools/export_scene_to_glb.py"
