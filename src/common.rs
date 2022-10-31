use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct Collider {
    pub height: f32,
    pub width: f32,
    pub offset: Option<Vec2>,
}

impl From<Collider> for Vec2 {
    fn from(c: Collider) -> Self {
        Vec2::new(c.width, c.height)
    }
}

// pub fn ease_out_sine(x: f32) -> f32 {
//     ((x * std::f32::consts::PI) / 2.).sin()
// }

pub fn ease_in_out_sine(x: f32) -> f32 {
    -((std::f32::consts::PI * x).cos() - 1.) / 2.
}
