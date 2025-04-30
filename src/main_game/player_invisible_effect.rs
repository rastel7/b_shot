use super::collision::Collision;
use super::player::Player;
use crate::{game_state::GameState, system_resource::ZeroTexture};
use bevy::{
    math::{VectorSpace, vec3},
    prelude::*,
    state::commands,
};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};
use std::rc::Rc;
#[derive(Component)]
pub struct PlayerInvisibleEffect {
    pub sprite_entities: Vec<Entity>,
    pub passed_time: f32,
}
#[derive(Component)]
pub struct PlayerInvisibleEffectSprite(usize);
impl PlayerInvisibleEffect {
    pub fn generate_invisible_effect(
        mut commands: Commands,
        parent_entity: Entity,
        zero_texture: &Res<ZeroTexture>,
    ) {
        let effect_entity = commands
            .entity(parent_entity)
            .with_child((
                Transform::from_xyz(0.0, 0.0, -0.1),
                Name::new("PlayerInvisibleManager"),
                Visibility::Hidden,
                StateScoped(GameState::InGame),
            ))
            .id();
        let effect_child0 = commands
            .entity(parent_entity)
            .with_child((
                Visibility::Hidden,
                Transform::from_xyz(0.0, 0.0, -0.1),
                Name::new("PlayerInvisibleEffect0"),
                AseSpriteSlice {
                    name: "Invisible0".into(),
                    aseprite: (*zero_texture).clone_weak(),
                    ..default()
                },
                PlayerInvisibleEffectSprite(0),
            ))
            .id();
        let effect_child1 = commands
            .entity(effect_entity)
            .with_child((
                Visibility::Hidden,
                Transform::from_xyz(0.0, 0.0, -0.1),
                Name::new("PlayerInvisibleEffect1"),
                AseSpriteSlice {
                    name: "Invisible1".into(),
                    aseprite: (*zero_texture).clone_weak(),
                    ..default()
                },
                PlayerInvisibleEffectSprite(1),
            ))
            .id();
        let muteffect = PlayerInvisibleEffect {
            sprite_entities: vec![effect_child0, effect_child1],
            passed_time: 0.0,
        };
        commands.entity(effect_entity).insert((muteffect));
    }
    fn get_show_sprite_id(&self) -> usize {
        if self.passed_time % 0.1 <= 0.05 {
            return 0;
        } else {
            return 1;
        }
    }
    pub fn update_invisible_effect_manager(
        player_query: Query<&Player>,
        mut invisible_effect_query: Query<&mut PlayerInvisibleEffect>,
        time: Res<Time>,
    ) {
        let player = player_query.get_single();
        if player.is_err() {
            return;
        }
        let player = player.unwrap();
        if !player.is_invisible() {
            return;
        }

        let invisible_effect = invisible_effect_query.get_single_mut();
        if invisible_effect.is_err() {
            return;
        }
        let mut invisible_effect = invisible_effect.unwrap();
        invisible_effect.passed_time += time.delta().as_secs_f32();
    }

    pub fn update_invisible_effect(
        player_query: Query<&Player>,
        invisible_effect_query: Query<&PlayerInvisibleEffect>,
        mut invisible_effect_sprite_query: Query<(
            &mut PlayerInvisibleEffectSprite,
            &mut Visibility,
        )>,
    ) {
        let player = player_query.get_single();
        if player.is_err() {
            return;
        }
        let player = player.unwrap();
        if !player.is_invisible() {
            // すべてのスプライトを非表示に
            for mut invisible_effect_sprite in invisible_effect_sprite_query.iter_mut() {
                *invisible_effect_sprite.1 = Visibility::Hidden;
            }
            return;
        }
        let invisible_effect = invisible_effect_query.get_single();
        if invisible_effect.is_err() {
            return;
        }
        let invisible_effect = invisible_effect.unwrap();
        if invisible_effect.passed_time == 0.0 {
            // まだ画面下に移動する処理が走っていないため、非表示に
            for mut invisible_effect_sprite in invisible_effect_sprite_query.iter_mut() {
                *invisible_effect_sprite.1 = Visibility::Hidden;
            }
            return;
        }
        let target_sprite_id = invisible_effect.get_show_sprite_id();

        for mut invisible_effect_sprite in invisible_effect_sprite_query.iter_mut() {
            let id = invisible_effect_sprite.0.0;
            *invisible_effect_sprite.1 = if id == target_sprite_id {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}
