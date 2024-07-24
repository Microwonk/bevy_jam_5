use bevy::prelude::*;

use crate::game::assets::{HandleMap, ImageKey};
use crate::screen::Screen;
use crate::ui::prelude::*;

use super::items::{ItemType, Items};
use super::{CurrentLevel, LevelState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LevelState::Loaded), enter_level)
        .add_systems(
            Update,
            update_hampter_ui.run_if(in_state(LevelState::Loaded)),
        );
}

// u8 used to store which hampter has been collected
#[derive(Component, Default, PartialEq, PartialOrd, Ord, Eq)]
struct HampterIconMarker(pub u8);

fn enter_level(
    mut commands: Commands,
    handles: Res<HandleMap<ImageKey>>,
    current_level: Res<CurrentLevel>,
) {
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::srgba(0., 0., 0., 0.5)),
            border_radius: BorderRadius::all(Val::Px(25.)),
            style: Style {
                width: Val::Percent(4.5 * current_level.hampters() as f32),
                height: Val::Percent(7.5),
                justify_self: JustifySelf::End,
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(StateScoped(Screen::Playing))
        .with_children(|child| {
            for (item, count) in current_level.items() {
                if matches!(item, ItemType::Hampter) {
                    for i in 0..count {
                        child
                            .icon(handles[&ImageKey::NotCollectedHampter].clone_weak())
                            .insert(HampterIconMarker(i));
                    }
                }
            }
        });
}

fn update_hampter_ui(
    mut commands: Commands,
    icons: Query<(Entity, &HampterIconMarker)>,
    handles: Res<HandleMap<ImageKey>>,
    items: Query<&Items>,
) {
    for item in items.iter() {
        let hampter_count = item.0.get(&ItemType::Hampter).unwrap_or(&0);

        for (e, marker) in icons.iter().sort::<&HampterIconMarker>() {
            if marker.0 >= *hampter_count {
                break;
            }
            commands.entity(e).remove::<UiImage>().insert(UiImage {
                texture: handles[&ImageKey::Hampter].clone_weak(),
                ..default()
            });
            // *handle = server.load("images/hampter.aseprite");
        }
    }
}
