use bevy::math::Vec2;

// Rotate a Vec2 by `angle` in radians
pub fn rotate_vec2(v: &Vec2, angle: f32) -> Vec2 {
    let s = angle.sin();
    let c = angle.cos();

    Vec2::new(v.x() * c - v.y() * s, v.y() * c + v.x() * s)
}

// Convert cursor pos to world position
// Hardcoded to 2.0 camera scale and 1280x720 window dimensions for now...
pub fn cursor_pos_to_world_pos(cursor_pos: &Vec2) -> Vec2 {
    let x = cursor_pos.x() * 2.0 - 1280.0;
    let y = cursor_pos.y() * 2.0 - 720.0;
    Vec2::new(x, y)
}
