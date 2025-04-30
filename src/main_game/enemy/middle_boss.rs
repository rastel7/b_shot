use super::enemy_bullet::enemy_bullet_0;
use super::{enemy::Enemy, enemy_0, enemy_reader};
use crate::explosion::ExplosionType;
use crate::main_game::collision::*;
use crate::system_consts;
use crate::system_resource::{OneTexture, Vec2toVec3};
use bevy::ecs::query;
use bevy::text::cosmic_text::ttf_parser::Tag;
use bevy::{prelude::*, state::commands, *};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};
use std::cmp::max;
use std::collections::btree_map::Range;
use std::f32;
use std::ops::RangeBounds;
const MIDDLEBOSSRADIUS: f32 = 32.0;
const SHOT_GENERATE_POINT_DIFF: [(f32, f32); 4] =
    [(-35.0, -20.0), (-20.0, -30.0), (20.0, -30.0), (35.0, -20.0)];
const ZAKO_SPAWN_LIMIT: usize = 5;
#[derive(Component, Default)]
pub struct MiddleBoss {
    generated_time: f32,
}
#[derive(Debug)]
pub struct MiddleBossSpawner {
    initial_position: Vec2,
    target_position: Vec2,
}
#[derive(Component)]
pub struct MiddleBossMover {
    velocity: f32,
    acceleration: f32,
    directon: Vec2,
    target_position: Vec2,
    move_type: MiddleBossMoveType,
    sin_passed_time: f32,
    prev_dir: f32,
    zako_spawn_count: usize,
}
#[derive(Component)]
pub struct MiddleBossBulletManager {
    generated_time: f32,
    after_shot_time: [f32; 4],
    shoting: [bool; 4],
}
impl Default for MiddleBossBulletManager {
    fn default() -> Self {
        Self {
            generated_time: 0.0,
            after_shot_time: [1_000_000.0, 1_000_000.0, 1_000_000.0, 1_000_000.0],
            shoting: [false, false, false, false],
        }
    }
}

enum MiddleBossMoveType {
    Initial,
    Sin,
    Terminate,
}

impl MiddleBossMover {
    pub fn to_target_position(
        &mut self,
        mut commands: Commands,
        entity: Entity,
        mut transform: Mut<'_, Transform>,
        time: &Res<Time>,
    ) {
        let delta = time.delta().as_secs_f32();
        let max_velocity = 100.0;
        if transform.translation.xy() == self.target_position {
            self.move_type = MiddleBossMoveType::Sin;
            commands
                .entity(entity)
                .insert((MiddleBossBulletManager { ..default() }));
            return;
        }
        self.velocity = self.velocity + self.acceleration * delta;
        if self.velocity > max_velocity {
            self.velocity = max_velocity;
        }
        let prev_position = transform.translation.xy();
        transform.translation.x += self.velocity * self.directon.x * delta;
        transform.translation.y += self.velocity * self.directon.y * delta;
        let new_position = transform.translation.xy();
        let target_position = self.target_position;
        if (target_position - prev_position).dot(target_position - new_position) < 0.0 {
            transform.translation.x = self.target_position.x;
            transform.translation.y = self.target_position.y;
        }
    }
    pub fn move_sin(
        &mut self,
        commands: &mut Commands,
        mut transform: Mut<'_, Transform>,
        time: &Res<Time>,
        one_texture: &Res<OneTexture>,
    ) {
        let per = 0.8;
        let width = 100.0;
        let prev_sin = (self.sin_passed_time * per).sin();
        self.sin_passed_time += time.delta().as_secs_f32();

        let sin = (self.sin_passed_time * per).sin();

        let add_x = sin * width;
        let dir = ((add_x + self.target_position.x) - transform.translation.x);
        transform.translation.x = add_x + self.target_position.x;
        if dir * self.prev_dir < -0.0 && self.zako_spawn_count < ZAKO_SPAWN_LIMIT {
            let posx = [50.0, 70.0, 90.0];
            for x in posx {
                let nx = x * if transform.translation.x < 0.0 {
                    1.0
                } else {
                    -1.0
                };
                enemy_0::spawn_enemy_0(
                    commands,
                    one_texture,
                    -Vec2::Y,
                    100.0,
                    Vec2::new(nx, system_consts::SCREEN_VIRTIAL_HALF_SIZE.1),
                );
            }
            self.zako_spawn_count += 1;
        }

        self.prev_dir = dir;
    }
}
impl MiddleBossSpawner {
    pub fn new(strs: &Vec<&str>) -> Self {
        Self {
            initial_position: Vec2::new(
                strs[2].parse::<f32>().unwrap(),
                strs[3].parse::<f32>().unwrap(),
            ),
            target_position: Vec2::new(
                strs[4].parse::<f32>().unwrap(),
                strs[5].parse::<f32>().unwrap(),
            ),
        }
    }

    pub fn spawn_enemy(&self, command: &mut Commands, one_texture: &Res<OneTexture>) -> Entity {
        let entity = spawn_middle_boss(
            command,
            one_texture,
            self.initial_position,
            self.target_position,
        );
        entity
    }
}

pub fn spawn_middle_boss(
    commands: &mut Commands,
    one_texture: &Res<OneTexture>,
    initial_position: Vec2,
    target_position: Vec2,
) -> Entity {
    let transform =
        Transform::from_translation(Vec3::new(initial_position.x, initial_position.y, -3.0));
    let mut enemy = Enemy::new(vec![4000.0], 10000, 30, ExplosionType::BossEnemy);
    enemy.is_destroy_touch_player = false;
    let entity = commands
        .spawn((
            Name::new("MiddleBoss"),
            MiddleBoss {
                generated_time: 0.0,
            },
            MiddleBossMover {
                velocity: 30.0,
                acceleration: 50.0,
                directon: (target_position - initial_position).normalize_or(Vec2::Y),
                target_position: target_position,
                move_type: MiddleBossMoveType::Initial,
                sin_passed_time: 0.0,
                prev_dir: 0.0,
                zako_spawn_count: 0,
            },
            enemy_reader::EnemyGenerateTimeStopper,
            AseSpriteSlice {
                name: "MiddleBoss".into(),
                aseprite: (*one_texture).clone_weak(),
                ..default()
            },
            transform,
            enemy,
            StateScoped(crate::GameState::InGame),
        ))
        .id();
    add_collision(
        commands,
        entity,
        MIDDLEBOSSRADIUS,
        Color::srgb(0.5, 0.0, 0.0),
    );
    entity
}

pub fn update_middleboss_transform(
    mut commands: Commands,
    mut query: Query<(&mut MiddleBossMover, &mut Transform, Entity)>,
    time: Res<Time>,
    one_texture: Res<OneTexture>,
) {
    for (mut middleboss_mover, mut transform, entity) in query.iter_mut() {
        match middleboss_mover.move_type {
            MiddleBossMoveType::Initial => {
                middleboss_mover.to_target_position(commands.reborrow(), entity, transform, &time);
            }
            MiddleBossMoveType::Sin => {
                middleboss_mover.move_sin(&mut commands, transform, &time, &one_texture);
            }
            MiddleBossMoveType::Terminate => {}
        };
    }
}

pub fn update_middleboss_bullet_manager(
    mut commands: Commands,
    mut query: Query<(&mut MiddleBossBulletManager, &Transform)>,
    one_texture: Res<OneTexture>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    let thr: f32 = 2.0;
    let shot_freq: f32 = 0.1;
    let shots_range: [(f32, f32); 4] = [(0.0, 0.7), (1.0, 1.7), (1.5, 0.2), (0.5, 1.2)];

    let shot_velocity: f32 = 130.0;
    let shots_direction_1: [(f32, f32); 4] = [(0.6, -1.0), (0.9, -1.0), (-0.9, -1.0), (-0.6, -1.0)];
    let shots_direction_2: [(f32, f32); 4] = [(0.9, -1.0), (0.6, -1.0), (-0.6, -1.0), (-0.9, -1.0)];
    for (mut bullet_manager, transform) in query.iter_mut() {
        bullet_manager.generated_time += delta;
        let modtime = bullet_manager.generated_time % thr;

        for i in 0..shots_range.len() {
            let mut need_shot: bool = if shots_range[i].0 < shots_range[i].1 {
                shots_range[i].0 <= modtime && modtime <= shots_range[i].1
            } else {
                shots_range[i].0 <= modtime || modtime <= shots_range[i].1
            };
            if !need_shot {
                bullet_manager.shoting[i] = false;
                continue;
            }

            let mut need_shot: bool = false;
            let mut add_time: f32 = 0.0;
            if !bullet_manager.shoting[i] {
                // 今回のthr内では初めての撃つタイミングなので必須
                need_shot = true;
                add_time = modtime - shots_range[i].0;
            } else {
                if modtime < bullet_manager.after_shot_time[i] {
                    let passedtime = modtime + (thr - bullet_manager.after_shot_time[i]);
                    if passedtime > shot_freq {
                        need_shot = true;
                        add_time =
                            modtime - (bullet_manager.after_shot_time[i] + shot_freq + thr) % thr;
                    }
                } else {
                    if modtime - bullet_manager.after_shot_time[i] > shot_freq {
                        need_shot = true;
                        add_time = modtime - (bullet_manager.after_shot_time[i] + shot_freq);
                    }
                }
            }
            if need_shot {
                bullet_manager.shoting[i] = true;

                bullet_manager.after_shot_time[i] = modtime - add_time;
                let mut shot_generate_position = transform.translation.xy();
                shot_generate_position.x += SHOT_GENERATE_POINT_DIFF[i].0;
                shot_generate_position.y += SHOT_GENERATE_POINT_DIFF[i].1;
                let direction = Vec2::new(shots_direction_1[i].0, shots_direction_1[i].1);
                shot_generate_position += direction * shot_velocity * add_time;
                enemy_bullet_0::spawn_enemy_bullet0(
                    commands.reborrow(),
                    shot_generate_position,
                    &one_texture,
                    shot_velocity,
                    direction,
                );

                shot_generate_position = transform.translation.xy();
                shot_generate_position.x += SHOT_GENERATE_POINT_DIFF[i].0;
                shot_generate_position.y += SHOT_GENERATE_POINT_DIFF[i].1;
                let direction = Vec2::new(shots_direction_2[i].0, shots_direction_2[i].1);
                shot_generate_position += direction * shot_velocity * add_time;
                enemy_bullet_0::spawn_enemy_bullet0(
                    commands.reborrow(),
                    shot_generate_position,
                    &one_texture,
                    shot_velocity,
                    direction,
                );
            }
        }
    }
}
