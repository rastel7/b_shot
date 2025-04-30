use super::super::explosion::ExplosionType;
use super::boss_enemy::BossEnemy;
use super::middle_boss::MiddleBoss;
use crate::main_game::score::{GameScore, ScoreTipEvent};
use crate::system_consts::SCREEN_VIRTIAL_HALF_SIZE;
use bevy::prelude::*;
// すべてのEnemyにつけるコンポーネント
#[derive(Component)]
pub struct Enemy {
    pub hp_index: usize,
    pub initial_hp: Vec<f32>,
    pub hp: Vec<f32>,
    pub is_hited_player: bool,
    pub score: u32,
    pub score_tip_num: usize,
    pub explosition_type: ExplosionType,
    pub is_destroy_touch_player: bool,
    pub is_out_of_screen: bool,
}
impl Enemy {
    pub fn new(
        hp: Vec<f32>,
        score: u32,
        score_tip_num: usize,
        explositon_type: ExplosionType,
    ) -> Self {
        Self {
            hp_index: 0,
            initial_hp: hp.clone(),
            hp: hp.clone(),
            is_hited_player: false,
            score: score,
            score_tip_num: score_tip_num,
            explosition_type: explositon_type,
            is_destroy_touch_player: true,
            is_out_of_screen: false,
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

    mut increment_life_writer: EventWriter<crate::life::IncrementLife>,
) {
    for enemy in enemy_query.iter_mut() {
        // 画面外判定
        let mut need_despawn = false;
        if enemy.1.is_out_of_screen {
            need_despawn = true;
        } else if enemy.1.hp[enemy.1.hp_index] <= 0.0 {
            // スコア加算
            game_score.add_score(enemy.1.score, &mut increment_life_writer);
            // チップ発生
            writer.send(ScoreTipEvent {
                position: enemy.2.translation.xy(),
                score: 300,
                num: enemy.1.score_tip_num,
            });
            need_despawn = true;
        } else if enemy.1.is_hited_player {
            need_despawn = true;
        }
        if need_despawn {
            info!("Despawn {} {}", enemy.3, enemy.0);
            commands.entity(enemy.0).despawn_recursive();
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
    for mut enemy in enemy_query.iter_mut() {
        if is_out_of_range_screen_enemy(&enemy.2) {
            enemy.1.is_out_of_screen = true;
        }
    }
}
