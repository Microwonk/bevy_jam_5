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

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct MovementConfig {
    pub movement_speed: f32,
    pub jump_impulse: f32,
    pub maximum_vel: f32,
}

pub const DEFAULT_MOVEMENT_SPEED: f32 = 10.;
pub const DEFAULT_JUMP_IMPULSE: f32 = 200.;
pub const DEFAULT_MAX_VEL: f32 = 300.;

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            movement_speed: DEFAULT_MOVEMENT_SPEED,
            jump_impulse: DEFAULT_JUMP_IMPULSE,
            maximum_vel: DEFAULT_MAX_VEL,
        }
    }
}

// impl MovementConfig {
//     pub fn reset(&mut self) {
//         self.movement_speed = DEFAULT_MOVEMENT_SPEED;
//         self.jump_impulse = DEFAULT_JUMP_IMPULSE;
//         self.maximum_vel = DEFAULT_MAX_VEL;
//     }
// }

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

fn apply_movement(
    mut movement_query: Query<(&MovementController, &MovementConfig, &mut Velocity), With<Player>>,
) {
    for (controller, config, mut vel) in &mut movement_query {
        // rolling movement
        vel.linvel.x += config.movement_speed * controller.movement.x;
        // jump movement
        vel.linvel += config.jump_impulse * controller.jump;
        // maximum velocity calculation
        vel.linvel = vel.linvel.min(Vec2::splat(config.maximum_vel));
    }
}

fn update_collisions(
    rapier_context: Res<RapierContext>,
    mut player: Query<(Entity, &mut MovementController, &Transform), With<Player>>,
    mut player_contact: ResMut<PlayerContact>,
) {
    for (e, mut m, t) in player.iter_mut() {
        // all collected normals of the player entity
        let mut all_normals: Vec<Vec2> = vec![];
        for contact_pair in rapier_context.contact_pairs_with(e) {
            contact_pair.manifolds().for_each(|manifold| {
                for solver_contact in &manifold.raw.data.solver_contacts {
                    all_normals.push(
                        Vec2::new(solver_contact.point.x, solver_contact.point.y)
                            - (t.translation.truncate()),
                    );
                }
            });
        }

        // colliding if there are normals
        let is_colliding = !all_normals.is_empty();
        m.colliding = is_colliding;
        // sums up all vectors
        player_contact.0 = all_normals.iter().sum::<Vec2>();
    }
}
