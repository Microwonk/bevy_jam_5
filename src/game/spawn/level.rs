pub mod components;
pub mod spawn;

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(spawn::plugin);
    app.observe(spawn_level);
}

// TODO new Level names
#[derive(Event, Debug)]
pub enum SpawnLevel {
    First,
}

impl SpawnLevel {
    pub fn path(&self) -> String {
        let p = match self {
            SpawnLevel::First => "Test.ldtk",
        };
        format!("levels/{}", p)
    }
}

fn spawn_level(
    trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ldtk_handle = asset_server.load(trigger.event().path());
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
        })
        .insert(StateScoped(Screen::Playing));
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
}
