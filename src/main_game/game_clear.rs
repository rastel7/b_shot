use super::enemy::boss_enemy::BossEnemy;
use super::enemy::enemy::Enemy;
use super::enemy::enemy_bullet::{enemy_bullet::EnemyBullet, enemy_bullet_0::EnemyBullet0};
use super::explosion::Explosion;
use super::explosion::ExplosionEvent;
use super::score::ScoreTip;
use super::score::ScoreTipEvent;
use super::score::{self, GameScore};
use crate::bgm::{BossBGM, StageBGM};
use crate::enemy::*;
use crate::player::*;
use crate::se::PlaySEEvent;
use crate::system_resource::{SystemsTexture, Vec2toVec3};
use bevy::math::VectorSpace;
use bevy::render::render_resource::encase::private::ReadFrom;
use bevy::scene::ron::de;
use bevy::{prelude::*, state::commands, utils::tracing::Instrument, *};
use bevy_aseprite_ultra::prelude::AseSpriteSlice;

#[derive(Component)]
pub struct GameClear {
    generated_time: f32,
    spawned_wipe: bool,
    played_se: bool,
    is_clear: bool,
}

impl GameClear {
    pub fn new(is_clear: bool) -> Self {
        Self {
            generated_time: 0.0,
            spawned_wipe: false,
            played_se: false,
            is_clear,
        }
    }
}
#[derive(Component)]
pub struct GameClearBlackWipe {
    initial_position: Vec2,
    target_position: Vec2,
    curve: EasingCurve<f32>,
    passed_time: f32,
    goal_time_inverse: f32,
}
impl GameClearBlackWipe {
    pub fn new(initial_position: Vec2) -> Self {
        Self {
            initial_position: initial_position,
            target_position: Vec2::ZERO,
            curve: easing::EasingCurve::new(0.0, 1.0, EaseFunction::SineIn),
            passed_time: 0.0,
            goal_time_inverse: 1.0 / 0.5,
        }
    }
}
#[derive(Component)]
pub struct GameClearLogo {
    initial_position: Vec2,
    target_position: Vec2,
    curve: EasingCurve<f32>,
    passed_time: f32,
    goal_time_inverse: f32,
}
impl GameClearLogo {
    pub fn new(initial_position: Vec2) -> Self {
        Self {
            initial_position: initial_position,
            target_position: Vec2::ZERO,
            curve: easing::EasingCurve::new(0.0, 1.0, EaseFunction::SineIn),
            passed_time: 0.0,
            goal_time_inverse: 1.0 / 1.0,
        }
    }
}

pub fn spawn_game_clear_entity(mut commands: Commands) {
    let entity = commands.spawn((
        Name::new("GameClear"),
        GameClear::new(true),
        StateScoped(crate::GameState::InGame),
        Visibility::Inherited,
    ));
}
pub fn spawn_game_end_entity(mut commands: Commands) {
    let entity = commands.spawn((
        Name::new("GameClear"),
        GameClear::new(false),
        StateScoped(crate::GameState::InGame),
        Visibility::Inherited,
    ));
}
pub fn update_game_clear(
    mut commands: Commands,
    mut query: Query<&mut GameClear>,
    system_texture: Res<SystemsTexture>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<crate::GameState>>,
    mut se_writer: EventWriter<PlaySEEvent>,
    score_tip_query: Query<&ScoreTip>,
    game_score: Res<GameScore>,
    mut boss_bgm_query:Query<Entity,(With<BossBGM>,Without<crate::bgm::DecreaseBGM>)> , 
    mut stage_bgm_query:Query<Entity,(With<StageBGM>,Without<crate::bgm::DecreaseBGM>)>,
    mut high_score : ResMut<score::HighScore>
) {
    
    
    for mut game_clear in query.iter_mut() {

        for boss_bgm in boss_bgm_query.iter() {
            commands
                .entity(boss_bgm)
                .insert(crate::bgm::DecreaseBGM(0.8));
        }
        for stage_bgm in stage_bgm_query.iter() {
            commands
                .entity(stage_bgm)
                .insert(crate::bgm::DecreaseBGM(0.8));
        }
        if score_tip_query.iter().len() != 0 && game_clear.is_clear{
            return;
        }
        game_clear.generated_time += time.delta().as_secs_f32();
        if game_clear.generated_time >= 1.0 && !game_clear.spawned_wipe {
            game_clear.spawned_wipe = true;
            let initial_position = Vec2::new(0.0, -180.0);
            // ワイプ用オブジェクトを作成
            let entity = commands.spawn((
                Name::new("GameClearWipe"),
                Transform::from_xyz(initial_position.x, initial_position.y, 200.0)
                    .with_scale(Vec3::new(371.0, 180.0, 0.0)),
                Sprite {
                    anchor: sprite::Anchor::TopCenter,
                    color: Color::srgb(0.1, 0.1, 0.1),
                    ..Default::default()
                },
                StateScoped(crate::GameState::InGame),
                GameClearBlackWipe::new(initial_position),
            ));
            // ワイプ用オブジェクトを作成
            let initial_position = Vec2::new(0.0, 180.0);
            let entity = commands.spawn((
                Name::new("GameClearWipe"),
                Transform::from_xyz(initial_position.x, initial_position.y, 200.0)
                    .with_scale(Vec3::new(371.0, 180.0, 0.0)),
                Sprite {
                    anchor: sprite::Anchor::BottomCenter,
                    color: Color::srgb(0.1, 0.1, 0.1),
                    ..Default::default()
                },
                StateScoped(crate::GameState::InGame),
                GameClearBlackWipe::new(initial_position),
            ));
            if game_clear.is_clear {
                // ロゴ用オブジェクトを生成
                let initial_position = Vec2::new(300.0, 0.0);
                let entity = commands.spawn((
                    Name::new("GameClearLogo"),
                    Transform::from_xyz(initial_position.x, initial_position.y, 201.0),
                    Sprite {
                        anchor: sprite::Anchor::Center,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    AseSpriteSlice {
                        aseprite: system_texture.clone(),
                        name: "GameClear".into(),
                    },
                    StateScoped(crate::GameState::InGame),
                    GameClearLogo::new(Vec2::new(initial_position.x, initial_position.y)),
                ));
            }
        }
        if game_clear.generated_time >= 1.5 && !game_clear.played_se {
            game_clear.played_se = true;
            if game_clear.is_clear {
                // ファンファーレ
                se_writer.send(PlaySEEvent(crate::se::SEType::ClearFanfare));
            } else {
                se_writer.send(PlaySEEvent(crate::se::SEType::GameOver));
            }
        }
        // タイトル遷移
        if game_clear.generated_time >= 4.0 {
            score::save_score(game_score.get_score(),&mut high_score);
            app_state.set(crate::GameState::TitleMenu);
        }
    }
}

pub fn update_game_clear_black_wipe(
    mut query: Query<(&mut GameClearBlackWipe, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut wipe, mut transform) in query.iter_mut() {
        let delta = time.delta().as_secs_f32();
        wipe.passed_time += delta;
        let t = wipe
            .curve
            .sample(wipe.passed_time * wipe.goal_time_inverse)
            .unwrap_or(1.0);
        let new_position = (t * wipe.target_position + (1.0 - t) * wipe.initial_position);
        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
    }
}

pub fn update_game_clear_logo(
    mut query: Query<(&mut GameClearLogo, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut logo, mut transform) in query.iter_mut() {
        let delta = time.delta().as_secs_f32();
        logo.passed_time += delta;
        let t = logo
            .curve
            .sample(logo.passed_time * logo.goal_time_inverse)
            .unwrap_or(1.0);
        let new_position = (t * logo.target_position + (1.0 - t) * logo.initial_position);
        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
    }
}
