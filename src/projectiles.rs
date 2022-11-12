use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    common::{self, Collider},
    obstacles::Obstacle,
    state::GameState,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub damage: f32,
    pub direction: Vec2,
    pub duration: Timer,
}

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(Self::move_bullets)
                .with_system(Self::process_collisions.after(Self::move_bullets))
                .with_system(Self::remove_bullets.after(Self::move_bullets)),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Level)
                .with_system(Self::remove_bullets),
        );
    }
}

impl ProjectilesPlugin {
    fn move_bullets(
        mut bullet_query: Query<(
            &mut Transform,
            &mut Bullet,
            &mut Collider,
            &common::Velocity,
        )>,
        time: Res<Time>,
    ) {
        for (mut transform, mut bullet, mut collider, velocity) in
            bullet_query.iter_mut()
        {
            let dt = time.delta_seconds();

            bullet.duration.tick(time.delta());

            transform.translation.x += bullet.direction.x * velocity.vx * dt;
            transform.translation.y += bullet.direction.y * velocity.vy * dt;

            let bullet_size =
                common::ease_in_out_sine(bullet.duration.percent_left());
            transform.scale = Vec3::new(bullet_size, bullet_size, 0.);

            collider.offset = Some(Vec2::new(
                (collider.width * bullet_size) / 2.,
                (collider.height * bullet_size) / 2.,
            ));
        }
    }

    fn process_collisions(
        bullet_query: Query<(Entity, &Collider, &Transform), With<Bullet>>,
        obstacle_query: Query<(Entity, &Collider, &Transform), With<Obstacle>>,
        mut commands: Commands,
    ) {
        for (e, collider, transform) in bullet_query.iter() {
            Self::handle_collisions(
                e,
                transform,
                collider,
                &obstacle_query,
                &mut commands,
            );
        }
    }

    fn handle_collisions(
        bullet: Entity,
        bullet_transform: &Transform,
        bullet_collider: &Collider,
        obstacle_query: &Query<(Entity, &Collider, &Transform), With<Obstacle>>,
        commands: &mut Commands,
    ) {
        for (obstacle, collider, transform) in obstacle_query.iter() {
            let collision = collide(
                bullet_transform.translation
                    + bullet_collider.offset.unwrap().extend(0.),
                (*bullet_collider).into(),
                transform.translation,
                collider.clone().into(),
            )
            .is_some();

            if collision {
                commands.entity(obstacle).despawn_recursive();
                commands.entity(bullet).despawn_recursive();
                break;
            }
        }
    }

    fn remove_bullets(
        mut commands: Commands,
        bullets_query: Query<(Entity, &Bullet)>,
    ) {
        for (e, bullet) in bullets_query.iter() {
            if bullet.duration.finished() {
                commands.entity(e).despawn_recursive();
            }
        }
    }
}
