//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

// TODO use a kinematic velocity based character maybe

use bevy::prelude::*;
use bevy_rapier2d::{dynamics::Velocity, plugin::RapierContext};

use crate::{screen::Screen, AppSet};

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.register_type::<PlayerContact>();
    app.init_resource::<PlayerContact>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.add_systems(
        FixedUpdate,
        (apply_movement, update_collisions).run_if(in_state(Screen::Playing)),
    );
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayerContact(pub Vec2);

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct MovementController {
    pub movement: Vec2,
    pub jump: Vec2,
    pub colliding: bool,
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
    player_contact: Res<PlayerContact>,
) {
    for mut controller in &mut controller_query {
        // Collect directional input.
        let mut intent = Vec2::ZERO;
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            intent.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            intent.x += 1.0;
        }

        let mut jump_intent = Vec2::ZERO;

        if input.pressed(KeyCode::Space) {
            jump_intent = -player_contact.0.normalize_or_zero();
        }

        let intent = intent.normalize_or_zero();

        // Apply movement intent to controllers.
        controller.movement = intent;
        controller.jump = jump_intent;
    }
}

fn apply_movement(mut movement_query: Query<(&MovementController, &mut Velocity), With<Player>>) {
    for (controller, mut vel) in &mut movement_query {
        // TODO: control speed differently
        // rolling movement
        vel.linvel.x += 10. * controller.movement.x;
        // jump movement
        vel.linvel += controller.jump * 300.;
        // maximum velocity calculation
        let max_vel = Vec2::splat(300.);
        vel.linvel = vel.linvel.min(max_vel);
    }
}

fn update_collisions(
    rapier_context: Res<RapierContext>,
    mut player: Query<(Entity, &mut MovementController), With<Player>>,
    mut player_contact: ResMut<PlayerContact>,
    mut correction: Local<Option<f32>>,
) {
    for (e, mut m) in player.iter_mut() {
        // all collected normals of the player entity
        let mut all_normals: Vec<Vec2> = vec![];
        for contact_pair in rapier_context.contact_pairs_with(e) {
            let mut normals: Vec<Vec2> = contact_pair
                .manifolds()
                .map(|manifold| {
                    if let Some(c) = *correction {
                        manifold.normal() * c
                    } else {
                        // FIXME my god
                        // very hacksawy
                        // basically detecting if the first time touching the ground (as every level starts with the player touching the ground)
                        // results in the expecting y component  of the vector (pointing up), and if it does not, it compensates by negating the vector
                        // please don't tell anyone
                        if manifold.normal().y > 0. {
                            *correction = Some(-1.);
                        } else {
                            *correction = Some(1.);
                        }
                        if let Some(c) = *correction {
                            manifold.normal() * c
                        } else {
                            manifold.normal()
                        }
                    }
                })
                .collect();

            all_normals.append(&mut normals);
        }

        let is_colliding = !all_normals.is_empty();
        m.colliding = is_colliding;
        // sums up all vectors and normalizes them
        player_contact.0 = all_normals.iter().sum::<Vec2>();
    }
}
