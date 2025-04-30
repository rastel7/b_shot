use bevy::{
    math::{VectorSpace, vec3},
    prelude::*,
    state::commands,
};

use crate::{game_state, system_consts};

#[derive(Component)]
pub struct StartEffect {
    passed_time: f32,
    erase_seconds: f32,
    easing: EasingCurve<f32>,
}

const INITIAL_HEIGHT: f32 = system_consts::SCREEN_VIRTIAL_HALF_SIZE.1 * 2.1;
pub fn spawn_gamestart_effect(mut commands: Commands) {
    commands.spawn((
        Name::new("StartEffect"),
        StartEffect {
            passed_time: 0.0,
            erase_seconds: 0.5,
            easing: EasingCurve::new(0.0, 1.00, EaseFunction::QuinticOut),
        },
        StateScoped(game_state::GameState::InGame),
        Sprite { ..default() },
        Transform::from_scale(Vec3::new(1000.0, INITIAL_HEIGHT, 1.0))
            .with_translation(Vec3::new(0.0, 0.0, 100.0)),
    ));
}

pub fn update_gamestart_effect(
    mut commands: Commands,
    mut start_effect_query: Query<(Entity, &mut StartEffect, &mut Transform)>,
    time: Res<Time>,
) {
    for mut start_effect in start_effect_query.iter_mut() {
        start_effect.1.passed_time += time.delta().as_secs_f32();
        let per = start_effect.1.easing.sample(start_effect.1.passed_time);
        if per.is_none() {
            commands.entity(start_effect.0).despawn();
            continue;
        }
        let per = per.unwrap();
        start_effect.2.scale.y = INITIAL_HEIGHT * (1.0 - per);
    }
}
