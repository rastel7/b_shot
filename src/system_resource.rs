use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};
pub const SYSTEM_TEXTURE_SIZE: usize = 16;

#[derive(Resource, Deref, DerefMut)]
pub struct SystemsTexture(pub Handle<Aseprite>);

#[derive(Resource, Deref, DerefMut)]
pub struct ZeroTexture(pub Handle<Aseprite>);

#[derive(Resource, Deref, DerefMut)]
pub struct OneTexture(pub Handle<Aseprite>);

pub trait Vec2toVec3 {
    fn to_vec3(&self) -> Vec3;
}
impl Vec2toVec3 for Vec2 {
    fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0.0,
        }
    }
}
