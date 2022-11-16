use bevy::prelude::*;

use crate::{HEIGHT, WIDTH};

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

pub fn ease_out_sine(x: f32) -> f32 {
    ((x * std::f32::consts::PI) / 2.).sin()
}

pub fn ease_in_out_sine(x: f32) -> f32 {
    -((std::f32::consts::PI * x).cos() - 1.) / 2.
}

pub fn out_of_bounds_x(x: f32, width: f32) -> bool {
    x + width >= WIDTH || x - width < 0.
}

pub fn out_of_bounds_y(y: f32, height: f32) -> bool {
    y + height >= HEIGHT || y - height < 0.
}
