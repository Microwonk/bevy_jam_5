use bevy::prelude::*;
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity, Worldly};

use crate::game::{animation::AsepriteAnimationBundleWrapper, movement::MovementController};

use super::level::components::{ColliderBundle, Items};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevySprityPlugin)
        .add_systems(Update, (on_player_bundle_added, update_hamster_orientation));
}

#[derive(Component)]
pub struct Hamster;

fn on_player_bundle_added(
    server: Res<AssetServer>,
    mut commands: Commands,
    player: Query<Entity, Added<Player>>,
) {
    for p in player.iter() {
        commands.entity(p).with_children(|child| {
            child
                .spawn(AsepriteAnimationBundleWrapper::from_identifier(
                    "Player", &server,
                ))
                .insert(Hamster);
        });
    }
}

// A WHOLE BUNCH OF VECTOR/MATH MAGIC
// don't ask me how i figured this out
fn update_hamster_orientation(
    mut player_query: Query<(&Parent, &mut Transform), With<Hamster>>,
    parent_query: Query<&Transform, Without<Hamster>>,
    mc_query: Query<&MovementController>,
    mut last_dir: Local<f32>,
) {
    for (parent, mut transform) in player_query.iter_mut() {
        if let Ok(parent_transform) = parent_query.get(parent.get()) {
            for controller in mc_query.iter() {
                // ROTATION
                let new_rotation = if controller.movement.x > 0. {
                    // Face right
                    *last_dir = controller.movement.x;
                    Quat::from_rotation_y(0.)
                } else if controller.movement.x < 0. {
                    // Face left (180 degrees around Y axis)
                    *last_dir = controller.movement.x;
                    Quat::from_rotation_y(std::f32::consts::PI)
                }
                // case that it is zero
                else if *last_dir > 0. {
                    Quat::from_rotation_y(0.)
                } else {
                    Quat::from_rotation_y(std::f32::consts::PI)
                };

                // Counteract the parent's rotation
                transform.rotation = parent_transform.rotation.inverse() * new_rotation;

                // TRANSLATION
                // FIXME change the offset if necessary
                let offset = Vec3::new(0., -10., -1.);
                // Apply the inverse rotation to the offset to negate the translation effect
                let negated_offset = parent_transform.rotation.inverse() * offset;
                // move around the centerpoint to negate this angle with an offset from above
                transform.translation = negated_offset;
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("images/hamsterwheel.png")]
    pub ball: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub movement_controller: MovementController,
    #[worldly]
    pub worldly: Worldly,

    // Build Items Component manually by using `impl From<&EntityInstance>`
    #[from_entity_instance]
    items: Items,
    // The whole EntityInstance can be stored directly as an EntityInstance component
    entity_instance: EntityInstance,
}
