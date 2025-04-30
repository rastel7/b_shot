use crate::main_game::player::Player;
use crate::system_consts;
use bevy::{
    math::{VectorSpace, vec3},
    prelude::*,
    state::commands,
};

#[derive(Component)]
pub struct HitedPlayerMovement {
    pub passed_time: f32,
    target_time: f32,
    initial_position: Vec2,
    target_position: Vec2,
    player_move_ease: EasingCurve<f32>,
}

impl Default for HitedPlayerMovement {
    fn default() -> Self {
        let target_time = 2.0;
        Self {
            passed_time: 0.0,
            target_time: target_time,
            initial_position: Vec2::new(0.0, -system_consts::SCREEN_VIRTIAL_HALF_SIZE.1 * 1.3),
            target_position: Vec2::new(0.0, -system_consts::SCREEN_VIRTIAL_HALF_SIZE.1 * 0.7),
            player_move_ease: easing::EasingCurve::new(0.0, 1.0, EaseFunction::SineIn),
        }
    }
}
pub fn update_hited_player_movement(
    mut commands: Commands,
    mut player_query: Query<(
        Entity,
        &mut HitedPlayerMovement,
        &mut Transform,
        &mut Player,
        &mut Visibility,
    )>,
    time: Res<Time>,
    game_clear: Query<&super::game_clear::GameClear>,
) {
    let mut player = player_query.get_single_mut();
    if player.is_err() {
        // 無いので終了
        return;
    }
    let is_clear = game_clear.get_single().is_ok();
    if is_clear{
        return;
    }
    let mut player = player.unwrap();
    player.3.set_is_cotroled_auto(true);
    // 撃破後に非表示にしていたのを戻す
    if player.1.passed_time != 0.0 {
        *player.4 = Visibility::Inherited;
    }
    player.1.passed_time += time.delta().as_secs_f32();
    let t = player
        .1
        .player_move_ease
        .sample(player.1.passed_time / player.1.target_time);
    if t.is_none() {
        player.2.translation.x = player.1.target_position.x;
        player.2.translation.y = player.1.target_position.y;
        // 動けるようになってからの無敵時間設定
        player.3.set_invisible_time(-1.5);
        // 既に時間切れなのでコンポーネントを消す
        commands.entity(player.0).remove::<HitedPlayerMovement>();
        return;
    }
    let t = t.unwrap();
    let new_position = (player.1.initial_position) * (1.0 - t) + (player.1.target_position) * (t);
    player.2.translation.x = new_position.x;
    player.2.translation.y = new_position.y;
}
