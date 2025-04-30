use super::{enemy::Enemy, enemy_bullet};
use crate::enemy_bullet_0::spawn_enemy_bullet0;
use crate::main_game::collision::*;
use crate::system_resource::{OneTexture, Vec2toVec3};
use bevy::{prelude::*, state::commands, *};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};
use crate::explosion::ExplosionType;
const ENEMY2_RADIUS: f32 = 14.0;
const ENEMY2_DEFAULT_RETURN: bool = false;
const ENEMY2_DEFAULT_WAIT_TIME: f32 = 3.0;
const ENEMY2_DAFAULT_VELOCITY: f32 = 70.0;
const ENEMY2_DAFAULT_BULLET_VELOCITY: f32 = 130.0;
const ENEMY2_DEFAULT_HP: f32 = 800.0;
#[derive(Component, Default)]
pub struct Enemy2 {
    generated_time: f32,
    arrive_time: Option<f32>,
    bullet_shot_data: Enemy2BulletShotData,
    wait_time: f32,
    direction: Vec2,
    velocity: f32,
    initial_position: Vec2,
    target_position: Vec2,
    is_return: bool,
}
#[derive(Debug, Default)]
struct Enemy2BulletShotData {
    shot_start_time: f32,
    shot_interval: f32,
    prev_shot_time: f32,
}

impl Enemy2BulletShotData {
    pub fn shot(
        &mut self,
        mut commands: Commands,
        position: Vec2,
        direction: Vec2,
        velocity: f32,
        one_texture: &Res<OneTexture>,
    ) {
        let mut position = position;
        // 時間の帳尻合わせ
        if self.prev_shot_time < 0.0 {
            let pass_time = -self.prev_shot_time;
            position += pass_time * direction * velocity;
        }
        let direction = direction.normalize_or(Vec2::Y);
        spawn_enemy_bullet0(commands, position, one_texture, velocity, direction);
    }
}

#[derive(Debug)]
pub struct Enemy2Spawner {
    direction: Vec2,
    wait_time: f32,
    velocity: f32,
    initial_position: Vec2,
    target_position: Vec2,
    is_return: bool,
}

impl Enemy2Spawner {
    pub fn new(strs: &Vec<&str>) -> Self {
        let initial_position: Vec2 = Vec2::new(strs[2].parse().unwrap(), strs[3].parse().unwrap());
        let target_position: Vec2 = Vec2::new(strs[4].parse().unwrap(), strs[5].parse().unwrap());
        let wait_time: f32 = strs[6].parse().unwrap_or(ENEMY2_DEFAULT_WAIT_TIME);
        let is_return: bool = strs[7].parse().unwrap_or(ENEMY2_DEFAULT_RETURN);
        let direction = (target_position - initial_position).normalize_or(Vec2::new(0.0, -1.0));
        Self {
            direction: direction,
            wait_time: wait_time,
            velocity: ENEMY2_DAFAULT_VELOCITY,
            initial_position: initial_position,
            target_position: target_position,
            is_return: is_return,
        }
    }

    pub fn spawn_enemy(&self, command: &mut Commands, one_texture: &Res<OneTexture>) -> Entity {
        let entity = Enemy2Spawner::spawn_enemy_2(
            command,
            one_texture,
            self.wait_time,
            self.direction,
            self.velocity,
            self.initial_position,
            self.target_position,
            self.is_return,
        );
        entity
    }
    fn spawn_enemy_2(
        commands: &mut Commands,
        one_texture: &Res<OneTexture>,
        wait_time: f32,
        direction: Vec2,
        velocity: f32,
        initial_position: Vec2,
        target_position: Vec2,
        is_return: bool,
    ) -> Entity {
        let mut tranform_vec = initial_position.to_vec3();
        tranform_vec.z = -5.0;
        let transform = Transform::from_translation(tranform_vec);
        let entity = commands
            .spawn((
                Name::new("Enemy2"),
                Enemy::new(vec![ENEMY2_DEFAULT_HP], 2500, 10,ExplosionType::Enemy),
                Enemy2 {
                    generated_time: 0.0,
                    arrive_time: None,
                    wait_time: wait_time,
                    direction: direction,
                    velocity: velocity,
                    initial_position: initial_position,
                    target_position: target_position,
                    bullet_shot_data: Enemy2BulletShotData {
                        shot_start_time: 1.5,
                        shot_interval: 0.3,
                        prev_shot_time: 0.0,
                    },
                    is_return: is_return,
                },
                AseSpriteSlice {
                    name: "Enemy2".into(),
                    aseprite: (*one_texture).clone_weak(),
                    ..default()
                },
                transform,
                StateScoped(crate::GameState::InGame)
            ))
            .id();
        add_collision(commands, entity, ENEMY2_RADIUS, Color::srgb(0.5, 0.0, 0.0));
        entity
    }
}

pub fn update_enemy_2(
    mut commands: Commands,
    mut enemy_0_query: Query<(Entity, &mut Enemy2, &mut Transform)>,
    one_texture: Res<OneTexture>,
    time: Res<Time>,
) {
    for (entity, mut enemy2, mut transform) in enemy_0_query.iter_mut() {
        fn is_move(enemy2: &Mut<'_, Enemy2>) -> bool {
            if !enemy2.is_return {
                return true;
            }
            if enemy2.arrive_time.is_none() {
                return true;
            }
            let arrive_time = enemy2.arrive_time.unwrap();
            let now_time = enemy2.generated_time;
            let wait_time = enemy2.wait_time;
            return (now_time - arrive_time > wait_time);
        }
        enemy2.generated_time += time.delta().as_secs_f32();
        let prev_position = transform.translation.xy();
        if is_move(&enemy2) {
            let mut add_position =
                (enemy2.velocity * enemy2.direction * time.delta().as_secs_f32()).to_vec3();
            if enemy2.generated_time < 1.0 {
                // 最初はちょっと遅くしたい
                add_position *= 0.6;
            }
            transform.translation += add_position;
        }
        // 到着判定
        let now_position = transform.translation.xy();

        if (enemy2.target_position - prev_position).dot(enemy2.target_position - now_position)
            <= 0.0
        {
            if enemy2.arrive_time.is_none() {
                enemy2.arrive_time = Some(enemy2.generated_time);
                //
                if enemy2.is_return {
                    enemy2.direction *= -1.0;
                }
            }
        }
        // 弾発射判定
        // 最初の数秒は弾を出さない
        if enemy2.generated_time >= enemy2.bullet_shot_data.shot_start_time {
            if enemy2.bullet_shot_data.prev_shot_time <= 0.0 {
                enemy2.bullet_shot_data.prev_shot_time += enemy2.bullet_shot_data.shot_interval;

                // 交互に時間を設定することで、ちょっとずらす
                if enemy2.bullet_shot_data.shot_interval == 0.3 {
                    enemy2.bullet_shot_data.shot_interval = 0.1;
                } else {
                    enemy2.bullet_shot_data.shot_interval = 0.3;
                }
                // 弾発射
                let bullet_shot_position_diff_x = 20.0;
                let bullet_shot_position_diff_y = 22.0;
                let left_bullet_shot_position = transform.translation.xy()
                    + Vec2::new(bullet_shot_position_diff_x, -bullet_shot_position_diff_y);
                let right_bullet_shot_position = transform.translation.xy()
                    + Vec2::new(-bullet_shot_position_diff_x, -bullet_shot_position_diff_y);
                enemy2.bullet_shot_data.shot(
                    commands.reborrow(),
                    left_bullet_shot_position,
                    Vec2::new(0.0, -1.0),
                    ENEMY2_DAFAULT_BULLET_VELOCITY,
                    &one_texture,
                );
                enemy2.bullet_shot_data.shot(
                    commands.reborrow(),
                    left_bullet_shot_position,
                    Vec2::new(-0.5, -1.0),
                    ENEMY2_DAFAULT_BULLET_VELOCITY * 1.3, // 斜め用はちょっと早く
                    &one_texture,
                );
                enemy2.bullet_shot_data.shot(
                    commands.reborrow(),
                    right_bullet_shot_position,
                    Vec2::new(0.0, -1.0),
                    ENEMY2_DAFAULT_BULLET_VELOCITY,
                    &one_texture,
                );
                enemy2.bullet_shot_data.shot(
                    commands.reborrow(),
                    right_bullet_shot_position,
                    Vec2::new(0.5, -1.0),
                    ENEMY2_DAFAULT_BULLET_VELOCITY * 1.3, // 斜め用はちょっと早く
                    &one_texture,
                );
            }

            enemy2.bullet_shot_data.prev_shot_time -= time.delta().as_secs_f32();
        }
    }
}
