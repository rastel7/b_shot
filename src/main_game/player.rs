use std::{cmp::min_by, time::Duration};

use crate::system_resource::ZeroTexture;
use crate::{game_state::GameState, se::PlaySEEvent};
use bevy::{
    math::{VectorSpace, vec3},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};

use super::{
    collision::{self, Collision},
    enemy::enemy_bullet::enemy_bullet::EnemyBullet,
    explosion::{self, ExplosionEvent},
    hited_player::{self, HitedPlayerMovement},
    player_invisible_effect,
};
const PLAYER_VELOCITY: f32 = 100.0;
const MOVE_LIMIT: Vec2 = Vec2::new(165.0, 116.0);
const PLAYER_MOVE_SPEED_MAX_TIME: f32 = 0.04;
const PLAYER_SHOT_DAMAGE: f32 = 25.0;
const STRAIGHT_SHOT_FREQUENCY: f32 = 0.15;
const LIGHT_SLANTING_SHOT_FREQUENCY: f32 = 0.22;
const SLANTING_SHOT_FREQUENCY: f32 = 0.30;
const STRAIGHT_SHOT_MAX_NUM: usize = 16;
const LIGHT_SLANTING_SHOT_MAX_NUM: usize = 10;
const SLANTING_SHOT_MAX_NUM: usize = 8;
const SHOT_VELOCITY: f32 = 250.0;
const DYSON_ATTACK_LATENCY: f32 = -1.5;
const DYSON_ATTACK_DAMAGE_PER_SECONDS: f32 = 500.0;
const DYSON_ATTACK_ALLIVE_TIME: f32 = 2.0;
const DYSON_ATTACK_VELOCITY: f32 = 20.0;
pub const PLAYER_DAMAGED_INVISIBLE_TIME: f32 = -100.0;
#[derive(Component)]
pub struct Player {
    z_pressed_time: f32,
    moving_time: f32,
    dyson_attak_after_time: f32,
    // 0未満のときは無敵
    invisible_time: f32,

    is_shoting: bool,
    is_cotroled_auto: bool,
}
impl Player {
    pub fn AddDamage(
        &mut self,
        mut commands: Commands,
        entity: Entity,
        collision: &mut Collision,
        explosion_writer: &mut EventWriter<explosion::ExplosionEvent>,
        player_position: Vec2,
    ) {
        self.invisible_time = PLAYER_DAMAGED_INVISIBLE_TIME;
        commands
            .entity(entity)
            .insert(hited_player::HitedPlayerMovement::default());
        collision.invisible = true;
        let effects_position = [
            player_position + Vec2::new(5.0, 5.0),
            player_position + Vec2::new(5.0, -5.0),
            player_position + Vec2::new(-5.0, 5.0),
            player_position + Vec2::new(-5.0, -5.0),
        ];
        for position in effects_position {
            explosion_writer.send(ExplosionEvent {
                position: position,
                explosion_type: explosion::ExplosionType::Player,
            });
        }
    }
    pub fn set_invisible_time(&mut self, invisible_time: f32) {
        self.invisible_time = invisible_time;
    }
    pub fn is_invisible(&self) -> bool {
        return self.invisible_time < 0.0;
    }
    pub fn is_shoting(&self) -> bool {
        self.is_shoting
    }
    pub fn is_cotroled_auto(&self) -> bool {
        self.is_cotroled_auto
    }
    pub fn set_is_cotroled_auto(&mut self, is_controled: bool) {
        self.is_cotroled_auto = is_controled;
    }
}
#[derive(Component)]
pub struct PlayerShot {
    velocity: Vec2,
    pub damage: f32,
    pub is_hit: bool,
}
#[derive(Component)]
pub struct PlayerStraightShot;
#[derive(Component)]
pub struct PlayerLittleSlantingShot;

#[derive(Component)]
pub struct PlayerSlantingShot;
#[derive(Component, Debug)]
pub struct PlayerDysonAttack {
    passed_time: f32,
    damage_per_seconds: f32,
}
impl PlayerDysonAttack {
    pub fn damage_per_seconds(&self) -> f32 {
        self.damage_per_seconds
    }
}
pub fn setup_player(mut commands: Commands, zero_texture: Res<ZeroTexture>) {
    let entity = commands
        .spawn((
            Name::new("Player"),
            StateScoped(GameState::InGame),
            Player {
                z_pressed_time: 0.0,
                moving_time: 0.0,
                dyson_attak_after_time: DYSON_ATTACK_LATENCY,
                invisible_time: 0.0,
                is_shoting: false,
                is_cotroled_auto: false,
            },
            AseSpriteSlice {
                name: "Player".into(),
                aseprite: (zero_texture.clone()),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, -50.0, 10.0)),
        ))
        .id();

    collision::add_collision(&mut commands, entity, 2.0, Color::srgb(0.0, 1.0, 0.0));

    // 無敵用エフェクト準備
    player_invisible_effect::PlayerInvisibleEffect::generate_invisible_effect(
        commands,
        entity,
        &zero_texture,
    );
}

pub fn update_player(
    mut commands: Commands,
    mut player_query: Query<
        (&mut Player, &mut Transform, &mut Collision, Entity),
        Without<hited_player::HitedPlayerMovement>,
    >,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    zero_texture: Res<ZeroTexture>,
    straight_shot: Query<&PlayerStraightShot>,
    little_slanting_shot: Query<&PlayerLittleSlantingShot>,
    slanting_shot: Query<&PlayerLittleSlantingShot>,
    mut se_writer: EventWriter<crate::se::PlaySEEvent>,
) {
    if player_query.get_single().is_err() {
        //HitedPlayerMovementがついていたらこのコンポーネントでは何もしない
        return;
    }
    let mut player = player_query.single_mut();
    // このPlayerコンポーネントで操作している証拠
    player.0.is_cotroled_auto = false;
    // 無敵エフェクト
    {
        // 無敵か否かの更新
        player.2.invisible = player.0.invisible_time < 0.0;

        // 無敵時間の減少
        if player.0.invisible_time < 0.0 {
            player.0.invisible_time += time.delta().as_secs_f32();
        }
    }
    {
        let mut player_transform = player.1.reborrow();
        move_player(
            player_transform.reborrow(),
            time.delta(),
            &keys,
            &mut player.0.moving_time,
        );
        limit_player_position(player_transform);
    }
    {
        update_shot_generation(
            commands.reborrow(),
            player.0.reborrow(),
            player.1.reborrow().translation.xy(),
            &time.delta(),
            &keys,
            &zero_texture,
            straight_shot,
            little_slanting_shot,
            slanting_shot,
        );
    }
    {
        player.0.reborrow().dyson_attak_after_time -= time.delta().as_secs_f32();
        if player.0.reborrow().dyson_attak_after_time < DYSON_ATTACK_LATENCY {
            if keys.just_pressed(KeyCode::KeyX) {
                let mut dyson_position = player.1.translation.xy();
                dyson_position.y += 40.0;
                player.0.reborrow().dyson_attak_after_time = 0.0;
                generate_dyson_attack(commands.reborrow(), dyson_position, &zero_texture);
                se_writer.send(PlaySEEvent(crate::se::SEType::PlayerDyson));
            }
        }
    }
}

fn move_player(
    mut player_transform: Mut<'_, Transform>,
    delta_time: Duration,
    keys: &Res<ButtonInput<KeyCode>>,
    player_movetime: &mut f32,
) {
    let mut move_vec: Vec2 = Vec2::default();
    if keys.pressed(KeyCode::ArrowDown) {
        move_vec.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        move_vec.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        move_vec.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        move_vec.x += 1.0;
    }
    if move_vec == Vec2::ZERO {
        *player_movetime = 0.0;
        return;
    }
    *player_movetime += delta_time.as_secs_f32();
    let velocity = (move_vec.normalize_or_zero()
        * PLAYER_VELOCITY
        * delta_time.as_secs_f32()
        * (1.0_f32).min((*player_movetime / PLAYER_MOVE_SPEED_MAX_TIME)));
    player_transform.translation.x += velocity.x;
    player_transform.translation.y += velocity.y;
}

fn limit_player_position(mut player_transform: Mut<'_, Transform>) {
    if player_transform.translation.x >= MOVE_LIMIT.x {
        player_transform.translation.x = MOVE_LIMIT.x;
    }
    if player_transform.translation.x <= -MOVE_LIMIT.x {
        player_transform.translation.x = -MOVE_LIMIT.x;
    }
    if player_transform.translation.y >= MOVE_LIMIT.y {
        player_transform.translation.y = MOVE_LIMIT.y;
    }
    if player_transform.translation.y <= -MOVE_LIMIT.y {
        player_transform.translation.y = -MOVE_LIMIT.y;
    }
}

fn update_shot_generation(
    mut commands: Commands,
    mut player: Mut<'_, Player>,
    player_position: Vec2,
    delta_time: &Duration,
    keys: &Res<ButtonInput<KeyCode>>,
    zero_texture: &Res<ZeroTexture>,
    straight_shot: Query<&PlayerStraightShot>,
    little_slanting_shot: Query<&PlayerLittleSlantingShot>,
    slanting_shot: Query<&PlayerLittleSlantingShot>,
) {
    if !keys.pressed(KeyCode::KeyZ) {
        player.z_pressed_time = 0.0;
        player.is_shoting = false;
        return;
    }
    player.is_shoting = true;
    let past_z_pressed_time = player.z_pressed_time;
    player.z_pressed_time += delta_time.as_secs_f32();
    // 正面のショット
    if (past_z_pressed_time / STRAIGHT_SHOT_FREQUENCY).ceil()
        != (player.z_pressed_time / STRAIGHT_SHOT_FREQUENCY).ceil()
    {
        if straight_shot.iter().count() < STRAIGHT_SHOT_MAX_NUM {
            {
                shot_generation(
                    commands.reborrow(),
                    "StraightBullet".into(),
                    player_position + Vec2::new(-5.0, 7.0),
                    Vec2::new(0.0, 1.0) * SHOT_VELOCITY,
                    &zero_texture,
                    false,
                );
            }
            {
                shot_generation(
                    commands.reborrow(),
                    "StraightBullet".into(),
                    player_position + Vec2::new(5.0, 7.0),
                    Vec2::new(0.0, 1.0) * SHOT_VELOCITY,
                    &zero_texture,
                    false,
                );
            }
        }
    }
    //　微妙に斜めのショット
    if (past_z_pressed_time / LIGHT_SLANTING_SHOT_FREQUENCY).ceil()
        != (player.z_pressed_time / LIGHT_SLANTING_SHOT_FREQUENCY).ceil()
    {
        if little_slanting_shot.iter().count() <= LIGHT_SLANTING_SHOT_MAX_NUM {
            shot_generation(
                commands.reborrow(),
                "LittleSlantingBullet".into(),
                player_position + Vec2::new(-7.0, 7.0),
                Vec2::new(-0.3, 1.0).normalize() * SHOT_VELOCITY,
                &zero_texture,
                true,
            );
            shot_generation(
                commands.reborrow(),
                "LittleSlantingBullet".into(),
                player_position + Vec2::new(7.0, 7.0),
                Vec2::new(0.3, 1.0).normalize() * SHOT_VELOCITY,
                &zero_texture,
                false,
            );
        }
    }
    // 斜めのショット
    if (past_z_pressed_time / SLANTING_SHOT_FREQUENCY).floor()
        != (player.z_pressed_time / SLANTING_SHOT_FREQUENCY).floor()
    {
        if slanting_shot.iter().count() <= SLANTING_SHOT_MAX_NUM {
            shot_generation(
                commands.reborrow(),
                "SlantingBullet".into(),
                player_position + Vec2::new(-8.0, 9.0),
                Vec2::new(-0.5, 1.0).normalize() * SHOT_VELOCITY,
                &zero_texture,
                true,
            );
            shot_generation(
                commands.reborrow(),
                "SlantingBullet".into(),
                player_position + Vec2::new(8.0, 9.0),
                Vec2::new(0.5, 1.0).normalize() * SHOT_VELOCITY,
                &zero_texture,
                false,
            );
        }
    }
    // 斜めのショット
}

fn shot_generation(
    mut commands: Commands,
    sprite_name: String,
    shot_generate_positon: Vec2,
    velocity: Vec2,
    zero_texture: &Res<ZeroTexture>,
    is_invert: bool,
) {
    let mut shot_generate_positon: Vec3 = shot_generate_positon.xyx();
    shot_generate_positon.z = 1.0;
    {
        let mut entity = commands.spawn((
            Name::new(sprite_name.clone()),
            Transform::from_translation(shot_generate_positon).with_scale(Vec3::new(
                if is_invert { -1.0 } else { 1.0 },
                1.0,
                1.0,
            )),
            PlayerShot {
                velocity: velocity,
                damage: PLAYER_SHOT_DAMAGE,
                is_hit: false,
            },
            AseSpriteSlice {
                name: sprite_name.clone(),
                aseprite: (*zero_texture).clone_weak(),

                ..default()
            },
            Visibility::Hidden,
            StateScoped(crate::GameState::InGame),
        ));
        if sprite_name == "StraightBullet" {
            entity.insert(PlayerStraightShot);
        } else if sprite_name == "LittleSlantingBullet" {
            entity.insert(PlayerLittleSlantingShot);
        } else if sprite_name == "LittleSlantingBullet" {
            entity.insert(PlayerSlantingShot);
        }
        let entity_id = entity.id();
        collision::add_collision(&mut commands, entity_id, 5.0, Color::srgb(0.0, 0.0, 0.5));
    }
}

pub fn update_player_bullet(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut PlayerShot, &mut Transform, &mut Visibility)>,
    time: Res<Time>,
) {
    for (entity, mut bullet, mut transform, mut visibility) in bullet_query.iter_mut() {
        *visibility = Visibility::Visible;
        let mut add_position = bullet.velocity.xyy();
        add_position.z = 0.0;
        add_position *= time.delta().as_secs_f32();
        transform.translation += add_position;
    }
}

pub fn update_dyson_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut dyson_query: Query<
        (
            Entity,
            &mut PlayerDysonAttack,
            &mut Transform,
            &mut Visibility,
        ),
        Without<PlayerShot>,
    >,
    mut enemy_bullet_query: Query<&mut EnemyBullet, Without<PlayerDysonAttack>>,
) {
    for (entity, mut bullet, mut transform, mut visibility) in dyson_query.iter_mut() {
        *visibility = Visibility::Visible;
        bullet.passed_time += time.delta().as_secs_f32();
        let mut add_position = Vec3::ZERO;
        add_position.y += time.delta().as_secs_f32() * DYSON_ATTACK_VELOCITY;
        transform.translation += add_position;
        if bullet.passed_time > DYSON_ATTACK_ALLIVE_TIME {
            // 減速していた弾のスピードを元に戻す
            for mut bullet in enemy_bullet_query.iter_mut() {
                bullet.velocity = bullet.get_initial_velocity();
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
fn generate_dyson_attack(mut commands: Commands, position: Vec2, zero_texture: &Res<ZeroTexture>) {
    let mut position = position.xyx();
    position.z = 0.5;
    let entity = commands
        .spawn((
            Name::new("DysonAttack"),
            PlayerDysonAttack {
                passed_time: 0.0,
                damage_per_seconds: DYSON_ATTACK_DAMAGE_PER_SECONDS,
            },
            Transform::from_translation(position),
            AseSpriteSlice {
                name: "DysonSphere".into(),
                aseprite: (*zero_texture).clone_weak(),
                ..default()
            },
            Visibility::Hidden,
            StateScoped(GameState::InGame),
        ))
        .id();

    collision::add_collision(&mut commands, entity, 37.5, Color::srgb(0.0, 0.0, 0.5));
}

pub fn destroy_hited_player_shot(
    mut commands: Commands,
    shot_query: Query<(Entity, &mut PlayerShot)>,
) {
    for shot in shot_query.into_iter() {
        if shot.1.is_hit {
            commands.entity(shot.0).despawn_recursive();
        }
    }
}

pub fn set_hidden_player_if_destroyed(
    mut player_query: Query<(&Player, &mut Visibility, &HitedPlayerMovement)>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if player.0.is_invisible() && player.2.passed_time == 0.0 {
            *player.1 = Visibility::Hidden;
        }
    }
}

pub fn set_dyson_hit_bullet_velocity_ratency(
    dyson_query: Query<(&PlayerDysonAttack, &Transform, &Collision), Without<EnemyBullet>>,
    mut enemy_bullet_query: Query<
        (&mut EnemyBullet, &Transform, &Collision),
        Without<PlayerDysonAttack>,
    >,
) {
    if let Ok(dyson) = dyson_query.get_single() {
        for (mut bullet, transform, collision) in enemy_bullet_query.iter_mut() {
            let is_hit = Collision::is_hit(
                dyson.2,
                collision,
                &dyson.1.translation,
                &transform.translation,
                true,
            );
            let velocity = if is_hit {
                bullet.get_initial_velocity() * 0.2
            } else {
                bullet.get_initial_velocity()
            };
            bullet.velocity = velocity;
        }
    }
}

pub fn destroy_out_display_player_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<PlayerShot>>,
) {
    for (entity, transform) in bullet_query.iter() {
        let position = transform.translation;
        // 画面外にいっぱい出てたら消す
        let is_out_of_screen = !(-MOVE_LIMIT.x * 1.1 <= position.x
            && position.x <= MOVE_LIMIT.x * 1.1
            && -MOVE_LIMIT.y * 1.1 <= position.y
            && position.y <= MOVE_LIMIT.y * 1.1);
        if is_out_of_screen {
            commands.entity(entity).despawn_recursive();
        }
    }
}
