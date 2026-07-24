use rand::Rng;
use serde::{Deserialize, Serialize};

/// Fox attribute values on a 1–10 scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    pub strength: u8,
    pub agility: u8,
    pub endurance: u8,
    pub movement: u8,
    pub dexterity: u8,
    pub perception: u8,
    pub cunning: u8,
    pub knowledge: u8,
    pub presence: u8,
    pub nerve: u8,
}

/// Per-stat deltas from a former trade (sparse in JSON; missing fields are zero).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatModifiers {
    #[serde(default)]
    pub strength: i8,
    #[serde(default)]
    pub agility: i8,
    #[serde(default)]
    pub endurance: i8,
    #[serde(default)]
    pub movement: i8,
    #[serde(default)]
    pub dexterity: i8,
    #[serde(default)]
    pub perception: i8,
    #[serde(default)]
    pub cunning: i8,
    #[serde(default)]
    pub knowledge: i8,
    #[serde(default)]
    pub presence: i8,
    #[serde(default)]
    pub nerve: i8,
}

impl Stats {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 10;

    pub fn apply_modifiers(self, modifiers: StatModifiers) -> Self {
        Self {
            strength: clamp_stat(self.strength as i16 + modifiers.strength as i16),
            agility: clamp_stat(self.agility as i16 + modifiers.agility as i16),
            endurance: clamp_stat(self.endurance as i16 + modifiers.endurance as i16),
            movement: clamp_stat(self.movement as i16 + modifiers.movement as i16),
            dexterity: clamp_stat(self.dexterity as i16 + modifiers.dexterity as i16),
            perception: clamp_stat(self.perception as i16 + modifiers.perception as i16),
            cunning: clamp_stat(self.cunning as i16 + modifiers.cunning as i16),
            knowledge: clamp_stat(self.knowledge as i16 + modifiers.knowledge as i16),
            presence: clamp_stat(self.presence as i16 + modifiers.presence as i16),
            nerve: clamp_stat(self.nerve as i16 + modifiers.nerve as i16),
        }
    }

    /// Add independent uniform noise in `[-1, 1]` per stat, then clamp.
    pub fn with_noise(self, rng: &mut impl Rng) -> Self {
        Self {
            strength: clamp_stat(self.strength as i16 + rng.random_range(-1..=1)),
            agility: clamp_stat(self.agility as i16 + rng.random_range(-1..=1)),
            endurance: clamp_stat(self.endurance as i16 + rng.random_range(-1..=1)),
            movement: clamp_stat(self.movement as i16 + rng.random_range(-1..=1)),
            dexterity: clamp_stat(self.dexterity as i16 + rng.random_range(-1..=1)),
            perception: clamp_stat(self.perception as i16 + rng.random_range(-1..=1)),
            cunning: clamp_stat(self.cunning as i16 + rng.random_range(-1..=1)),
            knowledge: clamp_stat(self.knowledge as i16 + rng.random_range(-1..=1)),
            presence: clamp_stat(self.presence as i16 + rng.random_range(-1..=1)),
            nerve: clamp_stat(self.nerve as i16 + rng.random_range(-1..=1)),
        }
    }
}

fn clamp_stat(value: i16) -> u8 {
    value.clamp(Stats::MIN as i16, Stats::MAX as i16) as u8
}
