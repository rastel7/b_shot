use super::enemy::Enemy;
use crate::main_game::collision::*;
use crate::system_resource::{OneTexture, Vec2toVec3};
use bevy::{prelude::*, state::commands, *};
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};
use crate::explosion::ExplosionType;
const ENEMY0RADIUS: f32 = 10.0;
#[derive(Component, Default)]
pub struct Enemy0 {
    generated_time: f32,
    initial_position: Vec2,
    direction: Vec2,
    velocity: f32,
}
#[derive(Debug)]
pub struct Enemy0Spawner {
    direction: Vec2,
    velocity: f32,
    position: Vec2,
}

impl Enemy0Spawner {
    pub fn new(strs: &Vec<&str>) -> Self {
        Self {
            direction: (Vec2::new(0.0, -1.0)),
            velocity: (100.0),
            position: Vec2::new(
                strs[2].parse::<f32>().unwrap(),
                strs[3].parse::<f32>().unwrap(),
            ),
        }
    }

    pub fn spawn_enemy(&self, command: &mut Commands, one_texture: &Res<OneTexture>) -> Entity {
        let entity = spawn_enemy_0(
            command,
            one_texture,
            self.direction,
            self.velocity,
            self.position,
        );
        entity
    }
}

pub fn spawn_enemy_0(
    commands: &mut Commands,
    one_texture: &Res<OneTexture>,
    direction: Vec2,
    velocity: f32,
    position: Vec2,
) -> Entity {
    let mut tranform_vec = position.xyx();
    tranform_vec.z = -5.0;
    let transform = Transform::from_translation(tranform_vec);
    let entity = commands
        .spawn((
            Name::new("Enemy0"),
            Enemy0 {
                initial_position: position,
                direction: direction,
                velocity: velocity,
                ..default()
            },
            AseSpriteSlice {
                name: "Enemy0".into(),
                aseprite: (*one_texture).clone_weak(),
                ..default()
            },
            transform,
            Enemy::new(vec![150.0], 300, 3,ExplosionType::Enemy),
            StateScoped(crate::GameState::InGame)
        ))
        .id();
    add_collision(commands, entity, ENEMY0RADIUS, Color::srgb(0.5, 0.0, 0.0));
    entity
}

pub fn update_enemy_0(
    mut commands: Commands,
    mut enemy_0_query: Query<(Entity, &mut Enemy0, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut enemy0, mut transform) in enemy_0_query.iter_mut() {
        let add_position =
            (enemy0.velocity * enemy0.direction * time.delta().as_secs_f32()).to_vec3();
        transform.translation += add_position;
        enemy0.generated_time += time.delta().as_secs_f32();
    }
}
