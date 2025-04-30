use crate::{
    game_state, system_consts,
    system_resource::{SYSTEM_TEXTURE_SIZE, SystemsTexture},
};
use bevy::{prelude::*, state::commands};
use bevy_aseprite_ultra::prelude::AseSpriteSlice;

#[derive(Component)]
struct EnemyHPGaugeFrame;

#[derive(Component)]
pub struct EnemyHPGauge {
    pub hp_rate: f32,
    pub remain_index: usize,
}
#[derive(Component)]
pub struct EnemyHPGaugeBlock {
    pub index: usize,
}
pub fn initialize_enemy_hp_gauge(mut commands: Commands, system_texture: Res<SystemsTexture>) {
    let parent_entity = commands
        .spawn((
            Name::new("EnemyHP_GaugeParent"),
            StateScoped(game_state::GameState::InGame),
            Transform::from_xyz(
                system_consts::SCREEN_VIRTIAL_HALF_SIZE.0,
                system_consts::SCREEN_VIRTIAL_HALF_SIZE.1,
                110.0,
            ),
            Visibility::Visible,
        ))
        .id();

    commands.entity(parent_entity).with_child((
        Name::new("EnemyHP_GaugeFrame"),
        Transform::from_xyz(0.0, 0.0, 0.0),
        AseSpriteSlice {
            name: "HPFrame".into(),
            aseprite: system_texture.clone(),
        },
        EnemyHPGaugeFrame,
        Visibility::Inherited,
    ));
    commands.entity(parent_entity).with_child((
        Name::new("EnemyHP_Gauge"),
        Transform::from_xyz(0.0, 0.0, -1.0),
        AseSpriteSlice {
            name: "HPGauge".into(),
            aseprite: system_texture.clone(),
        },
        EnemyHPGauge {
            hp_rate: 0.0,
            remain_index: 0,
        },
        Visibility::Inherited,
    ));
    commands.entity(parent_entity).with_child((
        Name::new("EnemyHP_GaugeBackGround"),
        Transform::from_xyz(0.0, 0.0, -2.0),
        AseSpriteSlice {
            name: "HPGaugeBackGround".into(),
            aseprite: system_texture.clone(),
        },
        Visibility::Inherited,
    ));

    for i in 0..6 {
        commands.entity(parent_entity).with_child((
            Name::new(format!("EnemyHP_GaugeBlock:{i}")),
            Transform::from_xyz(-8.0 - 16.0 * i as f32, -14.0, -1.0),
            AseSpriteSlice {
                name: "HPGaugeBlock".into(),
                aseprite: system_texture.clone(),
            },
            Visibility::Visible,
            EnemyHPGaugeBlock { index: i },
        ));
    }
}

pub fn update_enemy_hp_gauge(mut query: Query<(&EnemyHPGauge, &mut Transform)>) {
    let max_x = 96.0;
    for (gauge, mut transform) in query.iter_mut() {
        let rate = 1.0 - gauge.hp_rate;
        transform.translation.x = max_x * (rate);
    }
}

pub fn update_enemy_hp_block(
    gauge_query: Query<&EnemyHPGauge>,
    mut block: Query<(&mut EnemyHPGaugeBlock, &mut Visibility)>,
) {
    let gauge = gauge_query.get_single();
    if gauge.is_err() {
        return;
    }
    let gauge = gauge.unwrap();
    for (enemygauge, mut visibility) in block.iter_mut() {
        *visibility = if enemygauge.index < gauge.remain_index {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
