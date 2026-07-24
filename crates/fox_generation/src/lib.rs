//! Load [`FoxGenerationCatalog`] from `story/fox-generation.json` and roll tonight's Guy.
//!
//! A rolled fox always includes the story-required fields: **name**, **trade**,
//! **reason**, scrubbed Bank **piece**, and one **personal_touch**, plus stats
//! and Cousin mission affinities.

mod catalog;
mod generate;
mod mission;
mod stats;

pub use catalog::{FlavorEntry, FoxGenerationCatalog, TradeProfile};
pub use generate::{GeneratedFox, GenerateError};
pub use mission::MissionType;
pub use stats::{StatModifiers, Stats};

/// Path to the catalog JSON relative to the workspace root.
pub const FOX_GENERATION_JSON: &str = "story/fox-generation.json";

/// Embedded catalog (compile-time copy of `FOX_GENERATION_JSON`).
pub static EMBEDDED_CATALOG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../story/fox-generation.json"
));

/// Parse the embedded catalog.
pub fn embedded_catalog() -> Result<FoxGenerationCatalog, serde_json::Error> {
    serde_json::from_str(EMBEDDED_CATALOG)
}
