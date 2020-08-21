use bevy::math::Vec3;

pub struct Player;

pub struct Weapon {
    pub reload_speed: f32,
    pub reload_timer: f32,
}

pub struct Rot(pub f32);

pub struct Velocity(pub Vec3);

pub struct AngularVelocity(pub f32);

pub struct Drag {
    pub linear: f32,
    pub angular: f32,
}

impl Drag {
    pub fn new(linear: f32, angular: f32) -> Self {
        Self { linear, angular }
    }
}

// Marker to wrap an object around the screen edges
pub struct ScreenWrap;

// Marker to check if an object has left the screen
pub struct ScreenCheck;
