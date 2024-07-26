use std::cmp::Ordering;

use bevy::prelude::*;

use crate::game::assets::{HandleMap, ImageKey};
use crate::screen::Screen;
use crate::ui::prelude::*;

use super::items::{BluberryTimer, ItemType, Items};
use super::{CurrentLevel, LevelTimer};

pub(super) fn plugin(app: &mut App) {
    app.observe(enter_level).add_systems(
        Update,
        (update_hampter_ui, update_bluberry_ui, update_level_timer_ui)
            .run_if(in_state(Screen::Playing)),
    );
}

// u8 used to store which hampter has been collected
#[derive(Component, Default, PartialEq, PartialOrd, Ord, Eq)]
struct HampterIconMarker(pub u8);

#[derive(Component, Default)]
struct BluberryIconMarker;

#[derive(Component, Default)]
struct BluberryNodeMarker;

#[derive(Component, Default)]
struct BluberryTimerMarker;

#[derive(Component, Default)]
struct LevelTimerMarker;

#[derive(Event)]
pub struct StartLevelUi;

pub fn enter_level(
    _: Trigger<StartLevelUi>,
    mut commands: Commands,
    handles: Res<HandleMap<ImageKey>>,
    current_level: Res<CurrentLevel>,
) {
    // hampter ui spawn
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::NONE),
            // border_radius: BorderRadius::all(Val::Px(25.)),
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

    // bluberry ui spawn
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::NONE),
            // border_radius: BorderRadius::all(Val::Px(25.)),
            style: Style {
                width: Val::Percent(0.),
                height: Val::Percent(7.5),
                top: Val::Percent(10.),
                justify_self: JustifySelf::End,
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(BluberryNodeMarker)
        .insert(StateScoped(Screen::Playing));

    // bluberry timer
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::NONE),
            // border_radius: BorderRadius::all(Val::Px(25.)),
            style: Style {
                width: Val::Percent(5.),
                height: Val::Percent(5.),
                top: Val::Percent(17.),
                justify_self: JustifySelf::End,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(StateScoped(Screen::Playing))
        .with_children(|c| {
            c.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 24.0,
                    color: Color::BLACK,
                    ..default()
                },
            ))
            .insert(BluberryTimerMarker);
        });

    // level timer
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::NONE),
            // border_radius: BorderRadius::all(Val::Px(25.)),
            style: Style {
                width: Val::Auto,
                height: Val::Percent(5.),
                justify_self: JustifySelf::End,
                align_self: AlignSelf::End,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(StateScoped(Screen::Playing))
        .with_children(|c| {
            c.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 24.0,
                    color: Color::BLACK,
                    ..default()
                },
            ))
            .insert(LevelTimerMarker);
        });
}

// could probably be done better
// TODO
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

fn update_bluberry_ui(
    mut bluberry_ui_node: Query<(Entity, &mut Style), With<BluberryNodeMarker>>,
    mut bluberry_timer_text: Query<&mut Text, With<BluberryTimerMarker>>,
    bluberry_timer: Res<BluberryTimer>,
    bluberry_icons: Query<Entity, With<BluberryIconMarker>>,
    handles: Res<HandleMap<ImageKey>>,
    mut commands: Commands,
    items: Query<&Items>,
) {
    if let Ok((e, mut s)) = bluberry_ui_node.get_single_mut() {
        // Collect bluberry icons first
        let bluberry_is: Vec<Entity> = bluberry_icons.iter().collect();

        // Create a list of entities to despawn
        let mut entities_to_despawn = vec![];

        commands.entity(e).with_children(|child| {
            for item in items.iter() {
                let bluberry_count = item.0.get(&ItemType::Bluberry).unwrap_or(&0);

                // adjust node width
                s.width = Val::Percent(4.5 * *bluberry_count as f32);
                let length = bluberry_is.len() as u8;

                match length.cmp(bluberry_count) {
                    Ordering::Less => {
                        for _ in length..*bluberry_count {
                            child
                                .icon(handles[&ImageKey::Bluberry].clone_weak())
                                .insert(BluberryIconMarker);
                        }
                    }
                    Ordering::Greater => {
                        for over in *bluberry_count..length {
                            entities_to_despawn.push(bluberry_is[over as usize]);
                        }
                    }
                    Ordering::Equal => {
                        *bluberry_timer_text.single_mut() = Text::from_section(
                            if *bluberry_count == 0 {
                                "".into()
                            } else {
                                format!(
                                    "{:.2}",
                                    (bluberry_timer.0.duration() * *bluberry_count as u32
                                        - bluberry_timer.0.elapsed())
                                    .as_secs_f32()
                                )
                            },
                            TextStyle {
                                color: Color::BLACK,
                                ..default()
                            },
                        )
                    }
                }
            }
        });

        // Despawn entities outside of the with_children closure
        for entity in entities_to_despawn {
            commands.entity(entity).despawn();
        }
    }
}

fn update_level_timer_ui(
    mut timer_text: Query<&mut Text, With<LevelTimerMarker>>,
    level_timer: Res<LevelTimer>,
) {
    if let Ok(mut timer_text) = timer_text.get_single_mut() {
        *timer_text = Text::from_section(
            format!("{:.2}", level_timer.0.elapsed_secs()),
            TextStyle {
                color: Color::BLACK,
                ..default()
            },
        );
    }
}
