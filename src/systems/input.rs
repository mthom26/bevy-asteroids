use bevy::prelude::*;

use crate::{
    components::*,
    utils::{cursor_pos_to_world_pos, rotate_vec2},
    events::SpawnProjectileEvent,
    CursorPos,
};

pub fn cursor_pos_system(
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut mouse_input_state: ResMut<CursorPos>,
) {
    for event in mouse_input_state
        .cursor_moved_event_reader
        .iter(&cursor_moved_events)
    {
        mouse_input_state.cursor_position = event.position;
    }
}

pub fn player_input_system(
    time: Res<Time>,
    mouse_input_state: Res<CursorPos>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut my_events: ResMut<Events<SpawnProjectileEvent>>,
    // cursor_moved_events: Res<Events<CursorMoved>>,
    mut query: Query<(
        &Player,
        &Translation,
        &Rot,
        &mut Velocity,
        &mut AngularVelocity,
        &mut Weapon,
    )>,
) {
    let mut x = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        x += 1.0;
    }

    let mut y = 0.0;
    if keyboard_input.pressed(KeyCode::S) {
        y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        y += 1.0;
    }

    let input_vec = if x != 0.0 || y != 0.0 {
        Vec2::new(x, y).normalize()
    } else {
        Vec2::zero()
    };

    for (_player, pos, rot, mut velocity, mut ang_velocity, mut weapon) in &mut query.iter() {
        let input_vec = rotate_vec2(&input_vec, rot.0);
        let input_vec = Vec3::new(input_vec.x(), input_vec.y(), 0.0);
        velocity.0 += input_vec * time.delta_seconds * 200.0;

        let player_vec = rotate_vec2(&Vec2::new(0.0, 1.0), rot.0);
        // println!("player_vec: {}", player_vec);

        let target_angle = Vec2::new(0.0, 1.0).angle_between(
            cursor_pos_to_world_pos(&mouse_input_state.cursor_position)
                - Vec2::new(pos.0.x(), pos.0.y()),
        );
        let tgt_vec = rotate_vec2(&Vec2::new(0.0, 1.0), target_angle);
        // println!("tgt_vec: {}", tgt_vec);

        // Actual angle between current player direction and cursor direction
        let rot_angle = player_vec.angle_between(tgt_vec);

        let acc = 144.0;

        // Take into account current angular velocity
        let tgt_ang_vel = (1.0 * rot_angle) - (0.2 * ang_velocity.0);
        // Clamp it so acceleration is not instant
        let tgt_ang_vel = tgt_ang_vel.clamp(-0.08, 0.08);
        // println!("tgt_ang_vel: {}", tgt_ang_vel);

        ang_velocity.0 += tgt_ang_vel * time.delta_seconds * acc;
        // clamp max speed
        ang_velocity.0 = ang_velocity.0.clamp(-3.0, 3.0);

        // println!("tgt_ang_vel: {}", tgt_ang_vel);
        // println!("ang_vel: {}", ang_velocity.0);
        // println!();


        // Decrement Weapon reload time, this should probably be in another system
        // but it is fine here for now
        weapon.reload_timer -= time.delta_seconds;

        if mouse_input.pressed(MouseButton::Left) && weapon.reload_timer <= 0.0 {
            // println!("Player Shoot");
            weapon.reload_timer = weapon.reload_speed;
            my_events.send(SpawnProjectileEvent {
                pos: Translation::new(pos.x(), pos.y(), 3.0),
                rot: rot.0,
            });
        }
    }
}
