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
                collider: Collider::ball(24.), // half of the pixel (32x32 is the ball sprite)
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

impl From<IntGridCell> for SensorBundle {
    fn from(int_grid_cell: IntGridCell) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        // ladder
        if int_grid_cell.value == 2 {
            SensorBundle {
                collider: Collider::cuboid(8., 8.),
                sensor: Sensor,
                rotation_constraints,
                active_events: ActiveEvents::COLLISION_EVENTS,
            }
        } else {
            SensorBundle::default()
        }
    }
}

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Items(Vec<String>);

impl From<&EntityInstance> for Items {
    fn from(entity_instance: &EntityInstance) -> Self {
        Items(
            entity_instance
                .iter_enums_field("items")
                .expect("items field should be correctly typed")
                .cloned()
                .collect(),
        )
    }
}

// impl LdtkEntity for PlayerBundle {
//     fn bundle_entity(
//         entity_instance: &EntityInstance,
//         _: &LayerInstance,
//         _: Option<&Handle<Image>>,
//         _: Option<&TilesetDefinition>,
//         asset_server: &AssetServer,
//         _: &mut Assets<TextureAtlasLayout>,
//     ) -> Self {
//         Self {
//             worldly: bevy_ecs_ldtk::prelude::Worldly::from_entity_info(entity_instance),
//             hamster: AsepriteAnimationBundleWrapper::from_identifier(
//                 entity_instance.identifier.as_ref(),
//                 asset_server,
//             ),
//             items: Items::from(entity_instance),
//             collider_bundle: ColliderBundle::from(entity_instance),
//             entity_instance: entity_instance.clone(),
//             ..default()
//         }
//     }
// }

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
