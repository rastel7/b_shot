use bevy::{math::VectorSpace, prelude::*, state::commands, *};

use crate::system_resource::ZeroTexture;
pub mod back_ground;
pub mod collision;
pub mod enemy;
pub mod enemy_hp_gauge;
pub mod explosion;
pub mod game_clear;
pub mod hited_player;
pub mod life;
pub mod player;
pub mod player_invisible_effect;
pub mod score;
pub mod start_effect;
pub mod update_enemy_list;
pub fn setup_main_game_system_set(mut commands: Commands, zero_texture: Res<ZeroTexture>) {
    player::setup_player(commands, zero_texture);
}
