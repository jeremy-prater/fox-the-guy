# World object placement schema

JSON schema for placing mesh instances in the world, including a small layout DSL that expands one prototype into many transforms along points, lines, circles/arcs, and splines.

**No procedural mesh generation** — only stamping existing mesh assets.

## Why JSON

| Factor | Choice | Rationale |
|--------|--------|-----------|
| Load path | JSON | `fox_generation` already uses `serde` + `serde_json`; YAML cousins are authoring-only |
| Data shape | JSON | IDs, numbers, and tagged layout objects parse cleanly with serde enums |
| Procedural repeats | JSON | A tagged layout DSL expands to many instances without hand-writing each transform |
| Narrative / TTS cards | YAML | Keep under `story/npc/cousins/` |

**Split:** JSON = runtime/game data (including this DSL); YAML = narrative/TTS cards.

## Pipeline

```mermaid
flowchart LR
  json[World JSON] --> decode[WorldFile::from_json]
  decode --> expand[expand layout]
  expand --> instances["id_i + Transform"]
  instances --> spawn[spawn_world]
  spawn --> comps[insert ComponentSpec match arms]
```

Author one **prototype** (`id`, `mesh`, `layout`, `components`) plus a **layout** that says where to stamp copies. At load time, expand layouts into concrete transforms and spawn Bevy entities.

Fits map fiction in [`story/map_sizes.md`](../story/map_sizes.md): ring walls as arcs at fixed radii; facade brick rows as lines; curvy roads as splines.

## File shape

One level (or region) file with a `placements` array. Every entry has `id`, `mesh`, tagged `layout`, and an optional `components` array.

```json
{
  "version": 1,
  "placements": [
    {
      "id": "works_barrel_01",
      "mesh": "meshes/barrel",
      "layout": {
        "kind": "point",
        "translation": [12.0, 0.0, -3.5],
        "rotation": [0.0, 0.7071, 0.0, 0.7071],
        "scale": [1.0, 1.0, 1.0]
      },
      "components": [
        { "type": "health", "value": 40 },
        { "type": "texture", "value": "textures/barrel_tagged" },
        { "type": "trigger", "on": "interact", "event": "jam_or_leave" }
      ]
    },
    {
      "id": "hospice_facade_bricks",
      "mesh": "meshes/brick",
      "layout": {
        "kind": "line",
        "from": [10.0, 0.0, 5.0],
        "to": [40.0, 0.0, 5.0],
        "count": 30,
        "facing": "along"
      },
      "components": [
        { "type": "texture", "value": "textures/brick_cobble" }
      ]
    },
    {
      "id": "outer_ring_wall",
      "mesh": "meshes/wall_segment",
      "layout": {
        "kind": "circle",
        "center": [0.0, 0.0, 0.0],
        "radius": 10000.0,
        "count": 720,
        "start_angle_deg": -60.0,
        "end_angle_deg": 60.0,
        "facing": "outward"
      },
      "components": [
        { "type": "texture", "value": "textures/ring_facade" }
      ]
    },
    {
      "id": "canal_road",
      "mesh": "meshes/road_tile",
      "layout": {
        "kind": "spline",
        "points": [
          [0.0, 0.0, 0.0],
          [10.0, 0.0, 5.0],
          [25.0, 0.0, 8.0],
          [40.0, 0.0, 4.0]
        ],
        "spacing": 2.0,
        "facing": "along"
      }
    },
    {
      "id": "cousin_greeter",
      "mesh": "meshes/fox_npc",
      "layout": {
        "kind": "point",
        "translation": [0.0, 0.0, 0.0]
      },
      "components": [
        { "type": "person" },
        { "type": "name", "value": "John Doe" }
      ]
    }
  ]
}
```

## Components → Bevy ECS

Bevy components are **compile-time Rust types**. JSON cannot invent new structs at runtime. World JSON uses a closed tagged enum `ComponentSpec`; `spawn_world` matches each spec onto a real `#[derive(Component)]` type.

| JSON `type` | Rust component | Fields |
|-------------|----------------|--------|
| *(always)* | `PlacementId(String)` | `{placement.id}_{index}` |
| *(always)* | `MeshRef(String)` | from placement `mesh` |
| *(always)* | `Transform` | from layout expansion |
| `person` | `Person` | marker (no fields) |
| `name` | `Name(String)` | `value` |
| `health` | `Health(u32)` | `value` |
| `texture` | `TextureRef(String)` | `value` |
| `trigger` | `Trigger` | `on`, `event` |

### How to add a component

1. Add a `#[derive(Component)]` struct in `crates/world_generation/src/components.rs`.
2. Add a `ComponentSpec` variant with `#[serde(tag = "type")]`.
3. Add a match arm in `insert_component` inside `spawn_world`.
4. Document the JSON shape in this table.

Mesh **handles** / materials stay out of `world_generation` for now: store `MeshRef` / `TextureRef` paths; a later `fox_the_guy` system resolves them via `AssetServer`.

## Layout kinds (DSL)

| `kind` | Purpose | Key fields | Density |
|--------|---------|------------|---------|
| `point` | One-off prop | `translation`, optional `rotation` (quat), `scale` | 1 instance |
| `line` | Brick rows, fences | `from`, `to`, `facing` | `count` (≥ 2) evenly along segment |
| `circle` | Ring / crescent walls | `center`, `radius`, `start_angle_deg`, `end_angle_deg`, `facing` | `count` evenly along arc (full ring = 0→360) |
| `spline` | Curvy roads | `points` (≥ 2), `facing` | `spacing` world units along Catmull-Rom curve through the points |

### Facing

How instance rotation is derived:

| Value | Behavior |
|-------|----------|
| `identity` | Keep prototype/default rotation |
| `along` | Forward axis (+Z) follows path tangent (lines, splines; circles = tangential) |
| `outward` | Face away from circle center (ring walls); for line/spline, face to the right of tangent in XZ |
| `fixed` | Use optional `rotation` quaternion on the layout (same for every instance) |

### Conventions

- **Coordinate system:** Y-up.
- **Circle angles:** degrees, counterclockwise from +X in the XZ plane. Missing `start_angle_deg` / `end_angle_deg` default to `0` / `360`.
- **Quaternions:** `[x, y, z, w]`. Missing `rotation` / `scale` on `point` → identity quat / `[1, 1, 1]`.
- **Instance ids:** `{placement.id}_{index}` (e.g. `outer_ring_wall_0`).
- **Components:** prototype `components` copy onto every expanded instance.
- **Density:** line/circle use `count`; spline uses `spacing`. Do not mix both on one kind in v1.
- **Spline:** Catmull-Rom through control points (points lie on the path). No Bezier handles in v1.

### Why this shape

- Serde-friendly: `#[serde(tag = "kind")]` enum for `Layout`, `#[serde(tag = "type")]` for `ComponentSpec`
- One array to author and load; no separate “objects vs generators” lists
- Circles with angle range cover full rings and Outshire/Hedgerow crescents without a second primitive
- Spline spacing suits fixed-length road tiles; line/circle counts suit segment budgets

## Rust API

```rust
use bevy::prelude::*;
use world_generation::{expand, spawn_world, WorldFile};

let file = WorldFile::from_json(json)?;
// Pure math:
let instances = expand(&file.placements[0]);
// ECS:
spawn_world(&mut commands, &file);
```

Decode, expand, and spawn live in [`crates/world_generation`](../crates/world_generation).

## Intended locations

| Concern | Path |
|---------|------|
| Design (this doc) | `docs/world-placement.md` |
| Level data | `assets/world/*.json` (Bevy load) or `story/world/` while authoring with the story bible |
| Types, expand, spawn | [`crates/world_generation`](../crates/world_generation) |
| Mesh/texture asset loading | `fox_the_guy` (follow-up) |
| Fly-camera preview tool | [`crates/world_viewer`](../crates/world_viewer) — `cargo run -p world_viewer -- path/to/world.json` |

Flow: `WorldFile::from_json` → `expand` → `spawn_world` → later resolve `MeshRef` / `TextureRef`.

## Out of scope (v1)

- Procedural geometry / mesh generation
- Inventing unknown components from JSON strings alone
- Bevy Reflect / dynamic scenes
- Per-instance overrides, jitter, random seeds
- Closed spline loops, Bezier handles, multi-rail roads
- Loading mesh assets / materials
- Bevy RON scenes (revisit only if editor workflow demands it)
