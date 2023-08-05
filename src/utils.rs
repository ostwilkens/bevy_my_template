use bevy::{asset::Asset, prelude::*, reflect::*};
use std::marker::PhantomData;

#[derive(Resource)]
struct AssetHandle<T, H>
where
    H: TypeUuid + TypePath + Asset,
{
    handle: Handle<H>,
    asset_type: PhantomData<T>,
}

impl<T, H> AssetHandle<T, H>
where
    H: TypeUuid + TypePath + Asset,
{
    fn new(handle: Handle<H>) -> Self {
        Self {
            handle: handle,
            asset_type: PhantomData,
        }
    }
}
