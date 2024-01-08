use bevy::{audio::VolumeLevel, prelude::*};

pub struct MuteButtonPlugin;

impl Plugin for MuteButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_mute_button,))
            .add_systems(Update, (interact_mute_button,))
            .init_resource::<Muted>();
    }
}

#[derive(Component)]
struct MuteButton;

#[derive(Component)]
struct MuteButtonImage;

#[derive(Resource, Default)]
pub struct Muted(pub bool);

static MUTE_ICON_ACTIVE_OPACITY: f32 = 0.7;
static MUTE_ICON_INACTIVE_OPACITY: f32 = 0.2;

fn setup_mute_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                right: Val::Px(8.0),
                top: Val::Px(10.0),
                padding: UiRect::all(Val::Px(7.0)),
                position_type: PositionType::Absolute,
                border: UiRect {
                    left: Val::Px(1.0),
                    right: Val::Px(1.0),
                    top: Val::Px(1.0),
                    bottom: Val::Px(4.0),
                },
                ..default()
            },
            border_color: Color::BLACK.with_a(0.5).into(),
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
                    background_color: Color::WHITE.with_a(MUTE_ICON_ACTIVE_OPACITY).into(),
                    ..default()
                })
                .insert(MuteButtonImage);
        });
}

fn interact_mute_button(
    q_mute_button: Query<&Interaction, (Changed<Interaction>, With<MuteButton>)>,
    mut q_mute_button_image: Query<&mut BackgroundColor, With<MuteButtonImage>>,
    mut global_volume: ResMut<GlobalVolume>,
    mut muted: ResMut<Muted>,
) {
    if let Some(interaction) = q_mute_button.iter().next() {
        if let Ok(mut background_color) = q_mute_button_image.get_single_mut() {
            let is_muted = global_volume.volume.get() == 0.0;

            match interaction {
                Interaction::Pressed => {
                    if is_muted {
                        global_volume.volume = VolumeLevel::new(1.0);
                        background_color.0 = Color::WHITE.with_a(MUTE_ICON_ACTIVE_OPACITY);
                        muted.0 = false;
                    } else {
                        global_volume.volume = VolumeLevel::new(0.0);
                        background_color.0 = Color::WHITE.with_a(MUTE_ICON_INACTIVE_OPACITY);
                        muted.0 = true;
                    }
                }
                _ => {}
            }
        }
    }
}
