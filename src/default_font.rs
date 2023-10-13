use bevy::prelude::*;

pub struct DefaultFontPlugin {
    pub font_path: &'static str,
}

impl Plugin for DefaultFontPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DefaultFontPath(self.font_path))
        .add_systems(
            Startup,
            setup_default_font
        )
        .add_systems(
            Update,
            set_default_font.run_if(resource_exists::<DefaultFontHandle>()),
        );
    }

}

fn setup_default_font(mut commands: Commands, asset_server: Res<AssetServer>, font_path: Res<DefaultFontPath>) {
    let font = asset_server.load(font_path.0);
    commands.insert_resource(DefaultFontHandle(font));
}

#[derive(Resource)]
struct DefaultFontPath(&'static str);

#[derive(Resource)]
struct DefaultFontHandle(Handle<Font>);

#[derive(Component)]
pub struct DefaultFont;

fn set_default_font(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    font_handle: Res<DefaultFontHandle>,
) {
    if let Some(font) = fonts.remove(&font_handle.0) {
        fonts.set_untracked(TextStyle::default().font, font);
        commands.remove_resource::<DefaultFontHandle>();
        info!("Default font set");
    }
}
