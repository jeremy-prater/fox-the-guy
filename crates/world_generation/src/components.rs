use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

/// Marker component (tutorial-style / NPC tag).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Person;

/// Display name for an entity.
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct Name(pub String);

/// Hit points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Health(pub u32);

/// Path to a texture asset (resolved later via `AssetServer`).
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct TextureRef(pub String);

/// Stable id for a spawned placement instance (`{placement.id}_{index}`).
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct PlacementId(pub String);

/// Path to a mesh asset (resolved later via `AssetServer`).
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct MeshRef(pub String);

/// Interaction / event trigger on an entity.
#[derive(Debug, Clone, PartialEq, Eq, Component, Serialize, Deserialize)]
pub struct Trigger {
    pub on: String,
    pub event: String,
}

/// Authored component list entry in world JSON (`#[serde(tag = "type")]`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComponentSpec {
    Person,
    Name { value: String },
    Health { value: u32 },
    Texture { value: String },
    Trigger { on: String, event: String },
}
