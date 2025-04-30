use core::f32;

use super::{Enemy, boss_bit};
use crate::{main_game::enemy::enemy_bullet::enemy_bullet_0, system_resource::OneTexture};
use bevy::{prelude::*, render::render_resource::PreparedBindGroup, transform};
use rand::Rng;
#[derive(Component)]
pub struct BossEnemyAttackThird {
    generated_time: f32,
    prev_shot_time: f32,
}

impl BossEnemyAttackThird {
    pub fn new() -> Self {
        Self {
            generated_time: 0.0,
            prev_shot_time: -1.0,
        }
    }
}

pub fn update_boss_enemy_third(
    mut commands: Commands,
    mut query: Query<(&mut BossEnemyAttackThird, &Enemy, &Transform, Entity)>,
    one_texture: Res<OneTexture>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    for (mut boss, enemy, transform, entity) in query.iter_mut() {
        boss.generated_time += delta;
        boss.prev_shot_time += delta;
        let shot_thr = 1.7;
        if boss.prev_shot_time >= 0.0 {
            let num = 120.0;
            for i in 0..(num as usize) {
                let ang = (f32::consts::PI * 2.0) / num * i as f32;
                let sin = ang.sin();
                let cos = ang.cos();
                let direction = Vec2::new(sin, cos);
                let velocity = 120.0;
                let mut position = transform.translation.xy();
                position.x += direction.x * boss.prev_shot_time;
                position.y += direction.y * boss.prev_shot_time;
                enemy_bullet_0::spawn_enemy_bullet0(
                    commands.reborrow(),
                    transform.translation.xy(),
                    &one_texture,
                    velocity,
                    direction,
                );
            }
            boss.prev_shot_time -= shot_thr;
        }
    }
}
