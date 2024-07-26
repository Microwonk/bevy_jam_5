//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};
#[cfg(not(target_arch = "wasm32"))]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, log_transitions::<Screen>)
        .add_plugins((
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ));
}
