use super::super::explosion::ExplosionType;
use super::enemy::Enemy;
use super::enemy_bullet::enemy_bullet_0;
use crate::game_state::GameState;
use crate::main_game::{
    self,
    collision::{self, *},
};
use crate::system_resource::{OneTexture, Vec2toVec3};
use bevy::{prelude::*, state::commands, *};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};

#[derive(Component)]
pub struct BossBit {
    generated_time: f32,
    initial_position: Vec2,
    shoted_time: f32,
}

impl BossBit {
    pub fn new(initial_position: Vec2) -> Self {
        Self {
            generated_time: 0.0,
            initial_position: initial_position,
            shoted_time: 0.0,
        }
    }
}
pub fn spawn_boss_bit(commands: &mut Commands, one_texture: &Res<OneTexture>, position: Vec2) {
    let entity = commands
        .spawn((
            Name::new("BossBit"),
            Transform::from_xyz(position.x, position.y, -4.0),
            BossBit::new(position),
            Enemy::new(vec![300.0], 0, 0, ExplosionType::Enemy),
            AseSpriteSlice {
                name: "BossBit".into(),
                aseprite: (*one_texture).clone_weak(),
            },
            StateScoped(GameState::InGame),
        ))
        .id();

    collision::add_collision(commands, entity, 12.0, Color::srgb(0.5, 0.0, 0.0));
}

pub fn update_boss_bit(
    mut commands: Commands,
    mut query: Query<(&mut BossBit, &mut Transform)>,
    one_texture: Res<OneTexture>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    let velocity = 50.0;
    let width = 30.0;
    let sin_mul = 3.0;
    let shot_thr = 0.2;
    for (mut bit, mut transform) in query.iter_mut() {
        bit.generated_time += delta;
        transform.translation.y -= velocity * delta;
        let sin = (bit.generated_time * sin_mul).sin();
        transform.translation.x = sin * width + bit.initial_position.x;

        bit.shoted_time += delta;
        if bit.shoted_time > 0.0 {
            bit.shoted_time -= shot_thr;
            let velocity = 100.0;
            let direction = -Vec2::Y;
            let mut position = transform.translation.xy();
            if bit.shoted_time > 0.0 {
                position += velocity * direction * bit.shoted_time;
            }
            enemy_bullet_0::spawn_enemy_bullet0(
                commands.reborrow(),
                position,
                &one_texture,
                velocity,
                direction,
            );
        }
    }
}
