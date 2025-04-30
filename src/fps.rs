use crate::system_consts::SCREEN_VIRTIAL_HALF_SIZE;
use crate::system_resource::SystemsTexture;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
#[derive(Component)]
pub struct FpsDrawer {
    pub index: usize,
}

pub fn generate_fps_drawer(mut commands: Commands, system_resource: Res<SystemsTexture>) {
    let parent_entity = commands
        .spawn((
            Name::new("FpsDrawerParent"),
            Transform::from_xyz(
                SCREEN_VIRTIAL_HALF_SIZE.0,
                -SCREEN_VIRTIAL_HALF_SIZE.1 + 8.0,
                100.0,
            ),
            Visibility::Visible,
        ))
        .id();
    commands.entity(parent_entity).with_child((
        Transform::from_xyz(-8.0, 0.0, 0.0),
        Name::new("FpsDrawer0"),
        FpsDrawer { index: 0 },
        AseSpriteSlice {
            name: "0".into(),
            aseprite: system_resource.clone(),
            ..Default::default()
        },
        Visibility::Inherited,
    ));
    commands.entity(parent_entity).with_child((
        Transform::from_xyz(-24.0, 0.0, 0.0),
        Name::new("FpsDrawer1"),
        FpsDrawer { index: 1 },
        AseSpriteSlice {
            name: "0".into(),
            aseprite: system_resource.clone(),
            ..Default::default()
        },
        Visibility::Inherited,
    ));
}
pub fn update_fps_drawer(
    mut query: Query<(&mut AseSpriteSlice, &FpsDrawer)>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        let value = value.round();
        let str = format!("{:0>7}", value);
        // fps_drawer内のインデックスに合わせて反転
        let mut str = str.chars().rev().collect::<Vec<char>>();
        for mut fps_drawer in query.iter_mut() {
            fps_drawer.0.name = str[fps_drawer.1.index].to_string();
        }
    }
}
