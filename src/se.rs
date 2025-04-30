use bevy::{
    audio::{self, Volume},
    prelude::*,
};

#[derive(Event)]
pub struct PlaySEEvent(pub SEType);


#[derive(PartialEq,Eq,Clone,Copy)]
pub enum SEType {
    HitPlayerShot,
    PlayerDyson,
    DestroyEnemy,
    DestroyBoss,
}

impl std::fmt::Display for SEType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SEType::HitPlayerShot => write!(f, "HitPlayerShot"),
            SEType::PlayerDyson => write!(f, "PlayerDyson"),
            SEType::DestroyEnemy => write!(f, "DestroyEnemy"),
            SEType::DestroyBoss => write!(f, "DestroyBoss"),
        }
    }
}
impl SEType {
    pub fn get_path(&self) -> &str {
        match self {
            Self::HitPlayerShot => "sounds/hit_shot.ogg",
            Self::PlayerDyson => "sounds/shot_dyson.ogg",
            Self::DestroyEnemy => "sounds/destroy_enemy.ogg",
            Self::DestroyBoss => "sounds/destroy_boss.ogg",
        }
    }
    pub fn get_volume(&self) -> f32 {
      match self {
        Self::HitPlayerShot => 0.05,
        Self::PlayerDyson => 0.2,
        Self::DestroyEnemy => 0.3,
        Self::DestroyBoss => 0.5,
    }
    }
}

pub fn update_se_event(
    mut commands: Commands,
    mut event_reader: EventReader<PlaySEEvent>,
    asset_server: Res<AssetServer>,
) {
    let mut played_sound: Vec<SEType> = Vec::new();
    for event in event_reader.read() {
        if played_sound.contains(&event.0) {
            continue;
        }
        played_sound.push(event.0.clone());
        let source: Handle<AudioSource> = asset_server.load(event.0.get_path());
        commands.spawn((
            Name::new(format!("{}", event.0)),
            AudioPlayer::new(source),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(event.0.get_volume())),
            StateScoped(crate::GameState::InGame),
        ));
    }
}
