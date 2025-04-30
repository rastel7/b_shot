use crate::{game_state, system_resource::*};
use bevy::{math::VectorSpace, prelude::*, state::commands};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};

#[derive(Component)]
pub struct Title;

pub fn setup_title(mut commands: Commands, system_texture: Res<SystemsTexture>) {
    commands.spawn((
        Name::new("Logo"),
        AseSpriteSlice {
            name: "TitleLogo".into(),
            aseprite: (system_texture.clone()),
            ..default()
        },
        Transform::from_translation(Vec3::new(0., 50., 0.)),
        StateScoped(game_state::GameState::TitleMenu),
    ));
    commands.spawn((
        Name::new("PressZToStart"),
        StateScoped(game_state::GameState::TitleMenu),
        AseSpriteSlice {
            name: "PressZToStart".into(),
            aseprite: (system_texture.clone()),
            ..default()
        },
        Transform::from_translation(Vec3::new(0., -30., 0.)),
    ));
    commands.spawn((
        Name::new("PressEscToExit"),
        StateScoped(game_state::GameState::TitleMenu),
        AseSpriteSlice {
            name: "PressEscToExit".into(),
            aseprite: (system_texture.clone()),
            ..default()
        },
        Transform::from_translation(Vec3::new(0., -90., 0.)),
    ));
}

pub fn title_key_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<game_state::GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::KeyZ) {
        app_state.set(game_state::GameState::InGame);
    }
}