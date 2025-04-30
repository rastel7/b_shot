use std::io::{Read, Write};

use crate::game_state::GameState;
use crate::system_consts::SCREEN_VIRTIAL_HALF_SIZE;
use crate::system_resource::{SystemsTexture, ZeroTexture};
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::state::commands;
use bevy_aseprite_ultra::prelude::*;
use rand::Rng;

use super::collision::{self, Collision};
use super::life::IncrementLife;
use super::player::{self, Player};

const SCORE_PATH: &str = "./data.log";
const LIFEUP_SCORE: u32 = 50000;
#[derive(Resource, Default)]
pub struct GameScore(u32);
impl GameScore {
    pub fn reset_score(&mut self) {
        self.0 = 0;
    }
    pub fn add_score(&mut self, add: u32, increment_life_writer: &mut EventWriter<IncrementLife>) {
        let prev =self.0;
        self.0 += add;
        if (self.0 as f32 / LIFEUP_SCORE as f32).floor() > (prev as f32 / LIFEUP_SCORE as f32).floor(){
            increment_life_writer.send(IncrementLife(1));
        }
    }
    pub fn get_score(&self) -> u32 {
        self.0
    }
    pub fn sub_score(&mut self, sub: i32) {
        if self.0 < sub as u32 {
            self.0 = 0;
            return;
        }
        self.0 -= sub as u32;
    }
}
#[derive(Component)]
pub struct ScoreRender(usize);
#[derive(Component)]
pub struct ScoreTip {
    generater_time: f32,
    stop_strew_time_inverse: f32,
    pub score: u32,
    initial_velocity: Vec2,
    velocity: Vec2,
    attenuationer_velocity: EasingCurve<f32>,

    is_end_strew: bool,

    pub is_need_despawn: bool,
}
#[derive(Event, Clone, Copy)]
pub struct ScoreTipEvent {
    pub num: usize,
    pub position: Vec2,
    pub score: u32,
}
pub fn reset_score(mut game_score: ResMut<GameScore>) {
    game_score.0 = read_high_score();
    info!("reset {}", game_score.0);
}

pub fn read_high_score() -> u32 {
    let mut file = std::fs::OpenOptions::new().read(true).open(SCORE_PATH);
    if file.is_err() {
        return 0;
    }
    let mut file = file.unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf);
    return buf.parse::<u32>().unwrap_or(0);
}

pub fn save_score(new_score: u32) {
    if read_high_score() > new_score {
        return;
    };

    let mut file = std::fs::OpenOptions::new().write(true).open(SCORE_PATH);
    if file.is_err() {
        file = std::fs::File::create(SCORE_PATH);
    }
    file.unwrap().write_fmt(format_args!("{}", new_score));
}
pub fn init_score_renderer(mut commands: Commands, system_resource: Res<SystemsTexture>) {
    let score_renderer_parent = commands
        .spawn((
            Name::new("ScoreDrawerParent"),
            Transform::from_xyz(
                -SCREEN_VIRTIAL_HALF_SIZE.0,
                SCREEN_VIRTIAL_HALF_SIZE.1,
                220.0,
            ),
            Visibility::Visible,
        ))
        .id();
    for i in 0..7 {
        commands.entity(score_renderer_parent).with_child((
            Transform::from_xyz(8.0 + 16.0 * i as f32, -8.0, 0.0),
            Name::new(format!("ScoreDrawer:{}", i)),
            ScoreRender(i),
            AseSpriteSlice {
                name: i.to_string(),
                aseprite: system_resource.clone(),
                ..Default::default()
            },
            Visibility::Inherited,
        ));
    }
}

pub fn update_game_score_renderer(
    mut query: Query<(&ScoreRender, &mut AseSpriteSlice)>,
    game_score: Res<GameScore>,
) {
    let str: Vec<char> = format!("{:0>7}", game_score.0).chars().collect();
    for (score_render, mut aseprite) in query.iter_mut() {
        let sprite_name = &aseprite.name.chars().nth(0).unwrap();
        if !(str[score_render.0] == *sprite_name) {
            aseprite.name = str[score_render.0].to_string();
        }
    }
}

pub fn set_game_score_zero(mut game_score: ResMut<GameScore>) {
    game_score.reset_score();
}

pub fn spawn_score_tip(
    mut commands: Commands,
    mut reader: EventReader<ScoreTipEvent>,
    zero_texutre: Res<ZeroTexture>,
) {
    let max_strew_power = 80.0;
    for event in reader.read() {
        for _ in 0..event.num {
            let direction = Vec2::new(rand::random_range(-1.0..1.0), rand::random_range(-1.0..1.0))
                .normalize_or(Vec2::Y);
            let velocity = direction * max_strew_power * rand::random_range(0.5..1.0);

            let entity = commands
                .reborrow()
                .spawn((
                    Name::new("ScoreTip"),
                    ScoreTip {
                        generater_time: 0.0,
                        stop_strew_time_inverse: 1.0 / 0.7,
                        score: event.score,
                        initial_velocity: velocity,
                        velocity: velocity,
                        attenuationer_velocity: easing::EasingCurve::new(
                            0.0,
                            1.0,
                            EaseFunction::SineIn,
                        ),
                        is_end_strew: false,
                        is_need_despawn: false,
                    },
                    AseSpriteSlice {
                        name: "ScoreTip".into(),
                        aseprite: zero_texutre.clone(),
                    },
                    Transform::from_xyz(event.position.x, event.position.y, 0.0),
                    StateScoped(GameState::InGame),
                ))
                .id();
            collision::add_collision(
                &mut commands.reborrow(),
                entity,
                16.0,
                Color::srgb(0.0, 0.4, 0.4),
            );
        }
    }
}

pub fn update_score_tip(
    mut commands: Commands,
    mut tip_query: Query<(&mut ScoreTip, &mut Transform, &Collision, Entity), Without<Player>>,
    player_query: Query<(&Player, &Transform), Without<ScoreTip>>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    for mut tip in tip_query.iter_mut() {
        if tip.0.is_need_despawn {
            continue;
        }
        tip.0.generater_time += delta;
        if !tip.0.is_end_strew {
            let t = tip
                .0
                .attenuationer_velocity
                .sample(tip.0.generater_time * tip.0.stop_strew_time_inverse);
            if t.is_none() {
                tip.0.velocity = Vec2::ZERO;
                tip.0.is_end_strew = true;
            } else {
                let t = 1.0 - t.unwrap();
                tip.0.velocity = tip.0.initial_velocity * t;
            }
        } else {
            let player = player_query.get_single();
            if player.is_err() {
                continue;
            }
            let (player, transform) = player.unwrap();
            let gravity = -Vec2::Y * 90.0;
            let to_player_acceleration_per_seconds = 130.0;
            let is_follor_player = (!player.is_shoting()
                || (transform.translation.xy().distance(tip.1.translation.xy()) <= 10.0))
                && (!player.is_cotroled_auto());
            if is_follor_player {
                let direction =
                    (transform.translation.xy() - tip.1.translation.xy()).normalize_or(Vec2::Y);
                tip.0.velocity = direction * to_player_acceleration_per_seconds;
            } else {
                tip.0.velocity = gravity;
            }
        }

        tip.1.translation.x += tip.0.velocity.x * delta;
        tip.1.translation.y += tip.0.velocity.y * delta;
        if is_out_of_range_screen_tip(&tip.1, tip.2.radius()) {
            tip.0.is_need_despawn = true;
        }
    }
}

const SCREEN_LIMIT_ADD: f32 = 10.0;
fn is_out_of_range_screen_tip(transform: &Transform, collision_radius: f32) -> bool {
    let position = transform.translation;
    let mut maxsize = crate::system_consts::SCREEN_VIRTIAL_HALF_SIZE;
    maxsize.0 += SCREEN_LIMIT_ADD + collision_radius;
    maxsize.1 += SCREEN_LIMIT_ADD + collision_radius;
    !(-maxsize.0 <= position.x
        && position.x <= maxsize.0
        && -maxsize.1 <= position.y
        && position.y <= maxsize.1)
}

pub fn despawn_scoretip(mut commands: Commands, query: Query<(&ScoreTip, Entity)>) {
    for (tip, entity) in query.iter() {
        if tip.is_need_despawn {
            commands.entity(entity).despawn_recursive();
        }
    }
}
