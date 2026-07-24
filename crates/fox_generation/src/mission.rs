use serde::{Deserialize, Serialize};

/// Mission kinds assigned by Cousins (`story/npc/cousins/*.yaml`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionType {
    Courier,
    Clock,
    Redistribution,
    Ceremony,
    OddJob,
    Powder,
    Infiltration,
    Ledger,
    Pamphlet,
    Medical,
    Extraction,
    Quartermaster,
}

impl MissionType {
    pub const ALL: [Self; 12] = [
        Self::Courier,
        Self::Clock,
        Self::Redistribution,
        Self::Ceremony,
        Self::OddJob,
        Self::Powder,
        Self::Infiltration,
        Self::Ledger,
        Self::Pamphlet,
        Self::Medical,
        Self::Extraction,
        Self::Quartermaster,
    ];
}
