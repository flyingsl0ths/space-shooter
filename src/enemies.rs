use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{
    common::Collider, graphics::TexturesSheets, obstacles::Obstacle,
    player::Player, ray, state::GameState, HEIGHT, WIDTH,
};

#[derive(Component)]
pub struct Enemy {
    velocity: f32,
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Level)
                .with_system(Self::setup_enemies),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(Self::follow_player)
                .with_system(
                    Self::destroy_obstacles.after(Self::follow_player),
                ),
        );
    }
}

impl EnemiesPlugin {
    fn setup_enemies(mut commands: Commands, ts: Res<TexturesSheets>) {
        let mut rng = rand::thread_rng();
        let total: u32 = rng.gen_range(3..=10);

        for _ in 0..total {
            let sprite_index = rng.gen_range(0..=7);

            let collider = Self::enemy_collider(sprite_index).unwrap();

            let sprite = TextureAtlasSprite::new(sprite_index as usize);

            let x = rng.gen_range(0..(WIDTH as u32)) as f32;

            let enemy_height = collider.height as u32;

            let y = match rng.gen() {
                0 => 0. - (rng.gen_range(0..enemy_height) as f32),
                _ => HEIGHT + (rng.gen_range(0..enemy_height) as f32),
            };

            let transform = Transform::from_xyz(x, y, 1.);

            let enemy = Self::to_enemy(sprite_index, &mut rng).unwrap();

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite,
                    texture_atlas: ts.entities.clone(),
                    transform,
                    ..Default::default()
                })
                .insert(collider)
                .insert(enemy);
        }
    }

    fn follow_player(
        mut enemies_query: Query<(&mut Transform, &Enemy)>,
        player_query: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
        time: Res<Time>,
    ) {
        let player_transform = player_query.single();
        let player_pos = player_transform.translation;

        for (mut transform, enemy) in enemies_query.iter_mut() {
            let enemy_pos = transform.translation;

            let dist = player_pos.truncate() - enemy_pos.truncate();
            let angle = Vec2::new(0.0, 1.0).angle_between(dist);

            let enemy_pos =
                dist.normalize() * enemy.velocity * time.delta_seconds();

            transform.rotation = Quat::from_rotation_z(angle);
            transform.translation += enemy_pos.extend(0.);
        }
    }

    fn enemy_collider(sprite_index: u32) -> Option<Collider> {
        match sprite_index {
            0..=3 => Some(Collider {
                height: 92.,
                width: 91.,
                offset: None,
            }),
            4..=7 => Some(Collider {
                height: 83.,
                width: 93.,
                offset: None,
            }),
            _ => None,
        }
    }

    fn to_enemy(sprite_index: u32, rng: &mut ThreadRng) -> Option<Enemy> {
        let velocity = 100..=200;

        match sprite_index {
            0..=3 => Some(Enemy {
                velocity: rng.gen_range(velocity) as f32,
            }),
            4..=7 => Some(Enemy {
                velocity: rng.gen_range(velocity) as f32,
            }),
            _ => None,
        }
    }

    fn destroy_obstacles(
        enemies_query: Query<&Transform, With<Enemy>>,
        obstacles_query: Query<(&Transform, &Collider, Entity), With<Obstacle>>,
        mut commands: Commands,
    ) {
        for transform in enemies_query.iter() {
            let enemy_pos = transform.translation.truncate();

            for (obs_transform, obs_coll, obs) in obstacles_query.iter() {
                let obs_pos = obs_transform.translation.truncate();

                let y = obs_pos.y - (obs_coll.height / 2.);

                let left = Vec2::new(obs_pos.x - (obs_coll.width / 2.), y);

                let right = Vec2::new(obs_pos.x + (obs_coll.width / 2.), y);

                if ray::casted((left, right), enemy_pos) {
                    commands.entity(obs).despawn_recursive();
                }
            }
        }
    }
}
