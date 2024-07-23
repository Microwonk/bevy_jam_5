use std::str::FromStr;

use bevy::{asset::AssetServer, prelude::*};
use bevy_aseprite_ultra::prelude::AsepriteAnimationBundle;

use super::spawn::level::items::ItemType;

#[derive(Bundle, Default)]
pub struct AsepriteAnimationBundleWrapper {
    pub bundle: AsepriteAnimationBundle,
}

impl AsepriteAnimationBundleWrapper {
    pub fn item(path: &'static str, server: &AssetServer) -> Self {
        Self {
            bundle: AsepriteAnimationBundle {
                aseprite: server.load(path),
                ..default()
            },
        }
    }
}

// TODO maybe just use pngs.
impl AsepriteAnimationBundleWrapper {
    pub fn from_identifier(value: &str, server: &AssetServer) -> Self {
        match ItemType::from_str(value) {
            Ok(ItemType::Hampter) => {
                AsepriteAnimationBundleWrapper::item("aseprite/hampter.aseprite", server)
            }
            Ok(ItemType::Bluberry) => {
                AsepriteAnimationBundleWrapper::item("aseprite/bluberry.aseprite", server)
            }
            _ => todo!(),
        }
    }
}
