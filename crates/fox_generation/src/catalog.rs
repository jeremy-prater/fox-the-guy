use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::mission::MissionType;
use crate::stats::{StatModifiers, Stats};

/// Trade-specific stat shifts and Cousin mission affinities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeProfile {
    pub modifiers: StatModifiers,
    pub mission_types: Vec<MissionType>,
}

/// A catalog flavor entry: display text plus optional sparse stat shifts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlavorEntry {
    pub description: String,
    #[serde(default)]
    pub modifiers: StatModifiers,
}

/// Contents of `story/fox-generation.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoxGenerationCatalog {
    pub mission_types: Vec<MissionType>,
    pub baseline: Stats,
    pub name: Vec<String>,
    pub trade: HashMap<String, TradeProfile>,
    pub piece: HashMap<String, FlavorEntry>,
    pub personal_touch: HashMap<String, FlavorEntry>,
    pub reason: HashMap<String, FlavorEntry>,
}

impl FoxGenerationCatalog {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn profile(&self, trade: &str) -> Option<&TradeProfile> {
        self.trade.get(trade)
    }
}
