use bevy::{prelude::*, state::commands, *};

use crate::main_game::collision::Collision;
#[derive(Component)]
pub struct EnemyBullet {
    pub need_despawn: bool,
    initial_velocity: f32,
    pub velocity: f32,
}
impl EnemyBullet {
    pub fn new(need_despawn: bool, velocity: f32) -> Self {
        Self {
            need_despawn: need_despawn,
            initial_velocity: velocity,
            velocity: velocity,
        }
    }
    pub fn get_initial_velocity(&self) -> f32 {
        self.initial_velocity
    }
}

pub fn if_despawn_enemy_bullet(
    mut commands: Commands,
    mut enemy_bullet_query: Query<(Entity, &mut EnemyBullet, &Collision, &Transform)>,
) {
    for enemy_bullet in enemy_bullet_query.iter_mut() {
        if enemy_bullet.1.need_despawn {
            commands.entity(enemy_bullet.0).despawn_recursive();
        }
        if is_out_of_range_screen_bullet(enemy_bullet.3, enemy_bullet.2.radius()) {
            commands.entity(enemy_bullet.0).despawn_recursive();
        }
    }
}

const SCREEN_LIMIT_ADD: f32 = 16.0;
fn is_out_of_range_screen_bullet(transform: &Transform, collision_radius: f32) -> bool {
    let position = transform.translation;
    let mut maxsize = crate::system_consts::SCREEN_VIRTIAL_HALF_SIZE;
    maxsize.0 += SCREEN_LIMIT_ADD + collision_radius;
    maxsize.1 += SCREEN_LIMIT_ADD + collision_radius;
    !(-maxsize.0 <= position.x
        && position.x <= maxsize.0
        && -maxsize.1 <= position.y
        && position.y <= maxsize.1)
}
