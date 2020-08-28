use bevy::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;

use crate::{
    entities::ProjectileComponents,
    events::SpawnProjectileEvent,
    resources::{AssetHandles, SpawnProjectileListener},
    utils::rotate_vec2,
};

pub fn spawn_projectile_system(
    mut commands: Commands,
    assets: Res<AssetHandles>,
    audio_output: Res<AudioOutput>,
    mut event_reader_resource: ResMut<SpawnProjectileListener>,
    events: Res<Events<SpawnProjectileEvent>>,
) {
    for event in event_reader_resource.event_reader.iter(&events) {
        // println!(
        //     "Spawn projectile: pos - {:?}, rot - {:?}",
        //     event.pos, event.rot
        // );
        // TODO - Projectile should probably store its own velocity somewhere
        let vel = rotate_vec2(&Vec2::new(0.0, 100.0), event.rot);

        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(event.pos.x(), event.pos.y())
            .rotation(event.rot)
            .linvel(vel.x(), vel.y());

        commands
            .spawn(SpriteComponents {
                material: assets.projectile_texture,
                translation: event.pos,
                rotation: Rotation::from_rotation_z(event.rot),
                ..Default::default()
            })
            .with_bundle(ProjectileComponents {
                // rot: Rot(event.rot),
                // velocity: Velocity(Vec3::new(vel.x(), vel.y(), 0.0)),
                rigid_body,
                ..Default::default()
            });

        // Play audio
        audio_output.play(assets.weapon_fire);
    }
}
