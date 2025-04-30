use super::boss_enemy::{self, BossEnemySpawner};
use super::enemy_0::{self, Enemy0Spawner};
use super::enemy_1::{self, Enemy1Spawner};
use super::enemy_2::{self, Enemy2Spawner};
use super::middle_boss::{self, MiddleBossSpawner};
use crate::{
    game_state::GameState,
    system_consts,
    system_resource::{OneTexture, Vec2toVec3},
};
use bevy::{ecs::query, prelude::*, *};
use std::{io::Read, ops::Deref, panic, path::Path};
const ENEMY_LIST_PATH: &str = "assets/enemy_list.csv";
#[derive(Component)]
pub struct EnemyGenerator {
    generate_data_list: Vec<EnemyGenerateData>,
    generate_list: Vec<String>,
    passed_time: f32,
    test_generated: bool,
}
#[derive(Component)]
pub struct EnemyGenerateTimeStopper;

impl EnemyGenerator {
    pub fn new() -> Self {
        let str = get_string_list(ENEMY_LIST_PATH);
        dbg!(&str);
        Self {
            generate_data_list: generate_enemy_generate_data_list(&str[1..]),
            generate_list: str,
            passed_time: 0.0,
            test_generated: false,
        }
    }

    pub fn search_and_spawn_enemy(
        &mut self,
        commands: &mut Commands,
        one_texture: &Res<OneTexture>,
    ) {
        for generate_data in &mut self.generate_data_list {
            if self.passed_time < generate_data.spawn_time {
                continue;
            }
            if generate_data.generated {
                continue;
            }
            let entity = match &generate_data.type_data {
                EnemyTypeData::Enemy0(data) => data.spawn_enemy(commands, &one_texture),
                EnemyTypeData::Enemy1(data) => data.spawn_enemy(commands, &one_texture),
                EnemyTypeData::Enemy2(data) => data.spawn_enemy(commands, &one_texture),
                EnemyTypeData::MiddleBoss(data) => data.spawn_enemy(commands, &one_texture),
                EnemyTypeData::BossEnemy(data) => data.spawn_enemy(commands, &one_texture),
            };
            commands
                .entity(entity)
                .insert(StateScoped(crate::GameState::InGame));
            generate_data.generated = true;
        }
    }
}
fn generate_enemy_generate_data_list(str_datas: &[String]) -> Vec<EnemyGenerateData> {
    let mut generate_data_list: Vec<EnemyGenerateData> = Vec::new();
    for str in str_datas {
        if str.is_empty() {
            continue;
        }
        if &str[0..2] == "//" {
            continue;
        }
        let splited: Vec<&str> = str.split(',').map(|s| s).collect();
        let mut spawn_time = splited[1].parse::<f32>().unwrap();
        let mut generate_data = (match splited[0] {
            "0" => EnemyGenerateData::new(
                spawn_time,
                EnemyTypeData::Enemy0(Enemy0Spawner::new(&splited)),
            ),
            "1" => EnemyGenerateData::new(
                spawn_time,
                EnemyTypeData::Enemy1(Enemy1Spawner::new(&splited)),
            ),
            "2" => EnemyGenerateData::new(
                spawn_time,
                EnemyTypeData::Enemy2(Enemy2Spawner::new(&splited)),
            ),
            "mid" => EnemyGenerateData::new(
                spawn_time,
                EnemyTypeData::MiddleBoss(MiddleBossSpawner::new(&splited)),
            ),
            "boss" => EnemyGenerateData::new(
                spawn_time,
                EnemyTypeData::BossEnemy(BossEnemySpawner::new(&splited)),
            ),
            _ => {
                panic!("undef spawn enemy data {:?}", str);
            }
        });

        generate_data_list.push(generate_data);
    }

    return generate_data_list;
}

struct EnemyGenerateData {
    generated: bool,
    spawn_time: f32,
    type_data: EnemyTypeData,
}
impl EnemyGenerateData {
    pub fn new(spawn_time: f32, type_data: EnemyTypeData) -> Self {
        Self {
            generated: false,
            spawn_time: spawn_time,
            type_data: type_data,
        }
    }
}
#[derive(Debug)]
enum EnemyTypeData {
    Enemy0(enemy_0::Enemy0Spawner),
    Enemy1(enemy_1::Enemy1Spawner),
    Enemy2(enemy_2::Enemy2Spawner),
    MiddleBoss(middle_boss::MiddleBossSpawner),
    BossEnemy(boss_enemy::BossEnemySpawner)
}

fn get_string_list(path: &str) -> Vec<String> {
    let mut file = std::fs::File::open(path).expect("Undefind file");
    let mut buf = String::new();
    let result = file.read_to_string(&mut buf);
    if result.is_err() {
        panic!("Unload file");
    }
    buf.split('\n').map(|s| s.to_string()).collect()
}

pub fn setup_enemy_generator(mut commands: Commands, mut query: Query<&mut EnemyGenerator>) {
    commands.spawn((
        Name::new("EnemyGenarator"),
        EnemyGenerator::new(),
        StateScoped(GameState::InGame),
    ));
}

pub fn update_enemy_generator(
    mut commands: Commands,
    mut query: Query<&mut EnemyGenerator>,
    stop_query: Query<&EnemyGenerateTimeStopper>,
    one_texture: Res<OneTexture>,
    time: Res<Time>,
) {
    let mut enemy_generator = query.get_single_mut().unwrap();

    let count = stop_query.iter().len();
    if count == 0 {
        enemy_generator.passed_time += time.delta().as_secs_f32();
    }

    enemy_generator.search_and_spawn_enemy(&mut commands, &one_texture);
}
