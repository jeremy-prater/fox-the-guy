use bevy::ecs::system::Commands;
use log::info;
use serde::{Deserialize, Serialize};

use crate::Placement;
use crate::components::{
    ComponentSpec, Health, MeshRef, Name, Person, PlacementId, TextureRef, Trigger,
};
use crate::expand::expand;

/// Contents of a world placement JSON file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldFile {
    pub version: u32,
    pub placements: Vec<Placement>,
}

impl WorldFile {
    /// Parse a world placement document from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Expand placements and spawn Bevy entities with transforms + components.
pub fn spawn_world(commands: &mut Commands, world: &WorldFile) {
    for placement in &world.placements {
        for (instance_id, transform) in expand(placement) {
            info!("Spawning : {} : {}", placement.id, instance_id);
            info!("Spawning : {} : {:?}", placement.id, transform);
            let mut entity = commands.spawn((
                PlacementId(instance_id),
                MeshRef(placement.mesh.clone()),
                transform,
            ));
            for spec in &placement.components {
                insert_component(&mut entity, spec);
            }
        }
    }
}

fn insert_component(entity: &mut bevy::ecs::system::EntityCommands, spec: &ComponentSpec) {
    match spec {
        ComponentSpec::Person => {
            entity.insert(Person);
        }
        ComponentSpec::Name { value } => {
            entity.insert(Name(value.clone()));
        }
        ComponentSpec::Health { value } => {
            entity.insert(Health(*value));
        }
        ComponentSpec::Texture { value } => {
            entity.insert(TextureRef(value.clone()));
        }
        ComponentSpec::Trigger { on, event } => {
            entity.insert(Trigger {
                on: on.clone(),
                event: event.clone(),
            });
        }
    }
}
