//! Pan/orbit camera viewer for world placement JSON files.
//!
//! Usage: `cargo run -p world_viewer -- path/to/world.json`
//!
//! `mesh` values are Bevy asset paths under `assets/`, e.g. `"meshes/box-circle.glb"`.
//! They are loaded as glTF assets; scene 0 is spawned via [`WorldAssetRoot`].
//! Scene bounds for the camera come from mesh geometry AABBs (transformed by
//! each placement), not placement translations alone.
//!
//! Controls ([`bevy_panorbit_camera`]): left-drag orbit, right-drag pan, scroll
//! zoom. `[` / `]` snap to each AABB corner (quadrant order), looking at the
//! world origin.

use std::collections::HashMap;
use std::path::PathBuf;

use bevy::asset::RecursiveDependencyLoadState;
use bevy::camera::primitives::{Aabb, MeshAabb};
use bevy::gltf::{Gltf, GltfMesh};
use bevy::math::bounding::{Aabb3d, BoundingVolume};
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use world_generation::{spawn_world, MeshRef, PlacementId, WorldFile};

fn main() {
    let path = parse_world_path();
    let assets_path = workspace_assets_path();

    App::new()
        .insert_resource(WorldPath(path))
        .init_resource::<MeshHandles>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "World Viewer".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: assets_path,
                    ..default()
                }),
        )
        .add_plugins(PanOrbitCameraPlugin)
        .init_state::<AppState>()
        .add_systems(
            Startup,
            (setup_environment, load_and_spawn_world, begin_mesh_loads).chain(),
        )
        .add_systems(
            Update,
            check_meshes_loaded.run_if(in_state(AppState::Loading)),
        )
        .add_systems(
            OnEnter(AppState::Ready),
            (attach_scenes, compute_scene_bounds, spawn_camera).chain(),
        )
        .add_systems(
            Update,
            cycle_camera_corners.run_if(in_state(AppState::Ready)),
        )
        .run();
}

/// Workspace-root `assets/` (not `crates/world_viewer/assets`).
fn workspace_assets_path() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .canonicalize()
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
        })
        .to_string_lossy()
        .into_owned()
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    Loading,
    Ready,
}

#[derive(Resource, Debug, Clone)]
struct WorldPath(PathBuf);

/// Unique placement mesh paths → loaded glTF assets (scenes + mesh AABBs).
#[derive(Resource, Debug, Default)]
struct MeshHandles {
    by_path: HashMap<String, Handle<Gltf>>,
}

/// Axis-aligned bounds of placement mesh geometry (not just translations), plus
/// the active corner index used by the corner-cycle camera controls.
#[derive(Resource, Debug, Clone)]
struct SceneBounds {
    aabb: Aabb3d,
    /// 0 = Q1 (+X,+Z), 1 = Q2 (−X,+Z), 2 = Q3 (−X,−Z), 3 = Q4 (+X,−Z).
    corner_index: usize,
}

impl SceneBounds {
    fn aabb_corner(&self) -> Vec3 {
        let min = Vec3::from(self.aabb.min);
        let max = Vec3::from(self.aabb.max);
        match self.corner_index % 4 {
            0 => Vec3::new(max.x, max.y, max.z),
            1 => Vec3::new(min.x, max.y, max.z),
            2 => Vec3::new(min.x, max.y, min.z),
            _ => Vec3::new(max.x, max.y, min.z),
        }
    }

    /// Camera sit point: along the origin→corner ray, past the AABB by 20% of
    /// its longest edge, then the same margin above.
    fn camera_position(&self) -> Vec3 {
        let corner = self.aabb_corner();
        let len = corner.length();
        let dir = if len > 1e-4 {
            corner / len
        } else {
            Vec3::new(1.0, 0.5, 1.0).normalize()
        };
        let aabb_size = (self.aabb.max - self.aabb.min).max_element();
        let margin = aabb_size * CAMERA_MARGIN_FRACTION;
        dir * (len.max(MIN_AABB_HALF_EXTENT) + margin) + Vec3::Y * margin
    }
}

/// Minimum half-extent so a degenerate (single-point) AABB still yields distinct corners.
const MIN_AABB_HALF_EXTENT: f32 = 1.0;
/// Camera offset past the AABB corner / above, as a fraction of the AABB's longest edge.
const CAMERA_MARGIN_FRACTION: f32 = 0.20;

fn parse_world_path() -> PathBuf {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("Usage: world_viewer <world.json>");
        std::process::exit(2);
    };
    if args.next().is_some() {
        eprintln!("Usage: world_viewer <world.json>");
        eprintln!("Expected a single world JSON path argument.");
        std::process::exit(2);
    }
    PathBuf::from(path)
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(20.0, 40.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200.0, 200.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.2, 0.22),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn load_and_spawn_world(mut commands: Commands, world_path: Res<WorldPath>) {
    let path = &world_path.0;
    let json = std::fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Failed to read {}: {err}", path.display());
        std::process::exit(1);
    });
    let world = WorldFile::from_json(&json).unwrap_or_else(|err| {
        eprintln!("Failed to parse {}: {err}", path.display());
        std::process::exit(1);
    });

    info!(
        "Loaded {} placements from {}",
        world.placements.len(),
        path.display()
    );
    spawn_world(&mut commands, &world);
}

fn begin_mesh_loads(
    asset_server: Res<AssetServer>,
    mesh_refs: Query<&MeshRef>,
    mut mesh_handles: ResMut<MeshHandles>,
) {
    for mesh_ref in &mesh_refs {
        mesh_handles
            .by_path
            .entry(mesh_ref.0.clone())
            .or_insert_with(|| {
                info!("Loading mesh asset {}", mesh_ref.0);
                asset_server.load(mesh_ref.0.clone())
            });
    }

    if mesh_handles.by_path.is_empty() {
        info!("No mesh refs to load");
    } else {
        info!(
            "Waiting for {} unique mesh asset(s) before rendering",
            mesh_handles.by_path.len()
        );
    }
}

fn check_meshes_loaded(
    asset_server: Res<AssetServer>,
    mesh_handles: Res<MeshHandles>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if mesh_handles.by_path.is_empty() {
        next_state.set(AppState::Ready);
        return;
    }

    let mut loaded = 0_usize;
    for (path, handle) in &mesh_handles.by_path {
        match asset_server.recursive_dependency_load_state(handle.id()) {
            RecursiveDependencyLoadState::Loaded => loaded += 1,
            RecursiveDependencyLoadState::Failed(err) => {
                eprintln!("Failed to load mesh '{path}': {err}");
                std::process::exit(1);
            }
            RecursiveDependencyLoadState::NotLoaded
            | RecursiveDependencyLoadState::Loading => return,
        }
    }

    if loaded == mesh_handles.by_path.len() {
        info!("All mesh assets loaded");
        next_state.set(AppState::Ready);
    }
}

fn attach_scenes(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    gltfs: Res<Assets<Gltf>>,
    query: Query<(Entity, &MeshRef), Without<WorldAssetRoot>>,
) {
    let mut count = 0_u32;
    for (entity, mesh_ref) in &query {
        let Some(gltf_handle) = mesh_handles.by_path.get(&mesh_ref.0) else {
            warn!("No loaded glTF for mesh ref '{}'", mesh_ref.0);
            continue;
        };
        let Some(gltf) = gltfs.get(gltf_handle) else {
            warn!("glTF asset missing for '{}'", mesh_ref.0);
            continue;
        };
        let Some(scene) = gltf
            .default_scene
            .clone()
            .or_else(|| gltf.scenes.first().cloned())
        else {
            warn!("glTF '{}' has no scenes", mesh_ref.0);
            continue;
        };
        commands.entity(entity).insert(WorldAssetRoot(scene));
        count += 1;
    }
    info!("Attached WorldAssetRoot to {count} entities");
}

fn compute_scene_bounds(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    placements: Query<(&MeshRef, &Transform), With<PlacementId>>,
) {
    let mut local_aabb_by_path: HashMap<&str, Aabb3d> = HashMap::new();
    for (path, handle) in &mesh_handles.by_path {
        let Some(gltf) = gltfs.get(handle) else {
            continue;
        };
        if let Some(aabb) = gltf_model_aabb(gltf, &gltf_meshes, &meshes) {
            local_aabb_by_path.insert(path.as_str(), aabb);
        } else {
            warn!("No mesh AABB for '{path}'");
        }
    }

    let mut world_min: Option<Vec3A> = None;
    let mut world_max: Option<Vec3A> = None;
    for (mesh_ref, transform) in &placements {
        let Some(local) = local_aabb_by_path.get(mesh_ref.0.as_str()) else {
            // Fall back to the placement point so empty meshes still contribute.
            let p = Vec3A::from(transform.translation);
            world_min = Some(world_min.map_or(p, |m| m.min(p)));
            world_max = Some(world_max.map_or(p, |m| m.max(p)));
            continue;
        };
        let world = transform_aabb3d(*local, transform);
        world_min = Some(world_min.map_or(world.min, |m| m.min(world.min)));
        world_max = Some(world_max.map_or(world.max, |m| m.max(world.max)));
    }

    let aabb = match (world_min, world_max) {
        (Some(min), Some(max)) => {
            let aabb = Aabb3d::from_min_max(min, max);
            Aabb3d::new(
                aabb.center(),
                aabb.half_size().max(Vec3A::splat(MIN_AABB_HALF_EXTENT)),
            )
        }
        _ => Aabb3d::new(Vec3A::ZERO, Vec3A::splat(MIN_AABB_HALF_EXTENT * 10.0)),
    };

    info!(
        "Scene AABB min {:?} max {:?} size {:?}",
        Vec3::from(aabb.min),
        Vec3::from(aabb.max),
        Vec3::from(aabb.max - aabb.min)
    );
    commands.insert_resource(SceneBounds {
        aabb,
        corner_index: 0,
    });
}

/// Union of all primitive mesh AABBs in glTF model space.
fn gltf_model_aabb(
    gltf: &Gltf,
    gltf_meshes: &Assets<GltfMesh>,
    meshes: &Assets<Mesh>,
) -> Option<Aabb3d> {
    let mut min: Option<Vec3A> = None;
    let mut max: Option<Vec3A> = None;
    for mesh_handle in &gltf.meshes {
        let Some(gltf_mesh) = gltf_meshes.get(mesh_handle) else {
            continue;
        };
        for primitive in &gltf_mesh.primitives {
            let Some(mesh) = meshes.get(&primitive.mesh) else {
                continue;
            };
            let Some(Aabb {
                center,
                half_extents,
            }) = mesh.compute_aabb()
            else {
                continue;
            };
            let a_min = center - half_extents;
            let a_max = center + half_extents;
            min = Some(min.map_or(a_min, |m| m.min(a_min)));
            max = Some(max.map_or(a_max, |m| m.max(a_max)));
        }
    }
    Some(Aabb3d::from_min_max(min?, max?))
}

fn transform_aabb3d(aabb: Aabb3d, transform: &Transform) -> Aabb3d {
    let min = Vec3::from(aabb.min);
    let max = Vec3::from(aabb.max);
    let corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(min.x, max.y, max.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(max.x, max.y, max.z),
    ];
    let mut iter = corners.into_iter().map(|c| Vec3A::from(transform.transform_point(c)));
    let first = iter.next().expect("aabb has 8 corners");
    let (min, max) = iter.fold((first, first), |(min, max), p| (min.min(p), max.max(p)));
    Aabb3d::from_min_max(min, max)
}

fn spawn_camera(mut commands: Commands, bounds: Res<SceneBounds>) {
    let focus = Vec3::ZERO;
    let position = bounds.camera_position();
    let size = (bounds.aabb.max - bounds.aabb.min).max_element();
    let (yaw, pitch, radius) = orbit_from_translation(position, focus);

    info!(
        "Camera at AABB corner {} {:?} radius {radius:.1} (scene size {size:.1})",
        bounds.corner_index, position
    );

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            // City footprint is ~27 km across; keep far plane past corner views.
            far: (size * 4.0).max(100_000.0),
            near: (size * 1e-5).clamp(0.1, 10.0),
            ..default()
        }),
        Transform::from_translation(position).looking_at(focus, Vec3::Y),
        PanOrbitCamera {
            focus,
            radius: Some(radius),
            yaw: Some(yaw),
            pitch: Some(pitch),
            target_focus: focus,
            target_yaw: yaw,
            target_pitch: pitch,
            target_radius: radius,
            zoom_upper_limit: Some((size * 10.0).max(radius * 4.0)),
            zoom_lower_limit: (size * 0.001).max(1.0),
            ..default()
        },
    ));
}

fn cycle_camera_corners(
    keys: Res<ButtonInput<KeyCode>>,
    mut bounds: ResMut<SceneBounds>,
    mut camera: Query<&mut PanOrbitCamera>,
) {
    let step = if keys.just_pressed(KeyCode::BracketRight) {
        1_isize
    } else if keys.just_pressed(KeyCode::BracketLeft) {
        -1
    } else {
        return;
    };

    bounds.corner_index = (bounds.corner_index as isize + step).rem_euclid(4) as usize;
    let Ok(mut pan_orbit) = camera.single_mut() else {
        return;
    };
    apply_corner_view(&mut pan_orbit, &bounds);
    info!(
        "Camera corner {} radius {:?}",
        bounds.corner_index, pan_orbit.radius
    );
}

fn apply_corner_view(pan_orbit: &mut PanOrbitCamera, bounds: &SceneBounds) {
    let focus = Vec3::ZERO;
    let position = bounds.camera_position();
    let (yaw, pitch, radius) = orbit_from_translation(position, focus);
    pan_orbit.focus = focus;
    pan_orbit.target_focus = focus;
    pan_orbit.yaw = Some(yaw);
    pan_orbit.pitch = Some(pitch);
    pan_orbit.radius = Some(radius);
    pan_orbit.target_yaw = yaw;
    pan_orbit.target_pitch = pitch;
    pan_orbit.target_radius = radius;
    pan_orbit.force_update = true;
}

/// Match `bevy_panorbit_camera::util::calculate_from_translation_and_focus` (Y-up).
fn orbit_from_translation(translation: Vec3, focus: Vec3) -> (f32, f32, f32) {
    let offset = translation - focus;
    let mut radius = offset.length();
    if radius == 0.0 {
        radius = 0.05;
    }
    let yaw = offset.x.atan2(offset.z);
    let pitch = (offset.y / radius).asin();
    (yaw, pitch, radius)
}
