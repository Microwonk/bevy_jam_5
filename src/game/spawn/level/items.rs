use bevy::{asset::Handle, prelude::*, utils::HashMap};
use bevy_ecs_ldtk::{
    app::{LdtkEntity, LdtkEntityAppExt},
    ldtk::{LayerInstance, TilesetDefinition},
    prelude::LdtkFields,
    EntityInstance,
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    game::{
        animation::AsepriteAnimationBundleWrapper,
        movement::{MovementConfig, DEFAULT_MOVEMENT_SPEED},
    },
    screen::Screen,
};

use super::components::SensorBundle;

#[derive(Resource)]
struct BobbingTimer(Timer);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            bob_items,
            collect_items,
            display_items,
            boost_stats,
            temporary_items,
        )
            .run_if(in_state(Screen::Playing)),
    )
    .register_type::<BluberryTimer>()
    .init_resource::<BluberryTimer>()
    .insert_resource(BobbingTimer(Timer::from_seconds(
        0.01,
        TimerMode::Repeating,
    )))
    .register_ldtk_entity::<HampterBundle>(ItemType::Hampter.to_str())
    .register_ldtk_entity::<BluberryBundle>(ItemType::Bluberry.to_str());
}

fn bob_items(
    time: Res<Time>,
    mut timer: ResMut<BobbingTimer>,
    mut items: Query<&mut Transform, With<ItemType>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut transform in items.iter_mut() {
            // Calculate the new y position using a sine wave
            let elapsed_time = time.elapsed_seconds();
            let amplitude = 0.5; // Amplitude of the bobbing motion
            let frequency = 7.0; // Frequency of the bobbing motion
            transform.translation.y += amplitude * (elapsed_time * frequency).sin();
        }
    }
}

// boost stats from items
// TODO, on a timer
fn boost_stats(mut query: Query<(&Items, &mut MovementConfig)>) {
    for (i, mut config) in query.iter_mut() {
        for (item, count) in &i.0 {
            match item {
                ItemType::Bluberry => {
                    config.movement_speed =
                        DEFAULT_MOVEMENT_SPEED + 10. * *count.clamp(&0, &1) as f32;
                }
                _ => {
                    // do nothing
                }
            }
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct BluberryTimer(pub Timer);

impl Default for BluberryTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

fn temporary_items(
    time: Res<Time>,
    mut bluberry_timer: ResMut<BluberryTimer>,
    mut items: Query<&mut Items>,
) {
    for mut i in items.iter_mut() {
        for (item, count) in &mut i.0 {
            match item {
                // can only be in here once
                ItemType::Bluberry => {
                    if bluberry_timer.0.just_finished() && *count != 0 {
                        *count -= 1;
                        bluberry_timer.0.reset()
                    }
                    if *count > 0 {
                        bluberry_timer.0.tick(time.delta());
                    }
                }
                _ => {
                    // do nothing
                    // TODO if other items are added
                }
            }
        }
    }
}

fn collect_items(
    mut commands: Commands,
    mut item_holders: Query<&mut Items>,
    items: Query<(Entity, &ItemType)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let (Ok(mut holder), Ok(item)) =
                    (item_holders.get_mut(*collider_a), items.get(*collider_b))
                {
                    let count = holder.0.entry(item.1.clone()).or_insert(0);
                    *count += 1;
                    // remove entity
                    commands.entity(item.0).despawn();
                }

                if let (Ok(mut holder), Ok(item)) =
                    (item_holders.get_mut(*collider_b), items.get(*collider_a))
                {
                    let count = holder.0.entry(item.1.clone()).or_insert(0);
                    *count += 1;
                    // remove entity
                    commands.entity(item.0).despawn();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                // ignore
            }
        }
    }
}

fn display_items(items: Query<&mut Items>, inp: Res<ButtonInput<KeyCode>>) {
    if !inp.just_pressed(KeyCode::KeyI) {
        return;
    }
    for i in items.iter() {
        for (k, v) in i.0.iter() {
            println!("Item:{:?}", (k, v));
        }
    }
}

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Items(pub HashMap<ItemType, u8>);

impl From<&EntityInstance> for Items {
    fn from(entity_instance: &EntityInstance) -> Self {
        Items(
            entity_instance
                .iter_enums_field("items")
                .expect("items field should be correctly typed")
                .cloned()
                .map(|item| (ItemType::from_str(&item).unwrap_or_default(), 0))
                .collect(),
        )
    }
}

use paste::paste;
use std::str::FromStr;

macro_rules! generate_item_type_and_bundles {
    ($($item:ident),*) => {
        #[derive(Default, Debug, Component, Clone, Eq, PartialEq, Hash)]
        pub enum ItemType {
            #[default]
            None, // used
            $($item),*
        }

        impl FromStr for ItemType {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "None" => Ok(ItemType::None),
                    $(stringify!($item) => Ok(ItemType::$item)),*,
                    _ => Err(()),
                }
            }
        }

        impl ItemType {
            pub fn to_str(&self) -> &str {
                match self {
                    ItemType::None => "None",
                    $(ItemType::$item => stringify!($item)),*
                }
            }
        }
        $(
        paste! {
            #[derive(Bundle, Default)]
            pub struct [<$item Bundle>] {
                pub aseprite_bundle: AsepriteAnimationBundleWrapper,
                pub sensor_bundle: SensorBundle,
                pub item_type: ItemType,
            }

            impl LdtkEntity for [<$item Bundle>] {
                fn bundle_entity(
                    entity_instance: &EntityInstance,
                    _: &LayerInstance,
                    _: Option<&Handle<Image>>,
                    _: Option<&TilesetDefinition>,
                    asset_server: &AssetServer,
                    _: &mut Assets<TextureAtlasLayout>,
                ) -> Self {
                    Self {
                        aseprite_bundle: AsepriteAnimationBundleWrapper::from_identifier(
                            entity_instance.identifier.as_ref(),
                            asset_server,
                        ),
                        sensor_bundle: SensorBundle::from(entity_instance),
                        item_type: ItemType::$item,
                    }
                }
            }
        }
        )*
    }
}

generate_item_type_and_bundles!(Hampter, Bluberry);
