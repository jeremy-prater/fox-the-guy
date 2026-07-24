use rand::seq::{IndexedRandom, IteratorRandom};
use rand::Rng;

use crate::catalog::FoxGenerationCatalog;
use crate::mission::MissionType;
use crate::stats::Stats;

/// One rolled fox for tonight's run.
///
/// Required fields match the story bible: a name that is not Guy, a former
/// trade, a small reason, scrubbed Bank kit (`piece`), and one personal touch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFox {
    pub name: String,
    pub trade: String,
    pub reason: String,
    pub piece: String,
    pub personal_touch: String,
    pub stats: Stats,
    pub mission_types: Vec<MissionType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenerateError {
    NoNames,
    NoTrades,
    NoReasons,
    NoPieces,
    NoPersonalTouches,
    MissingTradeProfile(String),
    MissingReason(String),
    MissingPiece(String),
    MissingPersonalTouch(String),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoNames => write!(f, "catalog has no names"),
            Self::NoTrades => write!(f, "catalog has no trades"),
            Self::NoReasons => write!(f, "catalog has no reasons"),
            Self::NoPieces => write!(f, "catalog has no pieces"),
            Self::NoPersonalTouches => write!(f, "catalog has no personal touches"),
            Self::MissingTradeProfile(trade) => {
                write!(f, "missing trade profile for {trade:?}")
            }
            Self::MissingReason(key) => write!(f, "missing reason for {key:?}"),
            Self::MissingPiece(key) => write!(f, "missing piece for {key:?}"),
            Self::MissingPersonalTouch(key) => {
                write!(f, "missing personal touch for {key:?}")
            }
        }
    }
}

impl std::error::Error for GenerateError {}

impl FoxGenerationCatalog {
    /// Roll a fox using the catalog's required tables and trade profiles.
    pub fn generate<R: Rng>(&self, rng: &mut R) -> Result<GeneratedFox, GenerateError> {
        let name = self
            .name
            .choose(rng)
            .ok_or(GenerateError::NoNames)?
            .clone();
        let trade = self
            .trade
            .keys()
            .choose(rng)
            .ok_or(GenerateError::NoTrades)?
            .clone();
        let reason = self
            .reason
            .keys()
            .choose(rng)
            .ok_or(GenerateError::NoReasons)?
            .clone();
        let piece = self
            .piece
            .keys()
            .choose(rng)
            .ok_or(GenerateError::NoPieces)?
            .clone();
        let personal_touch = self
            .personal_touch
            .keys()
            .choose(rng)
            .ok_or(GenerateError::NoPersonalTouches)?
            .clone();
        self.generate_for_trade(rng, name, &trade, &reason, &piece, &personal_touch)
    }

    /// Roll stats and mission affinities for a specific trade and kit.
    ///
    /// `reason`, `piece`, and `personal_touch` are catalog keys.
    pub fn generate_for_trade<R: Rng>(
        &self,
        rng: &mut R,
        name: String,
        trade: &str,
        reason: &str,
        piece: &str,
        personal_touch: &str,
    ) -> Result<GeneratedFox, GenerateError> {
        let profile = self
            .profile(trade)
            .ok_or_else(|| GenerateError::MissingTradeProfile(trade.to_string()))?;
        let reason_entry = self
            .reason
            .get(reason)
            .ok_or_else(|| GenerateError::MissingReason(reason.to_string()))?;
        let piece_entry = self
            .piece
            .get(piece)
            .ok_or_else(|| GenerateError::MissingPiece(piece.to_string()))?;
        let touch_entry = self
            .personal_touch
            .get(personal_touch)
            .ok_or_else(|| GenerateError::MissingPersonalTouch(personal_touch.to_string()))?;

        let stats = self
            .baseline
            .apply_modifiers(profile.modifiers)
            .apply_modifiers(reason_entry.modifiers)
            .apply_modifiers(piece_entry.modifiers)
            .apply_modifiers(touch_entry.modifiers)
            .with_noise(rng);

        Ok(GeneratedFox {
            name,
            trade: trade.to_string(),
            reason: reason_entry.description.clone(),
            piece: piece_entry.description.clone(),
            personal_touch: touch_entry.description.clone(),
            stats,
            mission_types: profile.mission_types.clone(),
        })
    }

    /// Whether this fox's trade aligns with a Cousin's `mission_types` list.
    pub fn mission_overlap(fox: &GeneratedFox, cousin_missions: &[MissionType]) -> bool {
        fox.mission_types
            .iter()
            .any(|m| cousin_missions.contains(m))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedded_catalog;

    #[test]
    fn embedded_catalog_parses() {
        let catalog = embedded_catalog().expect("embedded JSON should parse");
        assert_eq!(catalog.mission_types.len(), 12);
        assert_eq!(catalog.trade.len(), 30);
        assert!(!catalog.name.is_empty());
        assert_eq!(catalog.piece.len(), 20);
        assert_eq!(catalog.personal_touch.len(), 20);
        assert_eq!(catalog.reason.len(), 32);
        assert!(catalog.trade.contains_key("mask-rewirer"));
        assert!(catalog.piece.contains_key("pocket_jammer"));
        assert!(catalog.personal_touch.contains_key("discontinued_prayer_bead"));
        assert!(catalog.reason.contains_key("mask_powered_on"));
    }

    #[test]
    fn generate_fills_required_fields() {
        let catalog = embedded_catalog().unwrap();
        let mut rng = rand::rng();
        for _ in 0..200 {
            let fox = catalog.generate(&mut rng).unwrap();
            assert!(!fox.name.is_empty());
            assert!(!fox.trade.is_empty());
            assert!(!fox.reason.is_empty());
            assert!(!fox.piece.is_empty());
            assert!(!fox.personal_touch.is_empty());
            let s = fox.stats;
            for value in [
                s.strength,
                s.agility,
                s.endurance,
                s.movement,
                s.dexterity,
                s.perception,
                s.cunning,
                s.knowledge,
                s.presence,
                s.nerve,
            ] {
                assert!((Stats::MIN..=Stats::MAX).contains(&value));
            }
            assert_eq!(fox.mission_types.len(), 2);
        }
    }

    #[test]
    fn fuse_cutter_matches_bramble_hook_missions() {
        let catalog = embedded_catalog().unwrap();
        let fox = catalog
            .generate_for_trade(
                &mut rand::rng(),
                "Test Fox".into(),
                "fuse-cutter",
                "mask_powered_on",
                "pocket_jammer",
                "discontinued_prayer_bead",
            )
            .unwrap();
        let bramble = [MissionType::OddJob, MissionType::Powder];
        assert!(FoxGenerationCatalog::mission_overlap(&fox, &bramble));
        assert!(fox.mission_types.contains(&MissionType::Powder));
        assert_eq!(fox.name, "Test Fox");
        assert_eq!(fox.piece, catalog.piece["pocket_jammer"].description);
    }

    #[test]
    fn jammer_scrubber_matches_hector_missions() {
        let catalog = embedded_catalog().unwrap();
        let fox = catalog
            .generate_for_trade(
                &mut rand::rng(),
                "Gutter-Spark".into(),
                "jammer-pack scrubber",
                "leakage_money_for_hector",
                "cut_down_riot_lance",
                "hectors_grease_pencil_x",
            )
            .unwrap();
        let hector = [MissionType::Redistribution, MissionType::Quartermaster];
        assert!(FoxGenerationCatalog::mission_overlap(&fox, &hector));
        assert!(fox.mission_types.contains(&MissionType::Quartermaster));
    }
}
