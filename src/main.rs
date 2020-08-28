#![feature(clamp)]

use bevy::{
    input::keyboard::KeyboardInputState, input::mouse::MouseMotion, prelude::*,
    render::pass::ClearColor, window::CursorMoved,
};
use bevy_rapier2d::physics::RapierPhysicsPlugin;

mod components;
mod entities;
mod events;
mod resources;
mod systems;
mod utils;
use events::SpawnProjectileEvent;
use resources::*;
use systems::{
    cursor_pos_system, handle_physics_events_system, player_input_system, screen_wrap_system,
    setup, spawn_asteroid_system, spawn_projectile_system, visibility_system,
};

pub const DENSITY: f32 = 0.002;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_event::<SpawnProjectileEvent>()
        .init_resource::<CursorPos>()
        .init_resource::<SpawnProjectileListener>()
        .add_resource(ClearColor(Color::rgb(0.02, 0.02, 0.02)))
        .add_resource(AsteroidSpawnTimer(Timer::from_seconds(3.0, true)))
        .add_startup_system(setup.system())
        .add_system(cursor_pos_system.system())
        .add_system(player_input_system.system())
        .add_system(spawn_asteroid_system.system())
        .add_system(spawn_projectile_system.system())
        .add_system(screen_wrap_system.system())
        .add_system(visibility_system.system())
        .add_system(handle_physics_events_system.system())
        // .add_system(test_player_bundle.system())
        // .add_system(print_mouse_events.system())
        .run();
}

// fn print_mouse_events(
//     mut mouse_input_state: ResMut<CursorPos>,
//     mouse_motion_events: Res<Events<MouseMotion>>,
//     cursor_moved_events: Res<Events<CursorMoved>>,
// ) {
//     // for event in input_state.mouse_motion_event_reader.iter(&mouse_motion_events) {
//     //     println!("{:?}", event);
//     // }

//     for event in mouse_input_state
//         .cursor_moved_event_reader
//         .iter(&cursor_moved_events)
//     {
//         // println!("Event: {:?}", event.position);
//         // println!("Converted: {:?}", cursor_pos_to_world_pos(&event.position));
//         // println!();
//     }
// }
