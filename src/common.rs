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
