use bevy::prelude::*;

use crate::components::*;

pub fn test_player_bundle(mut query: Query<(&Player, &Sprite)>) {
    for (player, sprite) in &mut query.iter() {
        println!("player sprite: {}", sprite.size);
    }
}
