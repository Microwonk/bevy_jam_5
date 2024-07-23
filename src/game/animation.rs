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

impl AsepriteAnimationBundleWrapper {
    pub fn from_identifier(value: &str, server: &AssetServer) -> Self {
        match ItemType::from_str(value) {
            Ok(ItemType::Hampter) => {
                AsepriteAnimationBundleWrapper::item("images/hampter.aseprite", server)
            }
            Ok(ItemType::Bluberry) => {
                AsepriteAnimationBundleWrapper::item("images/bluberry.aseprite", server)
            }
            Err(_) => match value {
                "Player" => AsepriteAnimationBundleWrapper {
                    bundle: AsepriteAnimationBundle {
                        aseprite: server.load("images/hamster-animation.aseprite"),
                        transform: Transform::from_xyz(0., -10., -1.),
                        ..default()
                    },
                },
                _ => todo!(),
            },
            // TODO
            _ => todo!(),
        }
    }
}
