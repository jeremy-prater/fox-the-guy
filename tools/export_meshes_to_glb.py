"""
Export each mesh object in the current Blender scene to assets/meshes/*.glb.

Usage:

    # CI / headless (preferred; defaults to content/blender/city-template.blend):
    tools/export_meshes.sh

    # Or directly:
    blender content/blender/city-template.blend --background --python tools/export_meshes_to_glb.py

    # Interactive Blender session:
    exec(open("tools/export_meshes_to_glb.py").read())

Optional env:
    FOX_MESHES_DIR  Override output directory (default: <repo>/assets/meshes)
"""

from __future__ import annotations

import os
import re
from pathlib import Path

import bpy

def _start_paths() -> list[Path]:
    paths: list[Path] = []
    try:
        paths.append(Path(__file__).resolve().parent)
    except NameError:
        pass
    paths.append(Path.cwd())
    blend = bpy.data.filepath
    if blend:
        paths.append(Path(blend).resolve().parent)
    return paths


def repo_root() -> Path:
    """Resolve workspace root from this file, cwd, or Blender blend path."""
    for start in _start_paths():
        for candidate in (start, *start.parents):
            if (candidate / "assets" / "meshes").is_dir():
                return candidate

    raise RuntimeError(
        "Could not find assets/meshes. Set FOX_MESHES_DIR or run from the repo."
    )


def output_dir() -> Path:
    env = os.environ.get("FOX_MESHES_DIR")
    if env:
        path = Path(env).expanduser().resolve()
        path.mkdir(parents=True, exist_ok=True)
        return path
    path = repo_root() / "assets" / "meshes"
    path.mkdir(parents=True, exist_ok=True)
    return path


def to_kebab(name: str) -> str:
    """CIBoT_Core / Inner Ring -> cibot-core / inner-ring."""
    s = name.strip().replace(" ", "_")
    return re.sub(r"_+", "-", s).lower().strip("-")


def mesh_objects() -> list[bpy.types.Object]:
    return sorted(
        (obj for obj in bpy.data.objects if obj.type == "MESH"),
        key=lambda o: o.name,
    )


def export_object(obj: bpy.types.Object, dest: Path) -> None:
    bpy.ops.object.select_all(action="DESELECT")
    obj.hide_set(False)
    obj.hide_viewport = False
    obj.hide_render = False
    obj.select_set(True)
    bpy.context.view_layer.objects.active = obj

    bpy.ops.export_scene.gltf(
        filepath=str(dest),
        check_existing=False,
        use_selection=True,
        use_visible=False,
        use_active_collection=False,
        export_format="GLB",
        export_apply=True,
        export_yup=True,
        export_materials="EXPORT",
        export_extras=True,
    )


def main() -> None:
    out = output_dir()
    exported: list[str] = []

    for obj in mesh_objects():
        dest = out / f"{to_kebab(obj.name)}.glb"
        export_object(obj, dest)
        exported.append(f"{obj.name} -> {dest}")

    bpy.ops.object.select_all(action="DESELECT")
    print(f"Exported {len(exported)} mesh(es) to {out}")
    for line in exported:
        print(f"  {line}")


main()
