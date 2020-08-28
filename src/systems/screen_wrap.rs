use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::{components::*, resources::ArenaData};

pub fn screen_wrap_system(
    arena_data: Res<ArenaData>,
    mut body_set: ResMut<RigidBodySet>,
    mut query: Query<(&ScreenWrap, &RigidBodyHandleComponent)>,
) {
    for (_, body_handle) in &mut query.iter() {
        let mut body = body_set.get_mut(body_handle.handle()).unwrap();
        let x = body.position.translation.x;
        let y = body.position.translation.y;

        if x > arena_data.half_width {
            body.position.translation.x = -arena_data.half_width;
        } else if x < -arena_data.half_width {
            body.position.translation.x = arena_data.half_width;
        }

        if y > arena_data.half_height {
            body.position.translation.y = -arena_data.half_height;
        } else if y < -arena_data.half_height {
            body.position.translation.y = arena_data.half_height;
        }
    }
}
