"""
Export the entire Blender scene as a single .glb under assets/meshes/.

Usage:

    # CI / headless (preferred; defaults to content/blender/cibot-ziggurat.blend):
    tools/export_scene.sh

    # Or directly:
    blender content/blender/cibot-ziggurat.blend --background --python tools/export_scene_to_glb.py

    # Interactive Blender session:
    exec(open("tools/export_scene_to_glb.py").read())

Optional env:
    FOX_MESHES_DIR   Override output directory (default: <repo>/assets/meshes)
    FOX_SCENE_GLB    Override output filename (default: <blend-stem>.glb)
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
    """CIBoT_Ziggurat / Inner Ring -> cibot-ziggurat / inner-ring."""
    s = name.strip().replace(" ", "_")
    return re.sub(r"_+", "-", s).lower().strip("-")


def output_path() -> Path:
    env = os.environ.get("FOX_SCENE_GLB")
    if env:
        path = Path(env).expanduser().resolve()
        path.parent.mkdir(parents=True, exist_ok=True)
        return path

    blend = bpy.data.filepath
    if blend:
        stem = to_kebab(Path(blend).stem)
    else:
        stem = to_kebab(bpy.context.scene.name or "scene")

    return output_dir() / f"{stem}.glb"


def prepare_export() -> None:
    """Unhide mesh objects so the full scene is included."""
    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        obj.hide_set(False)
        obj.hide_viewport = False
        obj.hide_render = False


def export_scene(dest: Path) -> None:
    prepare_export()
    bpy.ops.object.select_all(action="DESELECT")

    bpy.ops.export_scene.gltf(
        filepath=str(dest),
        check_existing=False,
        use_selection=False,
        use_visible=True,
        use_active_collection=False,
        export_format="GLB",
        export_apply=True,
        export_yup=True,
        export_materials="EXPORT",
        export_extras=True,
        export_cameras=False,
        export_lights=False,
    )


def main() -> None:
    dest = output_path()
    mesh_count = sum(1 for o in bpy.data.objects if o.type == "MESH")
    export_scene(dest)
    print(f"Exported scene ({mesh_count} mesh object(s)) -> {dest}")


main()
