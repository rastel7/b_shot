use super::{BossEnemyAttackTwo, Enemy, boss_bit};
use crate::system_resource::OneTexture;
use bevy::prelude::*;
use rand::Rng;
#[derive(Component)]
pub struct BossEnemyAttackOne {
    generated_time: f32,
}

impl BossEnemyAttackOne {
    pub fn new() -> Self {
        Self {
            generated_time: 0.0,
        }
    }
}

pub fn update_boss_attack_one(
    mut commands: Commands,
    mut query: Query<(&mut BossEnemyAttackOne, &Transform, &Enemy, Entity)>,
    one_texture: Res<OneTexture>,
    time: Res<Time>,
) {
    let freq = 0.6;
    let delta_time = time.delta().as_secs_f32();
    let spawn_candidates = [(100.0, -30.0),(80.0, -30.0), (60.0, -35.0), (40.0, -40.0), (20.0, -45.0)];
    for (mut boss_attack, transform, enemy, entity) in query.iter_mut() {
        if enemy.hp_index != 0 {
            commands.entity(entity).remove::<BossEnemyAttackOne>();
            commands.entity(entity).insert(BossEnemyAttackTwo::new());
            continue;
        }
        let need_spawn_bit = (boss_attack.generated_time % freq)
            > ((boss_attack.generated_time + delta_time) % freq);
        boss_attack.generated_time += delta_time;
        if need_spawn_bit {
            let x = if rand::rng().random_bool(0.5) {
                1.0
            } else {
                -1.0
            };
            let id: usize = rand::rng().random::<u32>() as usize % spawn_candidates.len();
            let position: Vec2 = Vec2::new(
                spawn_candidates[id].0 * x + transform.translation.x,
                spawn_candidates[id].1 + transform.translation.y,
            );
            boss_bit::spawn_boss_bit(&mut commands, &one_texture, position);
        }
    }
}
