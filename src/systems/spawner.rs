use bevy::prelude::*;

use crate::{
    components::{AngularVelocity, Velocity},
    entities::AsteroidComponents,
    resources::{AssetHandles, AsteroidSpawnTimer},
};

pub fn spawn_asteroid_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<AssetHandles>,
    // windows: Res<Windows>,
    // asset_server: Res<AssetServer>,
    // materials: Res<Assets<ColorMaterial>>,
    mut timer: ResMut<AsteroidSpawnTimer>,
) {
    // let window = windows.get_primary();
    // println!("{:?}", window);
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        // Spawn asteroid
        commands
            .spawn(SpriteComponents {
                material: assets.asteroid_texture,
                translation: Translation::new(1280.0, 0.0, 1.0),
                rotation: Rotation::from_rotation_z(0.0),
                ..Default::default()
            })
            .with_bundle(AsteroidComponents {
                velocity: Velocity(Vec3::new(-250.0, 0.0, 0.0)),
                angular_velocity: AngularVelocity(-1.4),
                ..Default::default()
            });

        timer.0.reset();
    }
}
