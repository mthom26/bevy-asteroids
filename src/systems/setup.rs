use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, physics::Gravity};

use crate::{
    components::*,
    entities::{AsteroidComponents, PlayerComponents},
    resources::{ArenaData, AssetHandles},
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gravity: ResMut<Gravity>,
    // windows: Res<Windows>,
) {
    // This window is `None` in the startup system but not in regular systems
    // let window = windows.get_primary();
    // println!("{:?}", window);

    let player_texture: Handle<Texture> = asset_server
        .load("assets/textures/player.png")
        .expect("Could not load player texture");
    let asteroid_texture: Handle<Texture> = asset_server
        .load("assets/textures/asteroid.png")
        .expect("Could not load asteroid texture");
    let projectile_texture: Handle<Texture> = asset_server
        .load("assets/textures/projectile.png")
        .expect("Could not load projectile texture");

    gravity.0 = Vector2::new(0.0, 0.0);

    commands.insert_resource(AssetHandles {
        // player_texture: materials.add(player_texture.into()),
        asteroid_texture: materials.add(asteroid_texture.into()),
        projectile_texture: materials.add(projectile_texture.into()),
    });

    // The arena size is the screen size multiplied by 2.0 (camera scale)
    // centered on 0, 0.
    // TODO - Set this (and camera scale) by variables so it is easier to change
    commands.insert_resource(ArenaData {
        half_width: 1280.0,
        half_height: 720.0,
    });

    // Camera
    commands.spawn(Camera2dComponents {
        scale: 2.0.into(), // Setting this less than 1.0, nothing renders
        // translation: Translation::new(0.0, 0.0, 1000.0),
        ..Default::default()
    });

    // Spawn player
    commands
        .spawn(SpriteComponents {
            material: materials.add(player_texture.into()),
            translation: Translation::new(0.0, 0.0, 2.0),
            rotation: Rotation::from_rotation_z(0.0),
            ..Default::default()
        })
        .with_bundle(PlayerComponents {
            // drag: Drag::new(0.4, 0.0),
            ..Default::default()
        });

    // Test asteroid
    // commands.spawn(SpriteComponents {
    //     material: materials.add(asteroid_texture.into()),
    //     translation: Translation::new(0.0, 0.0, 2.0),
    //     rotation: Rotation::from_rotation_z(0.0),
    //     ..Default::default()
    // })
    // .with_bundle(AsteroidComponents {
    //     velocity: Velocity(Vec3::new(-100.0, -180.0, 0.0)),
    //     angular_velocity: AngularVelocity(0.4),
    //     ..Default::default()
    // });
}
