pub mod components;
pub mod items;
pub mod spawn;
pub mod ui;

use bevy::{prelude::*, time::Stopwatch, utils::HashMap};
use bevy_ecs_ldtk::LdtkWorldBundle;
use items::ItemType;
use ui::StartLevelUi;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((spawn::plugin, items::plugin, ui::plugin));
    app.observe(spawn_level);
    app.add_systems(Update, tick_level_timer.run_if(in_state(Screen::Playing)));
    app.add_systems(OnExit(Screen::Playing), reset_level_timer);
}

#[derive(Resource, Default)]
pub struct LevelTimer(pub Stopwatch);

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

    #[allow(dead_code)]
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

pub fn spawn_level(
    trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // add current Level Resource
    commands.insert_resource(CurrentLevel(trigger.event().clone()));
    let ldtk_handle = asset_server.load(trigger.event().path());
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
        })
        .insert(StateScoped(Screen::Playing));

    // level stopwatch
    commands.init_resource::<LevelTimer>();
    commands.trigger(StartLevelUi);
}

fn tick_level_timer(time: Res<Time>, mut timer: ResMut<LevelTimer>) {
    timer.0.tick(time.delta());
}

fn reset_level_timer(mut timer: ResMut<LevelTimer>) {
    timer.0.reset();
}
