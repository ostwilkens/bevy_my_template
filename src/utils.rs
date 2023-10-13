use bevy::{asset::Asset, prelude::*, reflect::*, app::AppExit};
use std::marker::PhantomData;

#[derive(Resource)]
pub struct AssetHandle<T, H>
where
    H: TypeUuid + TypePath + Asset,
{
    pub handle: Handle<H>,
    asset_type: PhantomData<T>,
}

impl<T, H> AssetHandle<T, H>
where
    H: TypeUuid + TypePath + Asset,
{
    pub fn new(handle: Handle<H>) -> Self {
        Self {
            handle: handle,
            asset_type: PhantomData,
        }
    }
}

pub fn is_desktop() -> bool {
    std::env::consts::OS == "macos"
        || std::env::consts::OS == "windows"
        || std::env::consts::OS == "linux"
}

pub fn exit_on_esc(keyboard_input: ResMut<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
