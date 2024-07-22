use bevy::{asset::AssetServer, prelude::*};
use bevy_aseprite_ultra::prelude::AsepriteAnimationBundle;

#[derive(Bundle, Default)]
pub struct AsepriteAnimationBundleWrapper {
    pub bundle: AsepriteAnimationBundle,
}

impl AsepriteAnimationBundleWrapper {
    pub fn from_identifier(value: &str, server: &AssetServer) -> Self {
        match value {
            "Player" => AsepriteAnimationBundleWrapper {
                bundle: AsepriteAnimationBundle {
                    aseprite: server.load("images/hamster-animation.aseprite"),
                    transform: Transform::from_xyz(0., -10., -1.),
                    ..default()
                },
            },
            // TODO
            _ => todo!(),
        }
    }
}
