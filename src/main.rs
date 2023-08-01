use bevy::{
    app::AppExit,
    asset::{Asset, ChangeWatcher},
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::camera::ScalingMode,
    time::Stopwatch,
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions,
};
use bevy_mod_picking::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use button::{interact_button, ButtonCommands};
use mute::MuteButtonPlugin;
use std::{marker::PhantomData, time::Duration};

mod button;
mod mute;

static PRIMARY_COLOR_HUE: f32 = 0.5;
static MENU_MUSIC_VOLUME: f32 = 0.36;
static PLAYING_MUSIC_VOLUME: f32 = 0.66;

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

#[derive(Component)]
struct Music;

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

#[derive(Component)]
struct ScoreText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
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

    // spawn score text
    commands.spawn((
        ScoreText,
        TextBundle::from_section(
            format!("Score: 0"),
            TextStyle {
                font_size: 64.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::new(Val::Auto, Val::Auto, Val::Vh(20.0), Val::Auto),
            display: Display::None,
            ..default()
        }),
    ));

    // circle mesh & material
    // commands.insert_resource(AssetHandle::<Circle, ColorMaterial>::new(
    //     materials.add(Color::hsl((PRIMARY_COLOR_HUE - 0.5) * 360.0, 0.7, 0.8).into()),
    // ));
    // commands.insert_resource(AssetHandle::<Circle, Mesh>::new(
    //     meshes.add(shape::Circle::new(70.).into()).into(),
    // ));
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

#[derive(Resource)]
struct Score(usize);

fn on_enter_menu(mut commands: Commands, music_controller: Query<&AudioSink, With<Music>>) {
    // set music volume
    for sink in music_controller.iter() {
        sink.set_volume(MENU_MUSIC_VOLUME);
    }

    commands
        .spawn_text_button("Play", PRIMARY_COLOR_HUE)
        .insert(PlayButton);
}

fn on_exit_menu() {}

#[derive(Resource)]
struct GameTime(Stopwatch);

fn on_enter_playing(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut q_score_text: Query<&mut Style, With<ScoreText>>,
    music_controller: Query<&AudioSink, With<Music>>,
    // circle_mesh: Res<AssetHandle<Circle, Mesh>>,
    // circle_mat: Res<AssetHandle<Circle, ColorMaterial>>,
) {
    // reset score
    score.0 = 0;

    // hide score text
    for mut style in q_score_text.iter_mut() {
        style.display = Display::None;
    }

    // start stopwatch
    commands.insert_resource(GameTime(Stopwatch::new()));

    // increase music volume
    for sink in music_controller.iter() {
        sink.set_volume(PLAYING_MUSIC_VOLUME);
    }

    // spawn one circle
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: circle_mesh.handle.clone().into(),
    //         material: circle_mat.handle.clone().into(),
    //         transform: Transform::from_translation(vec3(0.0, 0.0, 0.0)),
    //         ..Default::default()
    //     },
    //     Circle,
    //     PickableBundle::default(),
    //     RaycastPickTarget::default(),
    //     On::<Pointer<Down>>::run(on_click_circle),
    // ));
}

fn on_exit_playing(
    mut commands: Commands,
    mut q_score_text: Query<(&mut Style, &mut Text), With<ScoreText>>,
    score: Res<Score>,
) {
    // display score text
    for (mut style, mut text) in q_score_text.iter_mut() {
        style.display = Display::Flex;
        for section in text.sections.iter_mut() {
            section.value = format!("Score: {}", score.0);
        }
    }

    // remove sw
    commands.remove_resource::<GameTime>();
}

fn exit_on_esc(keyboard_input: ResMut<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn on_click_circle(
    event: Listener<Pointer<Down>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
) {
    score.0 += 1;

    // despawn clicked circle
    // commands.entity(event.target).despawn_recursive();
}

fn update_music_speed(
    music_controller: Query<&AudioSink, With<Music>>,
    sw: Option<Res<GameTime>>,
    time: Res<Time>,
) {
    let target_speed = if let Some(sw) = sw {
        1.0 + sw.0.elapsed_secs() * 0.015
    } else {
        1.0
    };

    for sink in music_controller.iter() {
        let current_speed = sink.speed();
        let n = time.delta_seconds() * 8.0;
        let new_speed = current_speed * (1.0 - n) + target_speed * n;
        sink.set_speed(new_speed.clamp(0.0, 5.0));
    }
}

fn while_playing(
    time: Res<Time>,
    mut commands: Commands,
    mut game_time: ResMut<GameTime>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    game_time.0.tick(time.delta());
}
