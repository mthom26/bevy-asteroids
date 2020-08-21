use bevy::prelude::*;

use crate::{resources::SpawnProjectileListener, events::SpawnProjectileEvent};

pub fn spawn_projectile_system(
    mut commands: Commands,
    mut event_reader_resource: ResMut<SpawnProjectileListener>,
    events: Res<Events<SpawnProjectileEvent>>
) {
    for event in event_reader_resource.event_reader.iter(&events) {
        println!("Spawn projectile: pos - {:?}, rot - {:?}", event.pos, event.rot);
        // TODO - Spawn something
    }
}
