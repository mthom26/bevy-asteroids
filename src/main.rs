#![feature(clamp)]

use bevy::{
    app::ScheduleRunnerPlugin, input::keyboard::KeyboardInputState, input::mouse::MouseMotion,
    prelude::*, render::pass::ClearColor, window::CursorMoved,
};

// use bevy::{
//     asset::AssetPlugin, core::CorePlugin, diagnostic::DiagnosticsPlugin, input::InputPlugin,
//     pbr::PbrPlugin, render::RenderPlugin, sprite::SpritePlugin, transform::TransformPlugin,
//     type_registry::TypeRegistryPlugin, wgpu::WgpuPlugin, window::WindowPlugin, winit::WinitPlugin,
// };

mod components;
mod entities;
mod resources;
mod systems;
mod utils;
use resources::*;
use systems::{
    cursor_pos_system, drag_system, move_system, player_input_system, screen_wrap_system, setup,
    spawn_asteroid_system, visibility_system,
};

fn main() {
    App::build()
        // ScheduleRunnerPlugin has no effect
        // .add_plugin(ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f32(1.0 / 60.0)))
        .add_default_plugins()
        // .add_plugin(TypeRegistryPlugin::default())
        // .add_plugin(CorePlugin::default())
        // .add_plugin(TransformPlugin::default())
        // .add_plugin(DiagnosticsPlugin::default())
        // .add_plugin(InputPlugin::default())
        // .add_plugin(WindowPlugin::default())
        // .add_plugin(AssetPlugin::default())
        // .add_plugin(RenderPlugin::default())
        // .add_plugin(SpritePlugin::default())
        // .add_plugin(WgpuPlugin::default())
        // .add_plugin(PbrPlugin::default())
        // .add_plugin(WinitPlugin::default())
        .init_resource::<CursorPos>()
        .add_resource(ClearColor(Color::rgb(0.02, 0.02, 0.02)))
        .add_resource(AsteroidSpawnTimer(Timer::from_seconds(3.0)))
        // .add_resource(AssetHandles)
        .add_startup_system(setup.system())
        .add_system(cursor_pos_system.system())
        .add_system(player_input_system.system())
        .add_system(spawn_asteroid_system.system())
        .add_system(move_system.system())
        .add_system(drag_system.system())
        .add_system(screen_wrap_system.system())
        .add_system(visibility_system.system())
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
