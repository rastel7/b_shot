use super::enemy::Enemy;
use super::enemy_bullet::enemy_bullet_0;
use crate::explosion::ExplosionType;
use crate::main_game::{self, collision::*};
use crate::system_resource::{OneTexture, Vec2toVec3};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
const ENEMY1_HP: f32 = 300.0;
const ENEMY1_RADIUS: f32 = 10.0;
const ENEMY1_DEFAULT_WAIT_TIME: f32 = 2.0;
const ENEMY1_BULLET_SHOT_WAIT: f32 = 0.5;
const ENEMY1_BULLET_SHOT_INTERVAL: f32 = 0.1;
const ENEMY1_BULLET_SHOT_NUM: i32 = 3;
const ENEMY1_BULLET_VELOCITY: f32 = 150.0;
#[derive(Component, Default)]
pub struct Enemy1 {
    generated_time: f32,
    arrive_time: Option<f32>,
    bullet_shot_data: Enemy1BulletShotData,
    wait_time: f32,
    direction: Vec2,
    velocity: f32,
    initial_position: Vec2,
    target_position: Vec2,
}
#[derive(Debug, Default)]
struct Enemy1BulletShotData {
    shot_start_time: f32,
    shoted_num: i32,
    shot_interval: f32,
}

impl Enemy1BulletShotData {
    pub fn shot(
        &mut self,
        mut commands: Commands,
        transform: &Transform,
        direction: Vec2,
        velocity: f32,
        one_texture: &Res<OneTexture>,
    ) {
        let mut position = transform.translation.xy();
        // 時間の帳尻合わせ
        if self.shot_interval < 0.0 {
            let pass_time = -self.shot_interval;
            position += pass_time * direction * velocity;
        }
        enemy_bullet_0::spawn_enemy_bullet0(commands, position, one_texture, velocity, direction);
    }
}
#[derive(Debug)]
pub struct Enemy1Spawner {
    direction: Vec2,
    wait_time: f32,
    velocity: f32,
    initial_position: Vec2,
    target_position: Vec2,
}

impl Enemy1Spawner {
    pub fn new(strs: &Vec<&str>) -> Self {
        let target_length = strs[4].parse::<f32>().unwrap();
        let position = Vec2::new(
            strs[2].parse::<f32>().unwrap(),
            strs[3].parse::<f32>().unwrap(),
        );
        let wait_time = strs
            .get(5)
            .unwrap_or(&ENEMY1_DEFAULT_WAIT_TIME.to_string().as_str())
            .parse::<f32>()
            .unwrap_or(ENEMY1_DEFAULT_WAIT_TIME);
        Self {
            direction: (Vec2::new(0.0, -1.0)),
            wait_time: wait_time,
            velocity: (80.0),
            initial_position: position,
            target_position: position - Vec2::new(0.0, target_length),
        }
    }

    pub fn spawn_enemy(&self, command: &mut Commands, one_texture: &Res<OneTexture>) -> Entity {
        let entity = spawn_enemy_1(
            command,
            one_texture,
            self.wait_time,
            self.direction,
            self.velocity,
            self.initial_position,
            self.target_position,
        );
        entity
    }
}

pub fn spawn_enemy_1(
    commands: &mut Commands,
    one_texture: &Res<OneTexture>,
    wait_time: f32,
    direction: Vec2,
    velocity: f32,
    initial_position: Vec2,
    target_position: Vec2,
) -> Entity {
    let mut tranform_vec = initial_position.to_vec3();
    tranform_vec.z = -5.0;
    let transform = Transform::from_translation(tranform_vec);
    let entity = commands
        .spawn((
            Name::new("Enemy1"),
            Enemy1 {
                generated_time: 0.0,
                arrive_time: None,
                wait_time: wait_time,
                direction: direction,
                velocity: velocity,
                initial_position: initial_position,
                target_position: target_position,
                bullet_shot_data: Enemy1BulletShotData::default(),
            },
            AseSpriteSlice {
                name: "Enemy1".into(),
                aseprite: (*one_texture).clone_weak(),
                ..default()
            },
            transform,
            Enemy::new(vec![ENEMY1_HP], 900, 5,ExplosionType::Enemy),
            StateScoped(crate::GameState::InGame)
        ))
        .id();
    add_collision(commands, entity, ENEMY1_RADIUS, Color::srgb(0.5, 0.0, 0.0));
    entity
}

pub fn update_enemy_1(
    mut commands: Commands,
    mut enemy_0_query: Query<(Entity, &mut Enemy1, &mut Transform)>,
    player_query: Query<&Transform, (With<main_game::player::Player>, Without<Enemy1>)>,
    one_texture: Res<OneTexture>,
    time: Res<Time>,
) {
    for (entity, mut enemy1, mut transform) in enemy_0_query.iter_mut() {
        let add_position =
            (enemy1.velocity * enemy1.direction * time.delta().as_secs_f32()).to_vec3();
        enemy1.generated_time += time.delta().as_secs_f32();
        if enemy1.arrive_time.is_none() {
            // 到着前のみ下の方へ動く
            transform.translation += add_position;
        } else {
            // 到着から一定時間経過している場合は上に移動
            if enemy1.arrive_time.is_some() {
                if enemy1.generated_time - enemy1.arrive_time.unwrap() > enemy1.wait_time {
                    transform.translation -= add_position;
                }
            }
        }
        // 到着判定
        if transform.translation.y < enemy1.target_position.y {
            if enemy1.arrive_time.is_none() {
                transform.translation.y = enemy1.target_position.y;
                enemy1.arrive_time = Some(enemy1.generated_time);
                // 弾発射用のデータセット
                enemy1.bullet_shot_data.shot_start_time = enemy1.generated_time;
            }
        }
        // 到着後の弾発射判定
        if let Some(arrive_time) = enemy1.arrive_time {
            if enemy1.bullet_shot_data.shot_start_time <= enemy1.generated_time {
                if enemy1.bullet_shot_data.shot_interval <= 0.0 {
                    // 角度計算
                    let player_position = player_query.get_single().unwrap().translation.xy();
                    let direction =
                        (player_position - transform.translation.xy()).normalize_or(Vec2::Y);

                    enemy1.bullet_shot_data.shot(
                        commands.reborrow(),
                        &transform,
                        direction,
                        ENEMY1_BULLET_VELOCITY,
                        &one_texture,
                    );
                    enemy1.bullet_shot_data.shoted_num += 1;
                    enemy1.bullet_shot_data.shot_interval += ENEMY1_BULLET_SHOT_INTERVAL;
                    if enemy1.bullet_shot_data.shoted_num >= ENEMY1_BULLET_SHOT_NUM {
                        enemy1.bullet_shot_data.shot_interval = 0.0;
                        enemy1.bullet_shot_data.shoted_num = 0;
                        enemy1.bullet_shot_data.shot_start_time =
                            enemy1.generated_time + ENEMY1_BULLET_SHOT_WAIT;
                    }
                }
                enemy1.bullet_shot_data.shot_interval -= time.delta().as_secs_f32();
            }
        }
    }
}
