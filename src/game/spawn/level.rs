pub mod components;
pub mod items;
pub mod spawn;
pub mod ui;

use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::LdtkWorldBundle;
use items::ItemType;

use crate::screen::Screen;

#[allow(dead_code)]
#[derive(SubStates, Debug, Hash, Eq, PartialEq, Clone, Default)]
#[source(Screen = Screen::Playing)]
pub enum LevelState {
    #[default]
    Loading,
    Loaded,
    Paused,
    Finished,
}

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<LevelState>();
    app.enable_state_scoped_entities::<LevelState>();

    app.add_plugins((spawn::plugin, items::plugin, ui::plugin));
    app.observe(spawn_level);
}

#[derive(Resource)]
pub struct CurrentLevel(pub SpawnLevel);

impl CurrentLevel {
    pub fn items(&self) -> HashMap<ItemType, u8> {
        match self.0 {
            // TODO
            // bluberrys are disregarded for now
            SpawnLevel::First => [(ItemType::Hampter, 15), (ItemType::Bluberry, 0)]
                .iter()
                .cloned()
                .collect(),
        }
    }

    pub fn hampters(&self) -> u8 {
        *self
            .items()
            .iter()
            .find_map(|val| match val.0 {
                ItemType::Hampter => Some(val.1),
                _ => None,
            })
            .unwrap_or(&0)
    }

    pub fn bluberries(&self) -> u8 {
        *self
            .items()
            .iter()
            .find_map(|val| match val.0 {
                ItemType::Bluberry => Some(val.1),
                _ => None,
            })
            .unwrap_or(&0)
    }
}

// TODO new Level names
#[derive(Event, Debug, Clone)]
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
    mut next_state: ResMut<NextState<LevelState>>,
) {
    let ldtk_handle = asset_server.load(trigger.event().path());
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
        })
        .insert(StateScoped(Screen::Playing));

    // add current Level Resource
    commands.insert_resource(CurrentLevel(trigger.event().clone()));
    next_state.set(LevelState::Loaded);
}
