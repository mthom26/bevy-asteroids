use bevy::prelude::*;

use crate::{components::*, resources::ArenaData};

pub fn visibility_system(
    mut commands: Commands,
    arena_data: Res<ArenaData>,
    mut query: Query<(Entity, &ScreenCheck, &Translation)>,
) {
    for (entity, _, translation) in &mut query.iter() {
        let x = translation.0.x();
        let y = translation.0.y();

        if x > arena_data.half_width
            || x < -arena_data.half_width
            || y > arena_data.half_height
            || y < -arena_data.half_height
        {
            // Despawn entity
            // println!("Removing entity: {:?}", entity);
            // Disable this for now - https://github.com/bevyengine/bevy/issues/299
            // commands.despawn(entity);
        }
    }
}
