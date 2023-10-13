// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    math::vec3,
    prelude::*,
    render::camera::ScalingMode,
    window::{PrimaryWindow, WindowResized},
};
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::{lens::*, *};
// use button::interact_button;
use default_font::{DefaultFont, DefaultFontPlugin};
use framerate::{FramerateIsStable, FramerateMonitorPlugin};
use std::{f32::consts::PI, time::Duration};
use utils::*;
#[cfg(target_arch = "wasm32")]
use web_event::send_loaded_event;

mod button;
mod default_font;
mod framerate;
mod mute;
mod utils;
#[cfg(target_arch = "wasm32")]
mod web_event;

static PRIMARY_COLOR_HUE: f32 = 0.5;
// static MENU_MUSIC_VOLUME: f32 = 0.36;
// static PLAYING_MUSIC_VOLUME: f32 = 0.66;
static WINDOW_WORLD_HEIGHT: f32 = 10.0;

fn main() {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Game".to_owned(),
            // mode: WindowMode::Fullscreen,
            // present_mode: PresentMode::AutoNoVsync,
            canvas: Some("#bevy".to_owned()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    });
    // this comes in bevy 0.12
    // #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // if !web && debug
    // default_plugins.set(AssetPlugin::processed_dev());

    app.add_plugins(default_plugins);
    app.add_plugins(TweeningPlugin);
    app.add_plugins(FramerateMonitorPlugin);
    app.add_plugins(DefaultFontPlugin {
        font_path: "Nunito-Regular.ttf",
    });
    app.insert_resource(ClearColor(Color::NONE));
    // app.insert_resource(Score(0));
    app.insert_resource(PrimaryColorHue(PRIMARY_COLOR_HUE));
    // app.add_plugins(MuteButtonPlugin);
    app.add_state::<GameState>();
    app.add_systems(
        Update,
        (wait_for_loading,).run_if(in_state(GameState::Loading)),
    );
    app.add_systems(
        OnEnter(GameState::Loading),
        (pre_load_setup, load_assets).chain(),
    );
    // app.add_systems(Update, interact_button);
    app.add_systems(OnExit(GameState::Loading), setup);
    app.add_systems(OnExit(GameState::Loading), spawn_background);
    // app.add_systems(OnEnter(GameState::Menu), on_enter_menu);
    // app.add_systems(OnExit(GameState::Menu), on_exit_menu);
    // app.add_systems(OnEnter(GameState::Playing), on_enter_playing);
    // app.add_systems(OnExit(GameState::Playing), on_exit_playing);
    // app.add_systems(
    //     Update,
    //     (interact_play_button,).run_if(in_state(GameState::Menu)),
    // );
    app.add_systems(Update, resize_background_plane);
    
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, exit_on_esc);

    #[cfg(target_arch = "wasm32")]
    app.add_systems(OnExit(GameState::Loading), send_loaded_event);

    #[cfg(feature = "inspector")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
}

#[derive(Component)]
struct PlayButton;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct PrimaryColorHue(f32);

#[derive(Component)]
struct Music;

fn pre_load_setup(mut commands: Commands) {
    // spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(WINDOW_WORLD_HEIGHT),
            scale: 1.0,
            ..default()
        }),
        ..default()
    });
}

fn background_scale_from_window_size(width: f32, height: f32) -> Vec3 {
    let aspect_ratio = width / height;
    let margin = 0.01;
    let new_plane_height = WINDOW_WORLD_HEIGHT + margin; // the camera is already scaled to this height
    let new_plane_width = new_plane_height * aspect_ratio + margin;
    Vec3::new(new_plane_height, 1.0, new_plane_width)
}

fn resize_background_plane(
    mut resize_reader: EventReader<WindowResized>,
    mut q: Query<&mut Transform, With<BackgroundPlane>>,
) {
    // update BackgroundPlane scale to fill window
    for event in resize_reader.iter() {
        for mut transform in q.iter_mut() {
            // info!("window resized: {}x{}", event.width, event.height);
            transform.scale = background_scale_from_window_size(event.width, event.height);
        }
    }
}

#[derive(Component)]
struct BackgroundPlane;

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AssetHandle::<DefaultFont, Font>::new(
        asset_server.load("Nunito-Regular.ttf"),
    ));

    commands.insert_resource(AssetHandle::<BackgroundPlane, StandardMaterial>::new(
        standard_materials.add(StandardMaterial {
            base_color: Color::hsl(PRIMARY_COLOR_HUE * 360.0, 0.2, 0.2),
            unlit: true,
            ..default()
        }),
    ));

    commands.insert_resource(AssetHandle::<BackgroundPlane, Mesh>::new(meshes.add(
        Mesh::from(shape::Plane {
            size: 1.0,
            subdivisions: 0,
        }),
    )));

    // pbr precompilation mesh, to avoid lag spike when spawning first pbr object
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 1.0,
            subdivisions: 0,
        })),
        material: standard_materials.add(Color::RED.into()),
        ..default()
    });
}

fn wait_for_loading(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    font_handle: Res<AssetHandle<DefaultFont, Font>>,
    framerate_stable: Res<FramerateIsStable>,
) {
    let mut all_loaded = true;

    let mut handles: Vec<HandleUntyped> = vec![];
    handles.push(font_handle.handle.clone().into());

    for handle in handles.iter() {
        match asset_server.get_load_state(handle) {
            bevy::asset::LoadState::NotLoaded
            | bevy::asset::LoadState::Failed
            | bevy::asset::LoadState::Loading
            | bevy::asset::LoadState::Unloaded => {
                all_loaded = false;
            }
            bevy::asset::LoadState::Loaded => {}
        }
    }

    if all_loaded && framerate_stable.0 {
        info!("All assets loaded, entering menu state");
        next_state.set(GameState::Menu);
    }
}

fn spawn_background(
    mut commands: Commands,
    background_plane_material: Res<AssetHandle<BackgroundPlane, StandardMaterial>>,
    background_plane_mesh: Res<AssetHandle<BackgroundPlane, Mesh>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_window.single();

    let background_tween_scale = Tween::new(
        EaseFunction::ExponentialOut,
        Duration::from_secs_f32(1.7),
        TransformScaleLens {
            start: Vec3::splat(0.0001),
            end: Vec3::splat(1.0),
        },
    );

    let background_tween_rot = Tween::new(
        EaseFunction::ExponentialOut,
        Duration::from_secs_f32(1.1),
        TransformRotationLens {
            start: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, -PI / 2.0),
            end: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
        },
    );

    commands
        .spawn((
            SpatialBundle::default(),
            Animator::new(Tracks::new([background_tween_scale, background_tween_rot])),
        ))
        .with_children(|parent| {
            parent.spawn((
                BackgroundPlane,
                PbrBundle {
                    mesh: background_plane_mesh.handle.clone(),
                    material: background_plane_material.handle.clone(),
                    transform: Transform::from_translation(vec3(0.0, 0.0, -1.0))
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI * 0.5, PI * 0.5))
                        .with_scale(background_scale_from_window_size(
                            window.width(),
                            window.height(),
                        )),
                    ..default()
                },
            ));
        });
}

fn setup(
    mut commands: Commands,
    background_plane_material: Res<AssetHandle<BackgroundPlane, StandardMaterial>>,
    background_plane_mesh: Res<AssetHandle<BackgroundPlane, Mesh>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    info!("setup()");

    // // music
    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("music.ogg"),
    //         settings: PlaybackSettings {
    //             mode: PlaybackMode::Loop,
    //             volume: Volume::Relative(VolumeLevel::new(MENU_MUSIC_VOLUME)),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Music,
    // ));

    // // spawn score text
    // commands.spawn((
    //     ScoreText,
    //     TextBundle::from_section(
    //         format!("Score: 0"),
    //         TextStyle {
    //             font_size: 64.0,
    //             color: Color::WHITE,
    //             ..default()
    //         },
    //     )
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         margin: UiRect::new(Val::Auto, Val::Auto, Val::Vh(20.0), Val::Auto),
    //         display: Display::None,
    //         ..default()
    //     }),
    // ));
}

// fn interact_play_button(
//     mut q_button: Query<(&Interaction, &mut Style), (Changed<Interaction>, With<PlayButton>)>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     if let Some((interaction, mut style)) = q_button.iter_mut().next() {
//         match interaction {
//             Interaction::Pressed => {
//                 style.display = Display::None;
//                 next_state.set(GameState::Playing);
//             }
//             _ => {}
//         };
//     }
// }

// #[derive(Resource)]
// struct Score(usize);

// fn on_enter_menu(mut _commands: Commands, music_controller: Query<&AudioSink, With<Music>>) {
//     // set music volume
//     for sink in music_controller.iter() {
//         sink.set_volume(MENU_MUSIC_VOLUME);
//     }

//     // commands
//     //     .spawn_text_button("Play", PRIMARY_COLOR_HUE)
//     //     .insert(PlayButton);
// }

// fn on_exit_menu() {}

// #[derive(Resource)]
// struct GameTime(Stopwatch);

// fn on_enter_playing(
//     mut commands: Commands,
//     mut score: ResMut<Score>,
//     mut q_score_text: Query<&mut Style, With<ScoreText>>,
//     music_controller: Query<&AudioSink, With<Music>>,
//     // circle_mesh: Res<AssetHandle<Circle, Mesh>>,
//     // circle_mat: Res<AssetHandle<Circle, ColorMaterial>>,
// ) {
//     // reset score
//     score.0 = 0;

//     // hide score text
//     for mut style in q_score_text.iter_mut() {
//         style.display = Display::None;
//     }

//     // start stopwatch
//     commands.insert_resource(GameTime(Stopwatch::new()));

//     // increase music volume
//     for sink in music_controller.iter() {
//         sink.set_volume(PLAYING_MUSIC_VOLUME);
//     }

//     // spawn one circle
//     // commands.spawn((
//     //     MaterialMesh2dBundle {
//     //         mesh: circle_mesh.handle.clone().into(),
//     //         material: circle_mat.handle.clone().into(),
//     //         transform: Transform::from_translation(vec3(0.0, 0.0, 0.0)),
//     //         ..Default::default()
//     //     },
//     //     Circle,
//     //     PickableBundle::default(),
//     //     RaycastPickTarget::default(),
//     //     On::<Pointer<Down>>::run(on_click_circle),
//     // ));
// }

// fn on_exit_playing(
//     mut commands: Commands,
//     mut q_score_text: Query<(&mut Style, &mut Text), With<ScoreText>>,
//     score: Res<Score>,
// ) {
//     // display score text
//     for (mut style, mut text) in q_score_text.iter_mut() {
//         style.display = Display::Flex;
//         for section in text.sections.iter_mut() {
//             section.value = format!("Score: {}", score.0);
//         }
//     }

//     // remove sw
//     commands.remove_resource::<GameTime>();
// }
