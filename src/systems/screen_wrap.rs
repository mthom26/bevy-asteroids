use bevy::prelude::*;

use crate::{components::*, resources::ArenaData};

pub fn screen_wrap_system(
    arena_data: Res<ArenaData>,
    mut query: Query<(&ScreenWrap, &mut Translation)>,
) {
    for (_, mut translation) in &mut query.iter() {
        let mut new_x = translation.0.x();
        let mut new_y = translation.0.y();

        if new_x > arena_data.half_width {
            new_x = -arena_data.half_width;
        } else if new_x < -arena_data.half_width {
            new_x = arena_data.half_width;
        }

        if new_y > arena_data.half_height {
            new_y = -arena_data.half_height;
        } else if new_y < -arena_data.half_height {
            new_y = arena_data.half_height;
        }

        translation.0 = Vec3::new(new_x, new_y, translation.0.z());
    }
}
