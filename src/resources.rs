use bevy::prelude::*;

use crate::events::SpawnProjectileEvent;

pub struct AsteroidSpawnTimer(pub Timer);

pub struct AssetHandles {
    // pub player_texture: Handle<ColorMaterial>,
    pub asteroid_texture: Handle<ColorMaterial>,
}

// Resource to store the current cursor position
#[derive(Default)]
pub struct CursorPos {
    // mouse_motion_event_reader: EventReader<MouseMotion>,
    pub cursor_moved_event_reader: EventReader<CursorMoved>,
    pub cursor_position: Vec2,
}

pub struct ArenaData {
    pub half_width: f32,
    pub half_height: f32,
}

#[derive(Default)]
pub struct SpawnProjectileListener {
    pub event_reader: EventReader<SpawnProjectileEvent>
} 
