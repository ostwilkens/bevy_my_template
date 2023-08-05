use bevy::{audio::VolumeLevel, prelude::*};

pub struct MuteButtonPlugin;

impl Plugin for MuteButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_mute_button,))
            .add_systems(Update, (interact_mute_button,));
    }
}

#[derive(Component)]
struct MuteButton;

#[derive(Component)]
struct MuteButtonImage;

fn setup_mute_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                padding: UiRect::all(Val::Px(5.0)),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(MuteButton)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
                        ..default()
                    },
                    image: asset_server.load("volume.png").into(),
                    background_color: Color::WHITE.with_a(0.4).into(),
                    ..default()
                })
                .insert(MuteButtonImage);
        });
}

fn interact_mute_button(
    q_mute_button: Query<&Interaction, (Changed<Interaction>, With<MuteButton>)>,
    mut q_mute_button_image: Query<&mut BackgroundColor, With<MuteButtonImage>>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    if let Some(interaction) = q_mute_button.iter().next() {
        if let Ok(mut background_color) = q_mute_button_image.get_single_mut() {
            let is_muted = global_volume.volume.get() == 0.0;

            match interaction {
                Interaction::Pressed => {
                    if is_muted {
                        global_volume.volume = VolumeLevel::new(1.0);
                        background_color.0 = Color::WHITE.with_a(0.6);
                    } else {
                        global_volume.volume = VolumeLevel::new(0.0);
                        background_color.0 = Color::WHITE.with_a(0.3);
                    }
                }
                _ => {}
            }
        }
    }
}
