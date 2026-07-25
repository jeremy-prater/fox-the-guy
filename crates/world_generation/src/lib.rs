//! Decode world placement JSON and spawn Bevy entities.
//!
//! See `docs/world-placement.md` for the schema and layout DSL.

mod components;
mod expand;
mod layout;
mod placement;
mod world;

pub use components::{
    ComponentSpec, Health, MeshRef, Name, Person, PlacementId, TextureRef, Trigger,
};
pub use expand::expand;
pub use layout::{Facing, Layout};
pub use placement::Placement;
pub use world::{spawn_world, WorldFile};
