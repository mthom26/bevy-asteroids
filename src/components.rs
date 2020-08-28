use bevy::math::Vec3;

pub struct Player;

pub struct Weapon {
    pub reload_speed: f32,
    pub reload_timer: f32,
}

// Marker to wrap an object around the screen edges
pub struct ScreenWrap;

// Marker to check if an object has left the screen
pub struct ScreenCheck;

