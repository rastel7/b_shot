use crate::se::*;
use crate::{game_state::GameState, system_consts, system_resource};
use ExplosionType::*;
use bevy::{math::VectorSpace, prelude::*, state::commands, *};
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use rand::Rng;
use std::sync::LazyLock;
#[derive(Event, Clone, Copy)]
pub struct ExplosionEvent {
    pub position: Vec2,
    pub explosion_type: ExplosionType,
}
#[derive(Clone, Copy)]
pub enum ExplosionType {
    Player,
    Enemy,
    MiddleBoss,
    BossEnemy,
}
const EXPLOSION_SPLITE_DATA: [(f32, &str); 7] = [
    (0.02, "Explosion1"),
    (0.02, "Explosion2"),
    (0.05, "Explosion3"),
    (0.3, "Explosion4"),
    (0.15, "Explosion3"),
    (0.07, "Explosion2"),
    (0.05, "Explosion5"),
];
#[derive(Component)]
pub struct Explosion {
    passed_time: f32,
}

impl Explosion {
    fn get_sprite_name(&self) -> Option<&str> {
        let mut time_sum = 0.0;
        for (time, name) in EXPLOSION_SPLITE_DATA {
            time_sum += time;
            if self.passed_time < time_sum {
                return Some(name);
            }
        }
        return None;
    }
}
pub fn read_explosion_event(
    mut commands: Commands,
    mut reader: EventReader<ExplosionEvent>,
    zero_texture: Res<system_resource::ZeroTexture>,
    mut se_writer: EventWriter<PlaySEEvent>,
) {
    for event in reader.read() {
        match event.explosion_type {
            Enemy | Player => {
                spawn_explosion(&mut commands, event.position, &zero_texture);
                se_writer.send(PlaySEEvent(SEType::DestroyEnemy));
            }
            MiddleBoss => {
                repeat_explosion(20, 40.0, &mut commands, event.position, &zero_texture);
                se_writer.send(PlaySEEvent(SEType::DestroyBoss));
            }
            BossEnemy => {
                repeat_explosion(40, 60.0, &mut commands, event.position, &zero_texture);
                se_writer.send(PlaySEEvent(SEType::DestroyBoss));
            }
        }
    }
}
fn repeat_explosion(
    count: usize,
    range: f32,
    commands: &mut Commands,
    default_position: Vec2,
    zero_texture: &Res<system_resource::ZeroTexture>,
) {
    for _ in 0..count {
        let position: Vec2 = default_position
            + Vec2::new(
                rand::rng().random_range((-1.0)..(1.0)),
                rand::rng().random_range((-1.0)..(1.0)),
            )
            .normalize_or_zero()
                * range
                * (rand::rng().random_range((0.0)..(1.0)));
        spawn_explosion(commands, position, &zero_texture);
    }
}
fn spawn_explosion(
    commands: &mut Commands,
    position: Vec2,
    zero_texture: &Res<system_resource::ZeroTexture>,
) {
    commands.spawn((
        Name::new("Explosion"),
        Transform::from_xyz(position.x, position.y, 5.0),
        AseSpriteSlice {
            name: EXPLOSION_SPLITE_DATA[0].1.into(),
            aseprite: (*zero_texture).clone_weak(),
            ..default()
        },
        Explosion { passed_time: 0.0 },
        StateScoped(GameState::InGame),
    ));
}
pub fn update_explosion(
    mut commands: Commands,
    mut explosion_query: Query<(Entity, &mut Explosion, &mut AseSpriteSlice)>,
    time: Res<Time>,
) {
    for (entity, mut explosion, mut asepriteslice) in explosion_query.iter_mut() {
        explosion.passed_time += time.delta().as_secs_f32();
        let name = explosion.get_sprite_name();
        if name.is_none() {
            // エフェクト時間切れのため消す
            commands.entity(entity).despawn_recursive();
            return;
        }
        let name = name.unwrap();
        if name != asepriteslice.name {
            asepriteslice.name = name.into();
        }
    }
}
