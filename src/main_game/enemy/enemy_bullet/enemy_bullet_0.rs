use crate::{
    main_game::collision,
    system_resource::{OneTexture, Vec2toVec3},
};
use bevy::{prelude::*, render::view::visibility, state::commands, *};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};

use super::enemy_bullet::{self, EnemyBullet};
#[derive(Component)]
pub struct EnemyBullet0 {
    direction: Vec2,
}
const BULLET0_RADIUS: f32 = 5.0;
pub fn spawn_enemy_bullet0(
    mut commands: Commands,
    position: Vec2,
    one_texture: &Res<OneTexture>,
    velocity: f32,
    direction: Vec2,
) {
    let position = Vec3::new(position.x,position.y,11.0);
    let entity = commands
        .spawn((
            Name::new("Bullet0"),
            enemy_bullet::EnemyBullet::new(false, velocity) ,
            EnemyBullet0 {
                direction: direction,
            },
            Transform::from_translation(position),
            AseSpriteSlice {
                name: "EnemyBullet0".into(),
                aseprite: (*one_texture).clone(),
                ..default()
            },
            StateScoped(crate::GameState::InGame),
            Visibility::Hidden
        ))
        .id();
    collision::add_collision(
        &mut commands,
        entity,
        BULLET0_RADIUS,
        Color::srgb(0.6, 0.0, 0.0),
    );
}

pub fn update_enemy_bullet0(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut EnemyBullet0, &mut EnemyBullet, &mut Transform,&mut Visibility)>,
    time: Res<Time>,
) {
    for (entity, mut bullet0, mut bullet, mut transform , mut visibility) in bullet_query.iter_mut() {
        *visibility = Visibility::Visible;
        let mut add_position = bullet.velocity * bullet0.direction;
        add_position *= time.delta().as_secs_f32();
        transform.translation += add_position.to_vec3();
    }
}