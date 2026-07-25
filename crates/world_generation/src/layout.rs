use bevy::math::{Quat, Vec3};
use serde::{Deserialize, Serialize};

/// How instance rotation is derived from a path or fixed quat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Facing {
    Identity,
    Along,
    Outward,
    Fixed,
}

fn default_start_angle() -> f32 {
    0.0
}

fn default_end_angle() -> f32 {
    360.0
}

/// Where to stamp mesh instances for a placement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Layout {
    Point {
        translation: Vec3,
        #[serde(default)]
        rotation: Option<Quat>,
        #[serde(default)]
        scale: Option<Vec3>,
    },
    Line {
        from: Vec3,
        to: Vec3,
        count: u32,
        facing: Facing,
        /// Used when `facing` is [`Facing::Fixed`].
        #[serde(default)]
        rotation: Option<Quat>,
    },
    Circle {
        center: Vec3,
        radius: f32,
        count: u32,
        #[serde(default = "default_start_angle")]
        start_angle_deg: f32,
        #[serde(default = "default_end_angle")]
        end_angle_deg: f32,
        facing: Facing,
        /// Used when `facing` is [`Facing::Fixed`].
        #[serde(default)]
        rotation: Option<Quat>,
    },
    Spline {
        points: Vec<Vec3>,
        spacing: f32,
        facing: Facing,
        /// Used when `facing` is [`Facing::Fixed`].
        #[serde(default)]
        rotation: Option<Quat>,
    },
}
