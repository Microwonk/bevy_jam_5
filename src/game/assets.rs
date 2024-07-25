use bevy::{prelude::*, utils::HashMap};
use bevy_aseprite_ultra::{prelude::Aseprite, BevySprityPlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevySprityPlugin);
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<AsepriteKey>>();
    app.init_resource::<HandleMap<AsepriteKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();
}

#[macro_export]
macro_rules! asset_enum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident => $file_name:expr),* $(,)?
        }
        asset_type: $asset_type:ty,
    ) => {
        $(#[$outer])*
        $vis enum $name {
            $($variant),*
        }

        impl AssetKey for $name {
            type Asset = $asset_type;
        }

        impl FromWorld for HandleMap<$name> {
            fn from_world(world: &mut World) -> Self {
                let asset_server = world.resource::<AssetServer>();
                [
                    $(
                        (
                            $name::$variant,
                            asset_server.load($file_name),
                        )
                    ),*
                ].into()
            }
        }
    };
}

asset_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
    pub enum AsepriteKey {
        Bluberry => "aseprite/bluberry.aseprite",
        Hampter => "aseprite/hampter.aseprite",
        HamsterAnimation => "aseprite/hamster_animation.aseprite",
    }
    asset_type: Aseprite,
}

asset_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
    pub enum ImageKey {
        Hampter => "images/hampter.png",
        NotCollectedHampter => "images/not_collected_hampter.png",
        Bluberry => "images/bluberry.png",
        Hamsterwheel => "images/hamsterwheel.png",
    }
    asset_type: Image,
}

asset_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
    pub enum SfxKey {
        ButtonHover => "audio/sfx/button_hover.ogg",
        ButtonPress => "audio/sfx/button_press.ogg",
        Step1 => "audio/sfx/step1.ogg",
        Step2 => "audio/sfx/step2.ogg",
        Step3 => "audio/sfx/step3.ogg",
        Step4 => "audio/sfx/step4.ogg",
    }
    asset_type: AudioSource,
}

asset_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
    pub enum SoundtrackKey {
        Credits => "audio/soundtracks/monkeys_spinning_monkeys.ogg",
        Gameplay => "audio/soundtracks/fluffing_a_duck.ogg",
    }
    asset_type: AudioSource,
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
