use bevy::{
    app::AppExit,
    asset::ChangeWatcher,
    audio::{PlaybackMode, Volume, VolumeLevel},
    ecs::system::EntityCommands,
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions,
};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use button::{ButtonCommands, interact_button};
use mute::MuteButtonPlugin;
use std::time::Duration;

mod mute;
mod button;

static PRIMARY_COLOR_HUE: f32 = 0.5;
static MENU_MUSIC_VOLUME: f32 = 0.4;
static PLAYING_MUSIC_VOLUME: f32 = 0.67;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // mode: WindowMode::Fullscreen,
                    // present_mode: PresentMode::AutoNoVsync,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(1000)),
                ..Default::default()
            }),
    )
    .insert_resource(ClearColor(Color::hsl(PRIMARY_COLOR_HUE * 360.0, 0.2, 0.2)))
    .register_type::<PrimaryColorHue>()
    .insert_resource(PrimaryColorHue(PRIMARY_COLOR_HUE))
    .add_plugins(MuteButtonPlugin)
    .add_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(OnEnter(GameState::Menu), on_enter_menu)
    .add_systems(OnExit(GameState::Menu), on_exit_menu)
    .add_systems(OnEnter(GameState::Playing), on_enter_playing)
    .add_systems(OnExit(GameState::Playing), on_exit_playing)
    .add_systems(Update, (exit_on_esc.run_if(is_desktop), interact_button))
    .add_systems(
        Update,
        (interact_play_button,).run_if(in_state(GameState::Menu)),
    );

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_plugins(ScreenDiagnosticsPlugin::default());
        app.add_plugins(ScreenFrameDiagnosticsPlugin);
    }

    app.run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

#[derive(Component)]
struct PlayButton;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PrimaryColorHue(#[inspector(min = 0.0, max = 1.0)] f32);

fn is_desktop() -> bool {
    std::env::consts::OS == "macos" || std::env::consts::OS == "windows"
}

#[derive(Component, Default)]
struct Player;

#[derive(Component)]
struct Music;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // music
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Relative(VolumeLevel::new(MENU_MUSIC_VOLUME)),
                ..default()
            },
            ..default()
        },
        Music,
    ));

    // camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(720.0),
            ..default()
        },
        ..default()
    });
}

fn interact_play_button(
    mut q_button: Query<(&Interaction, &mut Style), (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some((interaction, mut style)) = q_button.iter_mut().next() {
        match interaction {
            Interaction::Pressed => {
                style.display = Display::None;
                next_state.set(GameState::Playing);
            }
            _ => {}
        };
    }
}

fn on_enter_menu(mut commands: Commands, music_controller: Query<&AudioSink, With<Music>>) {
    // set music volume
    for sink in music_controller.iter() {
        sink.set_volume(MENU_MUSIC_VOLUME);
    }

    commands.spawn_text_button("Play", PRIMARY_COLOR_HUE).insert(PlayButton);
}

fn on_exit_menu() {}

fn on_enter_playing(music_controller: Query<&AudioSink, With<Music>>) {
    // increase music volume
    for sink in music_controller.iter() {
        sink.set_volume(PLAYING_MUSIC_VOLUME);
    }
}

fn on_exit_playing() {}

fn exit_on_esc(keyboard_input: ResMut<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
