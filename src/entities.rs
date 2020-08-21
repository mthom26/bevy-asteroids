use bevy::{ecs::Bundle, prelude::*};

use crate::components::*;

#[derive(Bundle)]
pub struct PlayerComponents {
    pub velocity: Velocity,
    pub rot: Rot,
    pub angular_velocity: AngularVelocity,
    pub drag: Drag,
    pub screen_wrap: ScreenWrap,
    pub player: Player,
    pub weapon: Weapon,
}

impl Default for PlayerComponents {
    fn default() -> Self {
        Self {
            velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            rot: Rot(0.0),
            angular_velocity: AngularVelocity(0.0),
            drag: Drag::new(0.4, 0.4),
            screen_wrap: ScreenWrap,
            player: Player,
            weapon: Weapon {
                reload_speed: 1.0,
                reload_timer: 0.0,
            }
        }
    }
}

#[derive(Bundle)]
pub struct AsteroidComponents {
    pub velocity: Velocity,
    pub rot: Rot,
    pub angular_velocity: AngularVelocity,
    pub screen_check: ScreenCheck,
}

impl Default for AsteroidComponents {
    fn default() -> Self {
        Self {
            velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            rot: Rot(0.0),
            angular_velocity: AngularVelocity(0.0),
            screen_check: ScreenCheck,
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileComponents {
    pub velocity: Velocity,
    pub rot: Rot,
    pub angular_velocity: AngularVelocity,
    pub screen_check: ScreenCheck,
}

impl Default for ProjectileComponents {
    fn default() -> Self {
        Self {
            velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            rot: Rot(0.0),
            angular_velocity: AngularVelocity(0.0),
            screen_check: ScreenCheck,
        }
    }
}
