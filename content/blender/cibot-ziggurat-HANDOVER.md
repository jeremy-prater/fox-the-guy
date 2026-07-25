# CIBoT ziggurat — handover

How to recreate or revise the Bank tower mesh for a new agent session.

## Goal

Author the **Bank / CIBoT tower** (story: colossal mirrored ziggurat with clocks, vault, twisting spires) as a Blender blockout, export one GLB, and place it **on top of** `assets/meshes/cibot-core.glb` in the city template world.

This is an untextured draft. Lights / security cameras on the Core roof deck are **not** in this mesh — they ship later as a separate GLB.

## Canon references (read these first)

| Doc | What to take |
|-----|----------------|
| [`story/story.md`](../../story/story.md) § Pillars of the Bank | Bank building = mirrored ziggurat, clocks on faces, leaded-crystal roof; Vault is opaque / contested |
| [`story/map_sizes.md`](../../story/map_sizes.md) | **CIBoT Core** outer radius = **0.8 km (800 m)** from the Bank axis |
| Concept refs used in-session | Tall tapered faceted core + **3** ribbon helix spires with radial struts; vault / clock hub where spires meet; spires clear the shaft until they join the vault |

Visual lineage that landed:

1. Stepped ziggurat alone → wrong
2. Many wrapping ribbons → wrong
3. 3–4 ribbons + elevated vault → closer
4. **Tapered octagonal core + 3 ribbon spires + struts + clock vault crown** → accepted silhouette
5. Expand vault diameter so clocks sit **outside** the ribbons → accepted
6. Core roof deck for future kit; remove light/camera placeholders → accepted

## Files

| Path | Role |
|------|------|
| `content/blender/cibot-ziggurat.blend` | Source Blender scene |
| `assets/meshes/cibot-ziggurat.glb` | Exported runtime mesh |
| `assets/meshes/cibot-core.glb` | Core disk / pad (already authored; **800 m** radius, top at **Y = 2500**) |
| `worlds/tests/world-template.json` | Places core + ziggurat |
| `tools/export_scene.sh` | Headless whole-scene GLB export |
| `tools/export_scene_to_glb.py` | Blender Python exporter |

Do **not** put this placement in `worlds/tests/hello.json` (demo boxes only).

## Scale (authoritative)

From `map_sizes.md`:

- **CIBoT Core disk radius = 0.8 km = 800 m**
- Blender / Bevy: **1 unit = 1 meter**
- `cibot-core.glb` AABB (Y-up): roughly `[-800, 0, -800]` → `[800, 2500, 800]`
- Ziggurat was uniformly scaled so its **max planar radius ≈ 800 m** (matches core disk), height ≈ **2.18 km**
- World placement: ziggurat translation **`[0.0, 2500.0, 0.0]`** so mesh bottom (Y ≈ 0) sits on core top (Y = 2500)

If you rebuild the tower at a different authored size, re-measure GLB AABBs and update the Y offset in `world-template.json`.

## Intended mesh composition

Parent under empty `CIBoT_Root` (or equivalent). Untextured grey blockout.

### Plaza / ground

- Single circular `Plaza_Disk` (and optional `Plaza_Bridge`)
- Core and Spire bottoms must **meet or slightly penetrate** the plaza (no hover gaps)

### `Core`

- Tall **tapered faceted** shaft (octagon worked well)
- Wider at base, thinner under the vault
- **Roof deck** (`CoreTop_*`): deck plate, rail posts/rail, hatch, cabinet, conduit, antenna stub
- **Do not** include floodlights or security cameras on the deck

### Spires (exactly 3)

- Flat **ribbon** cross-section (wide × thin), not round tubes
- Helix around the Core with **air gap** (no Core intersection on the rise)
- Radial / diagonal **struts** from Core to ribbons
- Continuous path: orbit Core → pass vault → braid into needle tip
- Plant ribbon feet into the plaza

### Vault (clock crown)

- Drum / hub with framing rings where the three ribbons meet
- **Clock faces** on the hub (bezels, hands, ticks)
- Hub diameter must be **large enough that clocks sit outside the ribbons** (ribbons overlap / pass through the hub volume so faces read clearly)
- Upper ribbons continue into a short skeletal / needle tip above the hub

## Reproduce from scratch (new agent)

### A. Blender blockout

1. Open or create `content/blender/cibot-ziggurat.blend`.
2. Prefer the **Blender MCP** (`execute_blender_code`, viewport screenshots) if available; otherwise script in the Scripting workspace.
3. Build plaza → Core → 3 ribbon spires + struts → vault hub + clocks → CoreTop deck (no lights/cameras).
4. Iterate against story + map size + silhouette notes above. Screenshot often.
5. Plant Core/Spire bottoms into the plaza; watch for false “hover” from a second plaza shelf — keep **one** ground disk.
6. Scale the whole assembly so **max XY radius from origin = 800 m** (CIBoT Core). Bake scale (`Apply Scale`) before export.
7. Save the `.blend`.

### B. Export GLB

```bash
# From repo root (macOS finds /Applications/Blender.app if `blender` is not on PATH)
tools/export_scene.sh content/blender/cibot-ziggurat.blend
```

Writes `assets/meshes/cibot-ziggurat.glb` by default (stem from the `.blend` name).

Overrides:

- `FOX_MESHES_DIR` — output directory
- `FOX_SCENE_GLB` — full output path
- `BLENDER` — Blender binary

Related: `tools/export_meshes.sh` exports **each mesh object** as its own GLB (city template workflow). Use `export_scene.sh` for this tower.

### C. World placement

In `worlds/tests/world-template.json`:

```json
{
  "id": "cibot_core",
  "mesh": "meshes/cibot-core.glb",
  "layout": {
    "kind": "point",
    "translation": [0.0, 0.0, 0.0]
  }
},
{
  "id": "cibot_ziggurat",
  "mesh": "meshes/cibot-ziggurat.glb",
  "layout": {
    "kind": "point",
    "translation": [0.0, 2500.0, 0.0]
  }
}
```

Verify with world viewer:

```bash
cargo run -p world_viewer -- worlds/tests/world-template.json
```

## Design decisions to preserve

- **3 spires**, not 4–8 wrapping vines
- Ribbons are **flat straps**, with struts to the Core
- Vault clocks must be **visible** (hub wider than ribbon orbit)
- Core roof is a **kit pad** only — no baked lights/cameras
- Scale and placement are tied to **0.8 km CIBoT Core** and `cibot-core.glb` height **2500**

## Out of scope / follow-ups

- Materials / mirrored chrome / HUD amber
- Separate GLB for lights and security cameras on `CoreTop_Deck`
- Splitting vault vs Core vs spires into multiple assets
- Aligning Tower of Coins / Library / Powderworks as sibling core props

## Quick checklist for a rewrite

- [ ] Story silhouette: tapered Core + 3 ribbons + clock vault
- [ ] No Core/spire hover over plaza
- [ ] Clocks outside ribbons
- [ ] No light/camera meshes on Core top
- [ ] Planar radius ≈ 800 m; scale applied
- [ ] Exported via `tools/export_scene.sh`
- [ ] `world-template.json`: ziggurat at `[0, 2500, 0]` on `cibot_core`
