use bevy::prelude::*;

use crate::components::*;

pub fn move_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Translation,
        &mut Rotation,
        &mut Rot,
        &Velocity,
        &AngularVelocity,
    )>,
) {
    for (mut translation, mut rotation, mut rot, velocity, ang_velocity) in &mut query.iter() {
        translation.0 += velocity.0 * time.delta_seconds;
        rot.0 += ang_velocity.0 * time.delta_seconds;

        rotation.0 = Quat::from_rotation_z(rot.0);
    }
}

pub fn drag_system(
    time: Res<Time>,
    mut query: Query<(&Drag, &mut Velocity, &mut AngularVelocity)>,
) {
    for (drag, mut velocity, mut ang_velocity) in &mut query.iter() {
        let drag_vel = velocity.0.clone() * drag.linear * time.delta_seconds;
        velocity.0 -= drag_vel;
        let drag_ang = ang_velocity.0.clone() * drag.angular * time.delta_seconds;
        ang_velocity.0 -= drag_ang;
    }
}
