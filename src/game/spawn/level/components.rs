use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub restitution: Restitution,
    pub damping: Damping,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        match int_grid_cell.value {
            // 1x1 Concave bottom right
            4 => create_ramp_collider(-7.5, 7.5, 15.0, f32::to_radians(270.), 20),
            // 1x1 Concave bottom left
            5 => create_ramp_collider(7.5, 7.5, 15.0, f32::to_radians(180.), 20),
            // 1x1 Concave top left
            6 => create_ramp_collider(-7.5, -7.5, 15.0, 0., 20),
            // 1x1 Concave top right
            7 => create_ramp_collider(7.5, -7.5, 15.0, f32::to_radians(90.), 20),
            _ => ColliderBundle::default(),
        }
    }
}

// simple ramp collider using some magic trigonometry
fn create_ramp_collider(
    center_x: f32,
    center_y: f32,
    radius: f32,
    start_angle: f32,
    num_segments: usize,
) -> ColliderBundle {
    let vertices = (0..=num_segments).map(|i| {
        let theta = start_angle + PI / 2. * (i as f32 / num_segments as f32);
        let x = center_x + radius * theta.cos();
        let y = center_y + radius * theta.sin();
        Vec2::new(x, y)
    });

    ColliderBundle {
        collider: Collider::polyline(vertices.collect(), None),
        rigid_body: RigidBody::Fixed,
        ..default()
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct RampBundle {
    #[from_int_grid_cell]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::ball(24.), // half of the resolution (48x48 is the ball sprite)
                rigid_body: RigidBody::Dynamic,
                friction: Friction::default(),
                restitution: Restitution {
                    coefficient: 0.7,
                    combine_rule: CoefficientCombineRule::Multiply,
                },
                damping: Damping {
                    linear_damping: 1.5,
                    angular_damping: 1.0,
                },
                ..default()
            },
            // Handle other entities from LDtk
            _ => ColliderBundle::default(),
        }
    }
}

impl From<&EntityInstance> for SensorBundle {
    fn from(entity_instance: &EntityInstance) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Hampter" | "Bluberry" => SensorBundle {
                collider: Collider::cuboid(8., 8.),
                sensor: Sensor,
                rotation_constraints,
                active_events: ActiveEvents::COLLISION_EVENTS,
            },
            // TODO
            _ => todo!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
