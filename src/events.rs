use bevy::prelude::*;

pub struct SpawnProjectileEvent {
    pub pos: Translation,
    pub rot: f32,
    // Projectiles will also need velocity, material, etc.
    // Maybe just have an enum of projectile types here that the spawner 
    // system will use to decide those values
}

