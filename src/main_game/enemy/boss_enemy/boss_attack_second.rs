use core::f32;

use super::{BossEnemyAttackThird, Enemy, boss_bit};
use crate::{main_game::enemy::enemy_bullet::enemy_bullet_0, system_resource::OneTexture};
use bevy::{prelude::*, render::render_resource::PreparedBindGroup};
use rand::Rng;
#[derive(Component)]
pub struct BossEnemyAttackTwo {
    generated_time: f32,
    prev_shot_time: f32,
}

impl BossEnemyAttackTwo {
    pub fn new() -> Self {
        Self {
            generated_time: 0.0,
            prev_shot_time: 0.0,
        }
    }
}

pub fn update_boss_enemy_attack_two(
    mut commands: Commands,
    mut query: Query<(&mut BossEnemyAttackTwo, &Transform, &Enemy, Entity)>,
    time: Res<Time>,
    one_texture: Res<OneTexture>,
) {
    let sin_adds = [
        0.0,
        f32::consts::PI * 0.5,
        f32::consts::PI,
        f32::consts::PI * 1.5,
    ];
    let freq = 1.0;
    let unshot_range = (0.0..0.3);
    for (mut boss, transform, enemy, entity) in query.iter_mut() {
        if enemy.hp_index != 1 {
            commands.entity(entity).remove::<BossEnemyAttackTwo>();
            commands.entity(entity).insert(BossEnemyAttackThird::new());
            continue;
        }
        let delta = time.delta().as_secs_f32();
        boss.generated_time += delta;
        boss.prev_shot_time += delta;
        let modtime = boss.generated_time % freq;
        if unshot_range.contains(&modtime) {
            continue;
        }
        let prev_shot_th = 0.07;
        if boss.prev_shot_time >= 0.0 {
            let mut diff = 0.0;
            if (unshot_range.end <= modtime && modtime <= unshot_range.end + prev_shot_th) {
                info!("{}", boss.generated_time);
                diff = modtime + -unshot_range.end;
                boss.prev_shot_time = diff - prev_shot_th;
            } else {
                diff = boss.prev_shot_time;
                boss.prev_shot_time -= prev_shot_th;
            }
            let velocity = 100.0;
            for add in sin_adds {
                let theta = (boss.generated_time + add);
                let direction = Vec2::new(theta.sin(), theta.cos());
                let mut position = transform.translation.xy();
                position.x += velocity * diff * direction.x;
                position.y += velocity * diff * direction.y;
                enemy_bullet_0::spawn_enemy_bullet0(
                    commands.reborrow(),
                    position,
                    &one_texture,
                    velocity,
                    direction,
                );
            }
            for add in sin_adds {
                let theta = (-boss.generated_time + add + f32::consts::PI * 0.25);
                let direction = Vec2::new(theta.sin(), theta.cos());
                let mut position = transform.translation.xy();
                position.x += velocity * diff * direction.x;
                position.y += velocity * diff * direction.y;
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
}
