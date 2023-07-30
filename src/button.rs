use bevy::{prelude::*, ecs::system::EntityCommands};

use crate::PrimaryColorHue;

fn button_background_color(hue: f32) -> Color {
    Color::hsl(hue * 360.0, 0.5, 0.4)
}

fn button_hover_color(hue: f32) -> Color {
    Color::hsl(hue * 360.0, 0.5, 0.45)
}

pub fn interact_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    primary_color_hue: Res<PrimaryColorHue>,
) {
    if let Some((interaction, mut background_color)) = q_button.iter_mut().next() {
        match interaction {
            Interaction::Hovered => {
                background_color.0 = button_hover_color(primary_color_hue.0);
            }
            Interaction::None => {
                background_color.0 = button_background_color(primary_color_hue.0);
            }
            _ => {}
        }
    }
}

pub trait ButtonCommands<'w, 's> {
    fn spawn_text_button<'a>(&'a mut self, text: &str, hue: f32) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> ButtonCommands<'w, 's> for Commands<'w, 's> {
    fn spawn_text_button<'a>(&'a mut self, text: &str, hue: f32) -> EntityCommands<'w, 's, 'a> {
        let mut e = self.spawn_empty();

        e.insert(ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                border: UiRect {
                    left: Val::Px(1.0),
                    right: Val::Px(1.0),
                    top: Val::Px(1.0),
                    bottom: Val::Px(4.0),
                },
                ..default()
            },
            border_color: Color::BLACK.with_a(0.5).into(),
            background_color: button_background_color(hue).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect::top(Val::Px(4.0)),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font_size: 36.0,
                        color: Color::BLACK.with_a(0.5),
                        ..default()
                    },
                ),
                ..default()
            });
            parent.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font_size: 36.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..default()
            });
        });
        e
    }
}