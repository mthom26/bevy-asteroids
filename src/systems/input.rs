use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent,
    rapier::dynamics::RigidBodySet,
    na::{self, Vector2},
};

use crate::{
    components::*,
    events::SpawnProjectileEvent,
    utils::{cursor_pos_to_world_pos, rotate_vec2, rotate_vec2_na},
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
    mut body_set: ResMut<RigidBodySet>,
    mut query: Query<(
        &Player,
        &RigidBodyHandleComponent,
        &mut Weapon,
    )>,
) {
    for (_player, body_handle, mut weapon) in &mut query.iter() {
        let mut body = body_set.get_mut(body_handle.handle()).unwrap();

        let mut force = Vector2::new(0.0, 0.0);

        if keyboard_input.pressed(KeyCode::A) {
            force.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            force.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            force.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            force.y -= 1.0;
        }

        if let Some(force) = force.try_normalize(0.01) {
            let rotated_force = rotate_vec2_na(&force, body.position.rotation.angle());
            body.apply_force(rotated_force * 800.0 * time.delta_seconds * 100.0);
        }

        // Rotation
        let player_vec = rotate_vec2(&Vec2::new(0.0, 1.0), body.position.rotation.angle());

        let target_angle = Vec2::new(0.0, 1.0).angle_between(
            cursor_pos_to_world_pos(&mouse_input_state.cursor_position) - Vec2::new(body.position.translation.x, body.position.translation.y)
        );
        let target_vec = rotate_vec2(&Vec2::new(0.0, 1.0), target_angle);
        let rot_angle = player_vec.angle_between(target_vec);

        let target_ang_vel = rot_angle - body.angvel;
        let torque = target_ang_vel / (body.world_inv_inertia_sqrt * body.world_inv_inertia_sqrt);
        let mut torque = torque * time.delta_seconds * 100.0;
        
        // Limit max rotational force produced
        if torque < -14000.0 {
            torque = -14000.0;
        }
        if torque > 14000.0 {
            torque = 14000.0;
        }

        body.apply_torque(torque);
        
        // Limit max rotational velocity
        if body.angvel < -1.0 {
            body.angvel = -1.0;
        }
        if body.angvel > 1.0 {
            body.angvel = 1.0;
        }

        // Decrement Weapon reload time, this should probably be in another system
        // but it is fine here for now
        weapon.reload_timer -= time.delta_seconds;

        if mouse_input.pressed(MouseButton::Left) && weapon.reload_timer <= 0.0 {
            // println!("Player Shoot");
            weapon.reload_timer = weapon.reload_speed;
            my_events.send(SpawnProjectileEvent {
                pos: Translation::new(body.position.translation.x, body.position.translation.y, 3.0),
                rot: body.position.rotation.angle(),
            });
        }
    }
}
