use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::Rng;

use crate::{
    common::{Collider, Velocity},
    graphics::TexturesSheets,
    state::GameState,
    utils, HEIGHT, WIDTH,
};

#[derive(Component)]
pub struct Obstacle {
    pub can_split: bool,
}

#[derive(Component)]
pub struct MovingObstacle {
    direction: Vec2,
    duration: Timer,
    angle: f32,
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Level)
                .with_system(Self::setup_obstacles),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(Self::move_obstacles),
        );
    }
}

impl ObstaclePlugin {
    fn setup_obstacles(mut commands: Commands, ts: Res<TexturesSheets>) {
        let mut rng = rand::thread_rng();
        let total: u32 = rng.gen_range(5..=15);

        for _ in 0..total {
            let sprite_index = rng.gen_range(0..=3);

            let collider = Self::for_large_obstacle(sprite_index).unwrap();

            let width = collider.width as u32;
            let height = collider.height as u32;

            let sprite = TextureAtlasSprite::new(sprite_index);

            let x = rng.gen_range(width..((WIDTH as u32) - width)) as f32;
            let y = rng.gen_range(height..((HEIGHT as u32) - height)) as f32;

            let mut transform = Transform::from_xyz(x, y, 1.);

            let collision = collide(
                Vec3::new(WIDTH / 2., HEIGHT / 2., 1.),
                Vec2::new(99., 75.),
                transform.translation,
                collider.clone().into(),
            )
            .is_some();

            if collision {
                transform.translation.x -= (width + 10) as f32;
                transform.translation.y -= (height + 10) as f32;
            }

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite,
                    texture_atlas: ts.obstacles.clone(),
                    transform,
                    ..Default::default()
                })
                .insert(Obstacle { can_split: true })
                .insert(collider);
        }
    }

    fn move_obstacles(
        mut obstacles_query: Query<(
            &mut Transform,
            &mut MovingObstacle,
            &Velocity,
        )>,
        time: Res<Time>,
    ) {
        for (mut transform, mut mv_obs, velocity) in obstacles_query
            .iter_mut()
            .filter(|(_, mv_obs, _)| !mv_obs.duration.finished())
        {
            let MovingObstacle {
                direction,
                duration,
                angle,
            } = mv_obs.as_mut();

            duration.tick(time.delta());

            let dt = time.delta_seconds();

            *angle += *angle * dt;

            transform.rotation = Quat::from_rotation_z(*angle);

            transform.translation.x +=
                utils::ease_out_sine(direction.x * velocity.vx * dt);

            transform.translation.y +=
                utils::ease_out_sine(direction.y * velocity.vy * dt);
        }
    }

    fn for_large_obstacle(sprite_index: usize) -> Option<Collider> {
        match sprite_index {
            0 => Some(Collider {
                height: 98.0,
                width: 120.0,
                ..Default::default()
            }),

            1 => Some(Collider {
                height: 84.0,
                width: 101.0,
                ..Default::default()
            }),

            2 => Some(Collider {
                height: 82.0,
                width: 89.0,
                ..Default::default()
            }),

            3 => Some(Collider {
                height: 96.0,
                width: 98.0,
                ..Default::default()
            }),

            _ => None,
        }
    }

    fn for_small_obstacle(sprite_index: usize) -> Option<Collider> {
        match sprite_index {
            0 => Some(Collider {
                height: 43.0,
                width: 43.0,
                ..Default::default()
            }),

            1 => Some(Collider {
                height: 40.0,
                width: 45.0,
                ..Default::default()
            }),

            2 => Some(Collider {
                height: 28.0,
                width: 28.0,
                ..Default::default()
            }),

            3 => Some(Collider {
                height: 26.0,
                width: 29.0,
                ..Default::default()
            }),

            4 => Some(Collider {
                height: 18.0,
                width: 17.0,
                ..Default::default()
            }),

            5 => Some(Collider {
                height: 16.0,
                width: 15.0,
                ..Default::default()
            }),

            _ => None,
        }
    }
}

pub fn spawn_small_obstacles(
    commands: &mut Commands,
    ts: &TexturesSheets,
    initial_pos: Vec2,
    angle: f32,
) {
    let mut rng = rand::thread_rng();

    let total: u32 = rng.gen_range(3..=6);

    for _ in 0..total {
        let sprite_index = rng.gen_range(0..=5);

        let collider =
            ObstaclePlugin::for_small_obstacle(sprite_index).unwrap();

        let sprite = TextureAtlasSprite::new(sprite_index);

        let mut offset = rng.gen_range(1..=100) as f32 / 100.;

        let angle = angle + offset;

        let direction = Vec2::new(angle.cos(), angle.sin());

        let transform = Transform::from_xyz(
            initial_pos.x + offset,
            initial_pos.y + offset,
            1.,
        );

        offset += 1.;

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: ts.obstacles2.clone(),
                transform,
                ..Default::default()
            })
            .insert(MovingObstacle {
                direction,
                duration: Timer::from_seconds(offset, false),
                angle: offset,
            })
            .insert(Obstacle { can_split: false })
            .insert(Velocity { vx: 90., vy: 90. })
            .insert(collider);
    }
}
