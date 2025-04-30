use super::enemy_reader::EnemyGenerateTimeStopper;
use super::{Enemy, boss_bit, enemy};
use crate::bgm::StopStageBGMEvent;
use crate::explosion::ExplosionType;
use crate::game_state;
use crate::main_game::collision::*;
use crate::system_resource::{OneTexture, Vec2toVec3};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
pub mod boss_attack_one;
pub mod boss_attack_second;
pub mod boss_attack_third;
pub use boss_attack_one::*;
pub use boss_attack_second::*;
pub use boss_attack_third::*;
const BOSSENEMYRADIUS: f32 = 56.0;
#[derive(Component, Default)]
pub struct BossEnemy {
    generated_time: f32,
}
impl BossEnemy {
    pub fn new() -> Self {
        Self {
            generated_time: 0.0,
        }
    }
}
#[derive(Debug)]
pub struct BossEnemySpawner {
    initial_position: Vec2,
    target_position: Vec2,
}
#[derive(Component)]
pub struct BossEnemyMover {
    generated_time: f32,
    initial_position: Vec2,
    target_position: Vec2,
    velocity: f32,
    direction: Vec2,
}

impl BossEnemySpawner {
    pub fn new(strs: &Vec<&str>) -> Self {
        let initial_position = Vec2::new(
            strs[2].parse::<f32>().unwrap(),
            strs[3].parse::<f32>().unwrap(),
        );
        let target_position = Vec2::new(
            strs[4].parse::<f32>().unwrap(),
            strs[5].parse::<f32>().unwrap(),
        );

        Self {
            initial_position: initial_position,
            target_position: target_position,
        }
    }

    pub fn spawn_enemy(&self, command: &mut Commands, one_texture: &Res<OneTexture>) -> Entity {
        let entity = spawn_boss_enemy(
            command,
            one_texture,
            self.initial_position,
            self.target_position,
        );
        entity
    }
}

fn spawn_boss_enemy(
    commands: &mut Commands,
    one_texture: &Res<OneTexture>,
    initial_position: Vec2,
    taraget_position: Vec2,
) -> Entity {
    let transform = Transform::from_xyz(initial_position.x, initial_position.y, -5.0);
    let entity = commands
        .spawn((
            Name::new("BossEnemy"),
            BossEnemy::new(),
            EnemyGenerateTimeStopper,
            StateScoped(game_state::GameState::InGame),
            transform,
            Enemy::new(
                vec![3000.0, 3000.0, 3000.0],
                50000,
                100,
                ExplosionType::BossEnemy,
            ),
            AseSpriteSlice {
                name: "BossEnemy".into(),
                aseprite: (*one_texture).clone_weak(),
                ..default()
            },
            BossEnemyMover {
                generated_time: 0.0,
                initial_position: initial_position,
                target_position: taraget_position,
                velocity: 40.0,
                direction: (taraget_position - initial_position).normalize_or(-Vec2::Y),
            },
        ))
        .id();

    add_collision(
        commands,
        entity,
        BOSSENEMYRADIUS,
        Color::srgb(0.5, 0.0, 0.0),
    );
    entity
}

pub fn update_move_boss_enemy(
    mut commands: Commands,
    mut query: Query<(&mut BossEnemyMover, &mut Transform, Entity)>,
    time: Res<Time>,
    mut stagebgm_event_writer: EventWriter<crate::bgm::StopStageBGMEvent>,
    mut bossbgm_event_writer: EventWriter<crate::bgm::StartBossBGMEvent>,
    bgm_query: Query<&crate::bgm::BossBGM>,
) {
    if query.iter().len() == 0 {
        return;
    }
    let delta = time.delta().as_secs_f32();
    stagebgm_event_writer.send(StopStageBGMEvent);
    if bgm_query.iter().len() <= 0 {
        bossbgm_event_writer.send(crate::bgm::StartBossBGMEvent);
    }
    for (mut mover, mut transform, entity) in query.iter_mut() {
        mover.generated_time += time.delta().as_secs_f32();
        let prev_position = transform.translation.xy();

        transform.translation.x += mover.velocity * mover.direction.x * delta;
        transform.translation.y += mover.velocity * mover.direction.y * delta;
        let position = transform.translation.xy();

        if (mover.target_position - prev_position).dot(mover.target_position - position) < 0.0 {
            transform.translation = mover.target_position.to_vec3();
            commands.entity(entity).insert(BossEnemyAttackOne::new());
            commands.entity(entity).remove::<BossEnemyMover>();
        }
    }
}
