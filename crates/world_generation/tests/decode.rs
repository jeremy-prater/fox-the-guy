use bevy::ecs::system::SystemState;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Commands, Transform, With, World};
use world_generation::{
    expand, spawn_world, ComponentSpec, Facing, Health, Layout, MeshRef, Name, Person, Placement,
    PlacementId, TextureRef, Trigger, WorldFile,
};

#[test]
fn decodes_sample_world_fixture() {
    let json = include_str!("fixtures/sample_world.json");
    let world = WorldFile::from_json(json).expect("sample world should parse");

    assert_eq!(world.version, 1);
    assert_eq!(world.placements.len(), 5);

    let barrel = &world.placements[0];
    assert_eq!(barrel.id, "works_barrel_01");
    assert_eq!(barrel.mesh, "meshes/barrel");
    assert_eq!(
        barrel.components,
        vec![
            ComponentSpec::Health { value: 40 },
            ComponentSpec::Texture {
                value: "textures/barrel_tagged".into(),
            },
            ComponentSpec::Trigger {
                on: "interact".into(),
                event: "jam_or_leave".into(),
            },
        ]
    );
    assert!(matches!(
        barrel.layout,
        Layout::Point {
            translation,
            ..
        } if translation == Vec3::new(12.0, 0.0, -3.5)
    ));

    let bricks = &world.placements[1];
    assert!(matches!(
        bricks.layout,
        Layout::Line {
            count: 30,
            facing: Facing::Along,
            ..
        }
    ));

    let wall = &world.placements[2];
    assert!(matches!(
        wall.layout,
        Layout::Circle {
            radius: 10000.0,
            count: 720,
            start_angle_deg: -60.0,
            end_angle_deg: 60.0,
            facing: Facing::Outward,
            ..
        }
    ));

    let road = &world.placements[3];
    assert!(matches!(
        road.layout,
        Layout::Spline {
            spacing: 2.0,
            facing: Facing::Along,
            ..
        }
    ));
    if let Layout::Spline { points, .. } = &road.layout {
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], Vec3::ZERO);
    }

    let greeter = &world.placements[4];
    assert_eq!(
        greeter.components,
        vec![
            ComponentSpec::Person,
            ComponentSpec::Name {
                value: "John Doe".into(),
            },
        ]
    );
}

#[test]
fn circle_defaults_full_ring_angles() {
    let json = r#"{
      "version": 1,
      "placements": [{
        "id": "inner_ring",
        "mesh": "meshes/wall_segment",
        "layout": {
          "kind": "circle",
          "center": [0.0, 0.0, 0.0],
          "radius": 2500.0,
          "count": 360,
          "facing": "outward"
        }
      }]
    }"#;

    let world = WorldFile::from_json(json).expect("circle defaults should parse");
    let Layout::Circle {
        start_angle_deg,
        end_angle_deg,
        center,
        ..
    } = world.placements[0].layout
    else {
        panic!("expected circle layout");
    };
    assert_eq!(center, Vec3::ZERO);
    assert_eq!(start_angle_deg, 0.0);
    assert_eq!(end_angle_deg, 360.0);
}

#[test]
fn point_omits_optional_fields() {
    let json = r#"{
      "version": 1,
      "placements": [{
        "id": "crate_01",
        "mesh": "meshes/crate",
        "layout": {
          "kind": "point",
          "translation": [1.0, 0.0, 2.0]
        }
      }]
    }"#;

    let world = WorldFile::from_json(json).expect("minimal point should parse");
    let Layout::Point {
        translation,
        rotation,
        scale,
    } = world.placements[0].layout
    else {
        panic!("expected point layout");
    };
    assert_eq!(translation, Vec3::new(1.0, 0.0, 2.0));
    assert_eq!(rotation, None);
    assert_eq!(scale, None);
    assert!(world.placements[0].components.is_empty());
}

#[test]
fn expand_line_matches_count() {
    let placement = Placement {
        id: "row".into(),
        mesh: "meshes/brick".into(),
        layout: Layout::Line {
            from: Vec3::new(0.0, 0.0, 0.0),
            to: Vec3::new(9.0, 0.0, 0.0),
            count: 4,
            facing: Facing::Along,
            rotation: None,
        },
        components: vec![],
    };
    let instances = expand(&placement);
    assert_eq!(instances.len(), 4);
    assert_eq!(instances[0].0, "row_0");
    assert_eq!(instances[3].0, "row_3");
    assert_eq!(instances[0].1.translation.x, 0.0);
    assert_eq!(instances[3].1.translation.x, 9.0);
}

#[test]
fn spawn_world_inserts_components() {
    let world_file = WorldFile {
        version: 1,
        placements: vec![Placement {
            id: "npc".into(),
            mesh: "meshes/fox".into(),
            layout: Layout::Point {
                translation: Vec3::new(1.0, 0.0, 2.0),
                rotation: Some(Quat::IDENTITY),
                scale: Some(Vec3::ONE),
            },
            components: vec![
                ComponentSpec::Person,
                ComponentSpec::Name {
                    value: "Elaina Proctor".into(),
                },
                ComponentSpec::Health { value: 10 },
                ComponentSpec::Texture {
                    value: "textures/fox".into(),
                },
                ComponentSpec::Trigger {
                    on: "talk".into(),
                    event: "greet".into(),
                },
            ],
        }],
    };

    let mut world = World::new();
    let mut system_state: SystemState<Commands> = SystemState::new(&mut world);
    {
        let mut commands = system_state
            .get_mut(&mut world)
            .expect("Commands system param");
        spawn_world(&mut commands, &world_file);
    }
    system_state.apply(&mut world);

    let mut query = world.query_filtered::<(
        &PlacementId,
        &MeshRef,
        &Transform,
        &Name,
        &Health,
        &TextureRef,
        &Trigger,
    ), With<Person>>();
    let results: Vec<_> = query.iter(&world).collect();
    assert_eq!(results.len(), 1);
    let (id, mesh, transform, name, health, texture, trigger) = results[0];
    assert_eq!(id.0, "npc_0");
    assert_eq!(mesh.0, "meshes/fox");
    assert_eq!(transform.translation, Vec3::new(1.0, 0.0, 2.0));
    assert_eq!(name.0, "Elaina Proctor");
    assert_eq!(health.0, 10);
    assert_eq!(texture.0, "textures/fox");
    assert_eq!(trigger.on, "talk");
    assert_eq!(trigger.event, "greet");
}
