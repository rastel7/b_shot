
use bevy::{math::VectorSpace, prelude::*, state::commands, *};

use crate::system_resource::ZeroTexture;
pub mod player;
pub mod collision;
pub mod back_ground;
pub mod enemy;
pub mod start_effect;
pub mod hited_player;
pub mod player_invisible_effect;
pub mod explosion;
pub mod score;
pub mod enemy_hp_gauge;
pub fn setup_main_game_system_set(mut commands: Commands, zero_texture: Res<ZeroTexture>) {
    player::setup_player(commands, zero_texture);
}
