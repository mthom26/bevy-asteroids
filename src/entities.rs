use bevy::{ecs::Bundle, prelude::*};
use bevy_rapier2d::rapier::{dynamics::{RigidBodyBuilder}, geometry::ColliderBuilder};

use crate::{components::*, DENSITY};

#[derive(Bundle)]
pub struct PlayerComponents {
    // pub velocity: Velocity,
    // pub rot: Rot,
    // pub angular_velocity: AngularVelocity,
    // pub drag: Drag,
    pub screen_wrap: ScreenWrap,
    pub player: Player,
    pub weapon: Weapon,
    // pub collider: Collider,
    // pub collider_kind: ColliderKind,
    pub rigid_body: RigidBodyBuilder,
    pub collider: ColliderBuilder,
}

impl Default for PlayerComponents {
    fn default() -> Self {
        Self {
            // velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            // rot: Rot(0.0),
            // angular_velocity: AngularVelocity(0.0),
            // drag: Drag::new(0.4, 0.4),
            screen_wrap: ScreenWrap,
            player: Player,
            weapon: Weapon {
                reload_speed: 1.0,
                reload_timer: 0.0,
            },
            // collider: Collider { radius: 50.0 },
            // collider_kind: ColliderKind(ColliderType::Player),
            rigid_body: RigidBodyBuilder::new_dynamic().translation(0.0, 0.0).can_sleep(false),
            collider: ColliderBuilder::ball(55.0).density(DENSITY),
        }
    }
}

#[derive(Bundle)]
pub struct AsteroidComponents {
    // pub velocity: Velocity,
    // pub rot: Rot,
    // pub angular_velocity: AngularVelocity,
    pub screen_check: ScreenCheck,
    // pub collider: Collider,
    // pub collider_kind: ColliderKind,
    pub rigid_body: RigidBodyBuilder,
    pub collider: ColliderBuilder,
}

impl Default for AsteroidComponents {
    fn default() -> Self {
        Self {
            // velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            // rot: Rot(0.0),
            // angular_velocity: AngularVelocity(0.0),
            screen_check: ScreenCheck,
            // collider: Collider { radius: 50.0 },
            // collider_kind: ColliderKind(ColliderType::Asteroid),
            rigid_body: RigidBodyBuilder::new_dynamic().translation(0.0, 0.0).can_sleep(false),
            collider: ColliderBuilder::ball(55.0).density(DENSITY),
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileComponents {
    // pub velocity: Velocity,
    // pub rot: Rot,
    // pub angular_velocity: AngularVelocity,
    pub screen_check: ScreenCheck,
    // pub collider: Collider,
    // pub collider_kind: ColliderKind,
}

impl Default for ProjectileComponents {
    fn default() -> Self {
        Self {
            // velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            // rot: Rot(0.0),
            // angular_velocity: AngularVelocity(0.0),
            screen_check: ScreenCheck,
            // collider: Collider { radius: 10.0 },
            // collider_kind: ColliderKind(ColliderType::Projectile),
        }
    }
}
