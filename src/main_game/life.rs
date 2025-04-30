use bevy::{
    ecs::query,
    math::{VectorSpace, vec3},
    prelude::*,
    reflect::List,
    state::commands,
};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};

use crate::{game_state::GameState, se::PlaySEEvent, system_consts, system_resource::ZeroTexture};
#[derive(Event)]
pub struct IncrementLife(pub usize);
#[derive(Resource)]
pub struct Life {
    pub life: i32,
}

impl Life {
    pub fn new() -> Self {
        Self { life: 3 }
    }
}
#[derive(Component)]
pub struct LifeUI(u32);
pub fn setup_player_life(
    mut commands: Commands,
    mut life: ResMut<Life>,
    zero_texture: Res<ZeroTexture>,
) {
    (*life) = Life::new();

    let parent_entity = commands
        .spawn((
            Name::new("LifeParent"),
            Transform::from_xyz(
                -system_consts::SCREEN_VIRTIAL_HALF_SIZE.0,
                -system_consts::SCREEN_VIRTIAL_HALF_SIZE.1,
                110.0,
            ),
            StateScoped(GameState::InGame),
            Visibility::Inherited,
        ))
        .id();
    let max_count: usize = 20;
    for i in 0..max_count {
        let entity = commands.entity(parent_entity).with_child((
            Name::new(format!("Life {}", i)),
            Transform::from_xyz(8.0 + i as f32 * 16.0, 8.0, 0.0),
            AseSpriteSlice {
                name: "Life".into(),
                aseprite: zero_texture.clone(),
            },
            LifeUI(i as u32),
            Visibility::Inherited,
        ));
    }
}

pub fn update_life_ui(mut query: Query<(&mut LifeUI, &mut Visibility)>, life: Res<Life>) {
    for (mut lifeui, mut visibility) in query.iter_mut() {
        let life = life.life;
        *visibility = if (lifeui.0 as i32) >= life {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

pub fn update_life(mut life: ResMut<Life>, mut add_life: EventReader<IncrementLife>,mut se_writer:EventWriter<PlaySEEvent>) {
    for add in add_life.read() {
        life.life += add.0 as i32;
        se_writer.send(PlaySEEvent(crate::se::SEType::GetLife));
    }
}
