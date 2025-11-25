use bevy::prelude::*;

use crate::game::GameState;

#[derive(Component)]
struct StatusText;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(OnEnter(GameState::InGame), show_running)
            .add_systems(OnEnter(GameState::Paused), show_paused);
    }
}

fn spawn_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(16.0),
                left: Val::Px(16.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Move with WASD or the arrow keys. Press Esc to pause.",
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Status: Running",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(0.8, 0.9, 1.0),
                        ..default()
                    },
                ),
                StatusText,
            ));
        });
}

fn show_running(mut text: Query<&mut Text, With<StatusText>>) {
    if let Ok(mut status) = text.get_single_mut() {
        if let Some(section) = status.sections.first_mut() {
            section.value = "Status: Running".to_string();
        }
    }
}

fn show_paused(mut text: Query<&mut Text, With<StatusText>>) {
    if let Ok(mut status) = text.get_single_mut() {
        if let Some(section) = status.sections.first_mut() {
            section.value = "Status: Paused".to_string();
        }
    }
}
