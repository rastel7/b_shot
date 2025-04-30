use bevy::{
    math::{VectorSpace, vec3},
    prelude::*,
    state::commands,
};

use crate::system_consts;

const VERTICAL_COUNT: i32 = 16;
const HORIZONTAL_COUNT: i32 = (VERTICAL_COUNT * system_consts::WINDOW_SIZE.0 as i32
    + system_consts::WINDOW_SIZE.1 as i32
    - 1)
    / system_consts::WINDOW_SIZE.1 as i32;
const LINE_WIDTH: f32 = 1.5;
const HORIZONTAL_BGLINE_GAP: f32 = 2.5;
const BACK_GROUND_COLOR: Color = Color::linear_rgb(0.2, 0.2, 0.25);
#[derive(Component, Reflect, Resource)]

pub struct BackGroundLine {
    direction: Vec2,
    velocity: f32,
    regular_position: Vec3,
    max_position: Vec3,
    is_vertical: bool,
}

pub fn setup_back_ground_line(mut commands: Commands) {
    let back_groung_parent = commands
        .spawn((
            Name::new("BackGroundLineParent"),
            Visibility::Visible,
            Transform::IDENTITY,
            StateScoped(crate::GameState::InGame),
        ))
        .id();
    let distance: f32 =
        system_consts::WINDOW_SIZE.1 / system_consts::CAMERA_SCALE / (VERTICAL_COUNT as f32);
    {
        let initial_top_position = 0.5 * system_consts::WINDOW_SIZE.1 / system_consts::CAMERA_SCALE;
        let mut bottom_position = initial_top_position;

        for i in 0..VERTICAL_COUNT {
            commands.entity(back_groung_parent).with_child((
                Name::new(format!("VerticalBackGroundLine {}", i)),
                BackGroundLine {
                    direction: Vec2::new(0.0, 1.0),
                    velocity: -150.0,
                    regular_position: Vec3::new(0.0, initial_top_position * 2.0, 0.0),
                    max_position: Vec3::new(
                        0.0,
                        initial_top_position - distance * VERTICAL_COUNT as f32,
                        -100.0,
                    ),
                    is_vertical: true,
                },
                Transform::from_translation(Vec3::new(0.0, bottom_position, -100.0))
                    .with_scale(Vec3::new(system_consts::WINDOW_SIZE.0, LINE_WIDTH, 1.0)),
                Sprite {
                    color: BACK_GROUND_COLOR,
                    ..default()
                },
            ));
            bottom_position -= distance;
        }
    }
    {
        let initial_left_position =
            -0.5 * system_consts::WINDOW_SIZE.0 / system_consts::CAMERA_SCALE;
        let mut left_position = initial_left_position;
        for i in 0..HORIZONTAL_COUNT {
            commands.entity(back_groung_parent).with_child((
                Name::new(format!("HorizontalBackGroundLine {}", i)),
                BackGroundLine {
                    direction: Vec2::new(1.0, 0.0),
                    velocity: 0.0,
                    regular_position: Vec3::new(initial_left_position * 2.0, 0.0, 0.0),
                    max_position: Vec3::new(
                        initial_left_position + distance * HORIZONTAL_COUNT as f32,
                        0.0,
                        -100.0,
                    ),
                    is_vertical: true,
                },
                Transform::from_translation(Vec3::new(
                    left_position + HORIZONTAL_BGLINE_GAP,
                    0.0,
                    -100.0,
                ))
                .with_scale(Vec3::new(
                    LINE_WIDTH,
                    system_consts::WINDOW_SIZE.0,
                    1.0,
                )),
                Sprite {
                    color: BACK_GROUND_COLOR,
                    ..default()
                },
            ));
            left_position += distance;
        }
    }
}

pub fn update_back_ground_line(
    mut background_line_query: Query<(&mut BackGroundLine, &mut Transform)>,
    time: Res<Time>,
) {
    let deltatime = time.delta().as_secs_f32();
    for (mut bg_line, mut transfrom) in background_line_query.iter_mut() {
        fn need_reset_position(
            bg_line: &BackGroundLine,
            position: Vec3,
            max_position: Vec3,
        ) -> bool {
            if bg_line.is_vertical {
                position.y < max_position.y
            } else {
                position.x < max_position.x
            }
        }
        let mut add_position = (bg_line.velocity * bg_line.direction * deltatime).xyy();
        add_position.z = 0.0;
        transfrom.translation += add_position;

        if need_reset_position(&bg_line, transfrom.translation, bg_line.max_position) {
            transfrom.translation += bg_line.regular_position;
        }
    }
}
