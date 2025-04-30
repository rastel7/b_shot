use crate::main_game::score::{GameScore, ScoreTipEvent};
use crate::system_consts::SCREEN_VIRTIAL_HALF_SIZE;
use bevy::prelude::*;
use super::super::explosion::ExplosionType;
use super::boss_enemy::BossEnemy;
use super::middle_boss::MiddleBoss;
// すべてのEnemyにつけるコンポーネント
#[derive(Component)]
pub struct Enemy {
    pub hp_index: usize,
    pub initial_hp: Vec<f32>,
    pub hp: Vec<f32>,
    pub is_hited_player: bool,
    pub score: u32,
    pub score_tip_num: usize,
    pub explosition_type:ExplosionType
}
impl Enemy {
    pub fn new(hp: Vec<f32>, score: u32, score_tip_num: usize,explositon_type:ExplosionType) -> Self {
        Self {
            hp_index: 0,
            initial_hp: hp.clone(),
            hp: hp.clone(),
            is_hited_player: false,
            score: score,
            score_tip_num: score_tip_num,
            explosition_type: explositon_type
        }
    }
}
// 画面サイズに加算して消すための大きさを設定
const SCREEN_LIMIT_ADD: f32 = 32.0;
pub fn is_out_of_range_screen_enemy(transform: &Transform) -> bool {
    let position = transform.translation;
    let mut maxsize = SCREEN_VIRTIAL_HALF_SIZE;
    maxsize.0 += SCREEN_LIMIT_ADD;
    maxsize.1 += SCREEN_LIMIT_ADD;
    !(-maxsize.0 <= position.x
        && position.x <= maxsize.0
        && -maxsize.1 <= position.y
        && position.y <= maxsize.1)
}

pub fn if_despawn_destroy_enemy(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform, &Name)>,
    mut game_score: ResMut<GameScore>,
    mut writer: EventWriter<ScoreTipEvent>,
) {
    for enemy in enemy_query.iter_mut() {
        if enemy.1.hp[enemy.1.hp_index] <= 0.0 {
            // スコア加算
            game_score.add_score(enemy.1.score);
            // チップ発生
            writer.send(ScoreTipEvent {
                position: enemy.2.translation.xy(),
                score: 100,
                num: enemy.1.score_tip_num,
            });
            // 正常にプレイヤーの攻撃で敵を倒したとき
            commands.entity(enemy.0).try_despawn_recursive();
        }
        if enemy.1.is_hited_player {
            commands.entity(enemy.0).try_despawn_recursive();
        }
    }
}

pub fn if_despawn_outrange_enemy(
    mut commands: Commands,
    mut enemy_query: Query<
        (Entity, &mut Enemy, &Transform, &Name),
        (Without<MiddleBoss>, Without<BossEnemy>),
    >,
) {
    for enemy in enemy_query.iter_mut() {
        if is_out_of_range_screen_enemy(&enemy.2) {
            info!("Despawn {} {}", enemy.3, enemy.0);
            commands.entity(enemy.0).try_despawn_recursive();
        }
    }
}
