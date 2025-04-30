use bevy::{
    audio::{self, Volume},
    prelude::*,
    render::extract_component::ExtractComponent,
};
#[derive(Component)]
pub struct StageBGM;

#[derive(Component)]
pub struct BossBGM;

#[derive(Component)]
struct TitleBGM;
#[derive(Event)]
pub struct StopStageBGMEvent;
#[derive(Event)]
pub struct StartBossBGMEvent;
#[derive(Component)]
pub struct DecreaseBGM(f32);
#[derive(Component)]
pub struct IncreaseBGM(f32, f32);
pub fn set_stage_bgm(mut commands: Commands, asset_server: Res<AssetServer>) {
    let source: Handle<AudioSource> = asset_server.load("sounds/stage.ogg");
    commands.spawn((
        Name::new("stage bgm"),
        AudioPlayer::new(source),
        PlaybackSettings::LOOP.with_volume(Volume::new(0.3)),
        StateScoped(crate::GameState::InGame),
        StageBGM,
    ));
}
pub fn start_boss_bgm(mut commands: Commands, asset_server: &Res<AssetServer>) -> Entity {
    let source: Handle<AudioSource> = asset_server.load("sounds/boss.ogg");
    commands
        .spawn((
            Name::new("boss bgm"),
            AudioPlayer::new(source),
            PlaybackSettings::LOOP.with_volume(Volume::new(0.0)),
            StateScoped(crate::GameState::InGame),
            BossBGM,
            IncreaseBGM(0.1, 0.5),
        ))
        .id()
}
pub fn update_decrease_stage_bgm(
    mut commands: Commands,
    query: Query<Entity, (With<StageBGM>, Without<DecreaseBGM>)>,
    mut event_reader: EventReader<StopStageBGMEvent>,
) {
    for _ in event_reader.read() {
        for bgm in query.iter() {
            commands.entity(bgm).insert(DecreaseBGM(1.0));
        }
    }
}

pub fn update_start_boss_bgm(
    mut commands: Commands,
    mut event_reader: EventReader<StartBossBGMEvent>,
    asset_server: Res<AssetServer>,
    query: Query<&BossBGM>,
) {
    for _ in event_reader.read() {
        if query.iter().len() == 0 {
            start_boss_bgm(commands.reborrow(), &asset_server);
        }
    }
}

pub fn update_decrease_bgm(
    mut commands: Commands,
    mut query: Query<(&DecreaseBGM, &AudioSink, Entity)>,
    time: Res<Time>,
) {
    for (decrease, audiosink, entity) in query.iter_mut() {
        let delta = time.delta().as_secs_f32();
        let vol = audiosink.volume() - delta * decrease.0;
        if vol <= 0.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        audiosink.set_volume(vol);
    }
}

pub fn update_increase_bgm(
    mut commands: Commands,
    mut query: Query<(&IncreaseBGM, &AudioSink, Entity)>,
    time: Res<Time>,
) {
    for (increase, audiosink, entity) in query.iter_mut() {
        let delta = time.delta().as_secs_f32();
        let vol = audiosink.volume() + delta * increase.0;
        if vol >= increase.1 {
            audiosink.set_volume(vol);
            commands.entity(entity).remove::<IncreaseBGM>();
            continue;
        }
        audiosink.set_volume(vol);
    }
}
