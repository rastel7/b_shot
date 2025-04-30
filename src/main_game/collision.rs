use std::panic;

use super::enemy::boss_enemy::BossEnemy;
use super::enemy::enemy::Enemy;
use super::enemy::enemy_bullet::{enemy_bullet::EnemyBullet, enemy_bullet_0::EnemyBullet0};
use super::explosion::Explosion;
use super::explosion::ExplosionEvent;
use super::score::GameScore;
use super::score::ScoreTip;
use super::score::ScoreTipEvent;
use super::{game_clear, life};
use crate::bgm::BossBGM;
use crate::enemy::*;
use crate::player::*;
use crate::se::PlaySEEvent;
use bevy::render::render_resource::encase::private::ReadFrom;
use bevy::{prelude::*, state::commands, utils::tracing::Instrument, *};
#[derive(Component, Debug)]
pub struct Collision {
    radius: f32,
    pub invisible: bool,
}

impl Collision {
    fn new(radius: f32) -> Self {
        Self {
            radius: radius,
            invisible: false,
        }
    }
    pub fn is_hit(
        lhs_size: &Collision,
        rhs_size: &Collision,
        lhs_position: &Vec3,
        rhs_position: &Vec3,
        available_invisible: bool,
    ) -> bool {
        if available_invisible {
            if lhs_size.invisible || rhs_size.invisible {
                return false;
            }
        }

        let lhs_position = lhs_position.xy();
        let rhs_position = rhs_position.xy();
        let distance = lhs_position.distance(rhs_position);
        distance < lhs_size.radius + rhs_size.radius
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

pub fn add_collision(commands: &mut Commands, entity: Entity, radius: f32, color: Color) {
    commands.entity(entity).insert(Collision::new(radius));
    // 以下デバッグ用表示
    return;
    let mut color = color;
    color.set_alpha(0.6);
    let child = commands
        .entity(entity)
        .with_child((
            Name::new("Collision"),
            Sprite {
                color: color,
                ..Default::default()
            },
            Transform::from_scale(Vec3::new(radius * 2.0, radius * 2.0, radius * 2.0))
                .with_translation(Vec3::new(0.0, 0.0, 20.0)),
        ))
        .id();
}

pub fn solve_player_bullet_collision(
    mut commands: Commands,
    mut player_bullet_query: Query<
        (
            Entity,
            &mut PlayerShot,
            &mut Transform,
            &Collision,
            &Visibility,
        ),
        (Without<Player>, Without<Enemy>),
    >,
    mut explosion_writer: EventWriter<ExplosionEvent>,
    mut enemy_query: Query<
        (Entity, &mut Enemy, &mut Transform, &Collision, &Visibility),
        (Without<Player>, Without<PlayerShot>),
    >,
    mut hp_gauge_query: Query<(&mut super::enemy_hp_gauge::EnemyHPGauge)>,
    mut writer: EventWriter<ScoreTipEvent>,
    mut se_writer: EventWriter<crate::se::PlaySEEvent>,
) {
    let mut gauge = hp_gauge_query.get_single_mut();
    for mut player_bullet in player_bullet_query.iter_mut() {
        if player_bullet.4 == Visibility::Hidden || player_bullet.1.is_hit {
            continue;
        }

        for mut enemy in enemy_query.iter_mut() {
            if enemy.4 == Visibility::Hidden || enemy.1.hp[enemy.1.hp_index] <= 0.0 {
                continue;
            }
            let is_hit = Collision::is_hit(
                player_bullet.3,
                enemy.3,
                &player_bullet.2.translation,
                &enemy.2.translation,
                false,
            );
            if is_hit {
                se_writer.send(PlaySEEvent(crate::se::SEType::HitPlayerShot));
                let index = enemy.1.hp_index.clone();
                enemy.1.hp[index] -= player_bullet.1.damage;
                player_bullet.1.is_hit = true;

                if enemy.1.hp[index] <= 0.0 {
                    enemy.1.hp[index] = 0.0;
                    if index + 1 == enemy.1.hp.len() {
                        explosion_writer.send(ExplosionEvent {
                            position: enemy.2.translation.xy(),
                            explosion_type: enemy.1.explosition_type,
                        });
                    } else {
                        enemy.1.hp_index += 1;
                        // チップ発生
                        writer.send(ScoreTipEvent {
                            position: enemy.2.translation.xy(),
                            score: 100,
                            num: (enemy.1.score_tip_num as f32 * 0.5).ceil() as usize,
                        });
                    }
                }
                if (gauge).is_ok() {
                    gauge.as_deref_mut().unwrap().hp_rate =
                        enemy.1.hp[index] / enemy.1.initial_hp[index];
                    gauge.as_deref_mut().unwrap().remain_index =
                        (enemy.1.hp.len() - 1 - enemy.1.hp_index);
                }
            }
        }
    }
}

pub fn solve_player_dyson_collision(
    mut commands: Commands,
    mut player_dyson_query: Query<
        (
            Entity,
            &mut PlayerDysonAttack,
            &mut Transform,
            &Collision,
            &Visibility,
        ),
        (Without<Player>, Without<Enemy>, Without<PlayerShot>),
    >,
    mut explosion_writer: EventWriter<ExplosionEvent>,
    mut enemy_query: Query<
        (Entity, &mut Enemy, &mut Transform, &Collision, &Visibility),
        (
            Without<Player>,
            Without<PlayerShot>,
            Without<PlayerDysonAttack>,
        ),
    >,
    time: Res<Time>,
    mut writer: EventWriter<ScoreTipEvent>,
    mut hp_gauge_query: Query<(&mut super::enemy_hp_gauge::EnemyHPGauge)>,
) {
    let mut gauge = hp_gauge_query.get_single_mut();
    let dyson_attack = player_dyson_query.get_single_mut();
    if dyson_attack.is_err() {
        return;
    }
    let dyson_attack = dyson_attack.unwrap();
    if dyson_attack.4 == Visibility::Hidden {
        return;
    }

    let delta = time.delta().as_secs_f32();
    for mut enemy in enemy_query.iter_mut() {
        if enemy.4 == Visibility::Hidden || enemy.1.hp[enemy.1.hp_index] <= 0.0 {
            continue;
        }
        let is_hit = Collision::is_hit(
            dyson_attack.3,
            enemy.3,
            &dyson_attack.2.translation,
            &enemy.2.translation,
            true,
        );
        if is_hit {
            let index = enemy.1.hp_index;
            enemy.1.hp[index] -= dyson_attack.1.damage_per_seconds() * delta;
            if enemy.1.hp[index] <= 0.0 {
                enemy.1.hp[index] = 0.0;
                if index + 1 == enemy.1.hp.len() {
                    explosion_writer.send(ExplosionEvent {
                        position: enemy.2.translation.xy(),
                        explosion_type: enemy.1.explosition_type,
                    });
                } else {
                    enemy.1.hp_index += 1;
                    // チップ発生
                    writer.send(ScoreTipEvent {
                        position: enemy.2.translation.xy(),
                        score: 100,
                        num: (enemy.1.score_tip_num as f32 * 0.5).ceil() as usize,
                    });
                }
            }
            if (gauge).is_ok() {
                gauge.as_deref_mut().unwrap().hp_rate =
                    enemy.1.hp[index] / enemy.1.initial_hp[index];
                gauge.as_deref_mut().unwrap().remain_index =
                    (enemy.1.hp.len() - 1 - enemy.1.hp_index);
            }
        }
    }
}

pub fn solve_enemy_to_player_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player, &mut Transform, &mut Collision), Without<Enemy>>,
    mut enemy__query: Query<(Entity, &mut Enemy, &mut Transform, &Collision), (Without<Player>)>,
    mut explosion_writer: EventWriter<ExplosionEvent>,
    mut life: ResMut<super::life::Life>,
) {
    let player = player_query.get_single_mut();
    if player.is_err() {
        info!("Unfind Player");
        return;
    }
    let mut explosion_writer = explosion_writer;
    let mut player = player.unwrap();
    for mut enemy in enemy__query.iter_mut() {
        let is_hit = Collision::is_hit(
            player.3.as_ref(),
            enemy.3,
            &player.2.translation,
            &enemy.2.translation,
            true,
        );
        if is_hit {
            if enemy.1.is_destroy_touch_player {
                enemy.1.is_hited_player = true;
            }
            let player_collision = player.3.as_mut();
            player.1.add_damage(
                commands.reborrow(),
                player.0,
                player_collision,
                &mut explosion_writer,
                player.2.translation.xy(),
                &mut life,
            );
        }
    }
}

pub fn solve_enemy_bullet0_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player, &mut Transform, &mut Collision)>,
    mut enemy_bullet_0_query: Query<
        (
            Entity,
            &mut EnemyBullet,
            &mut EnemyBullet0,
            &mut Transform,
            &Collision,
        ),
        (Without<Player>,),
    >,
    explosion_writer: EventWriter<ExplosionEvent>,
    mut game_score: ResMut<GameScore>,
    mut life: ResMut<super::life::Life>,
) {
    let player = player_query.get_single_mut();
    if player.is_err() {
        info!("Unfind Player");
        return;
    }
    let mut player = player.unwrap();

    let mut explosion_writer = explosion_writer;
    for mut enemy_bullet_0 in enemy_bullet_0_query.iter_mut() {
        let is_hit = Collision::is_hit(
            player.3.as_ref(),
            enemy_bullet_0.4,
            &player.2.translation,
            &enemy_bullet_0.3.translation,
            true,
        );
        if is_hit {
            enemy_bullet_0.1.need_despawn = true;
            let player_collision = player.3.as_mut();
            player.1.add_damage(
                commands.reborrow(),
                player.0,
                player_collision,
                &mut explosion_writer,
                player.2.translation.xy(),
                &mut life,
            );
        }
    }
}

pub fn solve_score_tip_collision(
    mut commands: Commands,
    mut tip_query: Query<(&mut ScoreTip, &Transform, &Collision, Entity), Without<Player>>,
    player_query: Query<(&Player, &Transform, &Collision), Without<ScoreTip>>,
    mut game_score: ResMut<GameScore>,
    mut increment_life_writer: EventWriter<life::IncrementLife>,
    mut se_writer: EventWriter<crate::se::PlaySEEvent>,
) {
    let player = player_query.get_single();
    if player.is_err() {
        return;
    }
    let player = player.unwrap();

    for (mut tip, transorm, colision, entity) in tip_query.iter_mut() {
        if Collision::is_hit(
            colision,
            player.2,
            &transorm.translation,
            &player.1.translation,
            false,
        ) {
            game_score.add_score(tip.score, &mut increment_life_writer);
            se_writer.send(PlaySEEvent(crate::se::SEType::PickTip));
            tip.is_need_despawn = true;
        }
    }
}

pub fn is_need_gameclear_entity(
    mut commands: Commands,
    query: Query<&Enemy, With<BossEnemy>>,
    boss_bgm_query: Query<(Entity), With<BossBGM>>,
    mut bullet_query: Query<&mut Collision, With<EnemyBullet>>,
) {
    for boss in query.iter() {
        if boss.hp_index + 1 == boss.hp.len() && boss.hp[boss.hp_index] <= 0.0 {
            game_clear::spawn_game_clear_entity(commands.reborrow());
            for mut col in bullet_query.iter_mut() {
                col.invisible = true;
            }
        }
    }
}
