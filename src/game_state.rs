use bevy::{
  ecs::{schedule::common_conditions, system},
  image::ImageSamplerDescriptor,
  prelude::*,
  window::WindowResolution,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    BeforeInitialize,
    TitleMenu,
    InGame
}