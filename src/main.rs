#![allow(warnings)]
use std::io::Write;

use bevy::audio::{AudioPlugin, AudioSource};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::query;
use bevy::pbr::LightEntity;
use bevy::{
    ecs::{schedule::common_conditions, system},
    image::ImageSamplerDescriptor,
    prelude::*,
    window::WindowResolution,
};
use bevy_aseprite_ultra::prelude::*;
use bevy_remote_inspector::RemoteInspectorPlugins;
use collision::*;
use game_state::GameState;
use main_game::enemy::enemy_bullet::{enemy_bullet, enemy_bullet_0};
use main_game::score::ScoreTip;
use main_game::{
    back_ground::*,
    enemy::{
        boss_bit, boss_enemy, enemy_0::*, enemy_1::update_enemy_1, enemy_2::update_enemy_2,
        enemy_reader::*, middle_boss,
    },
    explosion::ExplosionEvent,
    hited_player::*,
    player::*,
    player_invisible_effect::*,
    score::ScoreTipEvent,
    *,
};
use system_consts::WINDOW_SIZE;
mod bgm;
mod fps;
mod game_state;
mod main_game;
mod se;
mod system_consts;
mod system_resource;
mod title;

fn main() {
    let window = Window {
        title: "G-Shot".to_string(),
        resolution: WindowResolution::new(WINDOW_SIZE.0, WINDOW_SIZE.1),
        resizable: false,
        ..default()
    };
    let primary_window = Some(window);
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window,
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
        )
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(RemoteInspectorPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.6, 0.6, 0.7)))
        .insert_resource::<score::GameScore>(score::GameScore::default())
        .insert_resource::<life::Life>(life::Life::new())
        .init_state::<GameState>()
        .add_event::<ExplosionEvent>()
        .add_event::<ScoreTipEvent>()
        .add_event::<bgm::StopStageBGMEvent>()
        .add_event::<bgm::StartBossBGMEvent>()
        .add_event::<life::IncrementLife>()
        .add_event::<se::PlaySEEvent>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, init_game)
        .add_systems(
            PostStartup,
            (
                fps::generate_fps_drawer,
                score::init_score_renderer,
            ),
        )
        .add_systems(
            OnEnter(GameState::TitleMenu),
            (title::setup_title, score::reset_score),
        )
        .add_systems(
            PreUpdate,
            (set_dyson_hit_bullet_velocity_ratency).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            go_title.run_if(in_state(GameState::BeforeInitialize)),
        )
        .add_systems(
            Update, // ゲーム中常に走らせる処理
            score::update_game_score_renderer,
        )
        .add_systems(Update, fps::update_fps_drawer)
        .add_systems(
            Update,
            title::title_key_input.run_if(in_state(GameState::TitleMenu)),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                setup_main_game_system_set,
                back_ground::setup_back_ground_line,
                setup_enemy_generator,
                start_effect::spawn_gamestart_effect,
                score::set_game_score_zero,
                enemy_hp_gauge::initialize_enemy_hp_gauge,
                bgm::set_stage_bgm,
                life::setup_player_life,
            ),
        )
        .add_systems(
            // PlayerUpdate
            Update,
            (
                update_player,
                update_player_bullet,
                update_dyson_attack,
                update_back_ground_line,
                update_hited_player_movement,
                PlayerInvisibleEffect::update_invisible_effect_manager,
                PlayerInvisibleEffect::update_invisible_effect,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            // EnemyUpdate
            Update,
            (
                update_enemy_generator,
                update_enemy_0,
                update_enemy_1,
                update_enemy_2,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            // EnemyBulletUpdate
            Update,
            (enemy_bullet_0::update_enemy_bullet0).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            // 中ボス用Updater
            Update,
            (
                middle_boss::update_middleboss_transform,
                middle_boss::update_middleboss_bullet_manager,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            // ボス更新
            Update,
            (
                boss_enemy::update_move_boss_enemy,
                boss_enemy::update_boss_attack_one,
                boss_enemy::update_boss_enemy_attack_two,
                boss_enemy::update_boss_enemy_third,
                boss_bit::update_boss_bit,
            ),
        )
        .add_systems(
            // Collision解決
            Update,
            (
                solve_player_bullet_collision,
                solve_player_dyson_collision,
                solve_enemy_bullet0_collision,
                solve_enemy_to_player_collision,
                solve_score_tip_collision,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            // Other
            Update,
            (
                start_effect::update_gamestart_effect,
                explosion::update_explosion,
                score::update_score_tip,
                enemy_hp_gauge::update_enemy_hp_gauge,
                enemy_hp_gauge::update_enemy_hp_block,
                bgm::update_decrease_stage_bgm,
                bgm::update_start_boss_bgm,
                bgm::update_decrease_bgm,
                bgm::update_increase_bgm,
                game_clear::update_game_clear,
                game_clear::update_game_clear_black_wipe,
                game_clear::update_game_clear_logo,
                life::update_life_ui,
                life::update_life,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            PostUpdate,
            ((
                is_need_gameclear_entity.before(enemy::if_despawn_destroy_enemy),
                enemy::if_despawn_destroy_enemy,
                destroy_hited_player_shot,
                enemy_bullet::if_despawn_enemy_bullet,
                enemy::if_despawn_outrange_enemy.after(enemy_bullet::if_despawn_enemy_bullet),
                player::destroy_out_display_player_bullet,
                // チップ生成
                score::spawn_score_tip.after(enemy::if_despawn_destroy_enemy),
                // SE再生
                se::update_se_event,
                // チップ破壊
                score::despawn_scoretip.after(score::spawn_score_tip),
            ))
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            PostUpdate,
            (
                set_hidden_player_if_destroyed, //　撃破されたプレイヤーを一時的に消す処理
                explosion::read_explosion_event, //　爆破エフェクトを作成するイベント
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, reset_button)
        .run();
}

fn init_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((
        Camera2d { ..default() },
        Transform {
            scale: Vec3::new(
                1.0 / system_consts::CAMERA_SCALE,
                1.0 / system_consts::CAMERA_SCALE,
                1.0 / system_consts::CAMERA_SCALE,
            ),
            ..default()
        },
    ));
    // SystemImage
    // SystemImageBundle
    commands.insert_resource(system_resource::SystemsTexture(
        asset_server.load("images/systems.aseprite"),
    ));
    // Images
    commands.insert_resource(system_resource::ZeroTexture(
        asset_server.load("images/0.aseprite"),
    ));
    commands.insert_resource(system_resource::OneTexture(
        asset_server.load("images/1.aseprite"),
    ));
}

fn go_title(mut app_state: ResMut<NextState<GameState>>) {
    app_state.set(GameState::TitleMenu);
}

fn loging_state(app_state: Res<State<GameState>>) {
    info!("{:?}", app_state)
}

fn reset_button(keys: Res<ButtonInput<KeyCode>>, mut app_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.set(GameState::TitleMenu);
    }
}

fn reset_entity_log() {
    std::fs::remove_file("log.txt");
}
fn entity_log(query: Query<(Entity, &Name)>) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("log.txt");
    if file.is_err() {
        file = std::fs::File::create("log.txt");
    }
    let mut file = file.unwrap();
    let mut contents = String::new();
    for (entity, name) in query.iter() {
        file.write_fmt(format_args!("{:?} {:?}\n", entity, name));
    }
}
