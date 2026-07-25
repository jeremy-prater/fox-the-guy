use bevy::math::{Quat, Vec3};
use bevy::prelude::Transform;

use crate::{Facing, Layout, Placement};

/// Expand a placement layout into instance ids and transforms.
pub fn expand(placement: &Placement) -> Vec<(String, Transform)> {
    expand_layout(&placement.id, &placement.layout)
}

fn instance_id(placement_id: &str, index: usize) -> String {
    format!("{placement_id}_{index}")
}

fn expand_layout(placement_id: &str, layout: &Layout) -> Vec<(String, Transform)> {
    match layout {
        Layout::Point {
            translation,
            rotation,
            scale,
        } => {
            let transform = Transform {
                translation: *translation,
                rotation: rotation.unwrap_or(Quat::IDENTITY),
                scale: scale.unwrap_or(Vec3::ONE),
            };
            vec![(instance_id(placement_id, 0), transform)]
        }
        Layout::Line {
            from,
            to,
            count,
            facing,
            rotation,
        } => expand_line(placement_id, *from, *to, *count, *facing, *rotation),
        Layout::Circle {
            center,
            radius,
            count,
            start_angle_deg,
            end_angle_deg,
            facing,
            rotation,
        } => expand_circle(
            placement_id,
            *center,
            *radius,
            *count,
            *start_angle_deg,
            *end_angle_deg,
            *facing,
            *rotation,
        ),
        Layout::Spline {
            points,
            spacing,
            facing,
            rotation,
        } => expand_spline(placement_id, points, *spacing, *facing, *rotation),
    }
}

fn expand_line(
    placement_id: &str,
    from: Vec3,
    to: Vec3,
    count: u32,
    facing: Facing,
    fixed_rotation: Option<Quat>,
) -> Vec<(String, Transform)> {
    if count == 0 {
        return Vec::new();
    }

    let delta = to - from;
    let tangent = if delta.length_squared() > f32::EPSILON {
        delta.normalize()
    } else {
        Vec3::Z
    };

    let denom = (count - 1).max(1) as f32;
    (0..count)
        .map(|i| {
            let t = if count == 1 {
                0.0
            } else {
                i as f32 / denom
            };
            let translation = from.lerp(to, t);
            let rotation = facing_rotation(facing, tangent, None, fixed_rotation);
            (
                instance_id(placement_id, i as usize),
                Transform {
                    translation,
                    rotation,
                    scale: Vec3::ONE,
                },
            )
        })
        .collect()
}

fn expand_circle(
    placement_id: &str,
    center: Vec3,
    radius: f32,
    count: u32,
    start_angle_deg: f32,
    end_angle_deg: f32,
    facing: Facing,
    fixed_rotation: Option<Quat>,
) -> Vec<(String, Transform)> {
    if count == 0 {
        return Vec::new();
    }

    let span = end_angle_deg - start_angle_deg;
    // Evenly around the arc without duplicating the start angle on a full ring.
    (0..count)
        .map(|i| {
            let t = i as f32 / count as f32;
            let angle_deg = start_angle_deg + span * t;
            let angle = angle_deg.to_radians();
            let offset = Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius);
            let translation = center + offset;
            let outward = if offset.length_squared() > f32::EPSILON {
                offset.normalize()
            } else {
                Vec3::X
            };
            // Tangential in XZ: rotate outward 90° CCW around Y.
            let tangent = Vec3::new(-outward.z, 0.0, outward.x);
            let rotation = facing_rotation(facing, tangent, Some(outward), fixed_rotation);
            (
                instance_id(placement_id, i as usize),
                Transform {
                    translation,
                    rotation,
                    scale: Vec3::ONE,
                },
            )
        })
        .collect()
}

fn expand_spline(
    placement_id: &str,
    points: &[Vec3],
    spacing: f32,
    facing: Facing,
    fixed_rotation: Option<Quat>,
) -> Vec<(String, Transform)> {
    if points.len() < 2 || spacing <= 0.0 {
        return Vec::new();
    }

    let samples = sample_catmull_rom(points, spacing);
    samples
        .into_iter()
        .enumerate()
        .map(|(i, (translation, tangent))| {
            let rotation = facing_rotation(facing, tangent, None, fixed_rotation);
            (
                instance_id(placement_id, i),
                Transform {
                    translation,
                    rotation,
                    scale: Vec3::ONE,
                },
            )
        })
        .collect()
}

fn facing_rotation(
    facing: Facing,
    tangent: Vec3,
    outward: Option<Vec3>,
    fixed_rotation: Option<Quat>,
) -> Quat {
    let tangent = if tangent.length_squared() > f32::EPSILON {
        tangent.normalize()
    } else {
        Vec3::Z
    };

    match facing {
        Facing::Identity => Quat::IDENTITY,
        Facing::Along => Quat::from_rotation_arc(Vec3::Z, tangent),
        Facing::Outward => {
            let dir = outward.unwrap_or_else(|| {
                // Right of tangent in XZ.
                let right = Vec3::Y.cross(tangent);
                if right.length_squared() > f32::EPSILON {
                    right.normalize()
                } else {
                    Vec3::X
                }
            });
            Quat::from_rotation_arc(Vec3::Z, dir)
        }
        Facing::Fixed => fixed_rotation.unwrap_or(Quat::IDENTITY),
    }
}

/// Sample positions and tangents along a Catmull-Rom spline at roughly `spacing` intervals.
fn sample_catmull_rom(points: &[Vec3], spacing: f32) -> Vec<(Vec3, Vec3)> {
    let mut samples = Vec::new();
    let mut remaining = 0.0_f32;
    let mut prev_pos = points[0];
    samples.push((prev_pos, initial_tangent(points)));

    let segment_count = points.len() - 1;
    // Subdivide each segment finely, then emit by arc length.
    const SUBDIV: u32 = 16;

    for seg in 0..segment_count {
        let p0 = points[seg.saturating_sub(1)];
        let p1 = points[seg];
        let p2 = points[seg + 1];
        let p3 = points[(seg + 2).min(points.len() - 1)];

        for step in 1..=SUBDIV {
            let t = step as f32 / SUBDIV as f32;
            let pos = catmull_rom(p0, p1, p2, p3, t);
            let delta = pos - prev_pos;
            let dist = delta.length();
            if dist < f32::EPSILON {
                prev_pos = pos;
                continue;
            }

            let mut traveled = 0.0;
            while remaining + (dist - traveled) >= spacing {
                let need = spacing - remaining;
                traveled += need;
                let sample_pos = prev_pos.lerp(pos, traveled / dist);
                let tangent = delta / dist;
                samples.push((sample_pos, tangent));
                remaining = 0.0;
            }
            remaining += dist - traveled;
            prev_pos = pos;
        }
    }

    samples
}

fn initial_tangent(points: &[Vec3]) -> Vec3 {
    let delta = points[1] - points[0];
    if delta.length_squared() > f32::EPSILON {
        delta.normalize()
    } else {
        Vec3::Z
    }
}

fn catmull_rom(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Layout;

    #[test]
    fn point_expands_once() {
        let placement = Placement {
            id: "barrel".into(),
            mesh: "meshes/barrel".into(),
            layout: Layout::Point {
                translation: Vec3::new(1.0, 2.0, 3.0),
                rotation: None,
                scale: None,
            },
            components: vec![],
        };
        let instances = expand(&placement);
        assert_eq!(instances.len(), 1);
        assert_eq!(instances[0].0, "barrel_0");
        assert_eq!(instances[0].1.translation, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(instances[0].1.scale, Vec3::ONE);
    }

    #[test]
    fn line_includes_endpoints() {
        let placement = Placement {
            id: "bricks".into(),
            mesh: "meshes/brick".into(),
            layout: Layout::Line {
                from: Vec3::ZERO,
                to: Vec3::new(10.0, 0.0, 0.0),
                count: 3,
                facing: Facing::Identity,
                rotation: None,
            },
            components: vec![],
        };
        let instances = expand(&placement);
        assert_eq!(instances.len(), 3);
        assert_eq!(instances[0].1.translation, Vec3::ZERO);
        assert_eq!(instances[1].1.translation, Vec3::new(5.0, 0.0, 0.0));
        assert_eq!(instances[2].1.translation, Vec3::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn circle_places_count_instances() {
        let placement = Placement {
            id: "wall".into(),
            mesh: "meshes/wall".into(),
            layout: Layout::Circle {
                center: Vec3::ZERO,
                radius: 10.0,
                count: 4,
                start_angle_deg: 0.0,
                end_angle_deg: 360.0,
                facing: Facing::Outward,
                rotation: None,
            },
            components: vec![],
        };
        let instances = expand(&placement);
        assert_eq!(instances.len(), 4);
        assert!((instances[0].1.translation.x - 10.0).abs() < 1e-4);
    }
}
