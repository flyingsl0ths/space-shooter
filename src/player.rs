use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{
    common::{Collider, Velocity},
    graphics::TexturesSheets,
    obstacles::Obstacle,
    projectiles::Bullet,
    state::GameState,
    HEIGHT, WIDTH,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub just_moved: bool,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Cursor {
    actual_angle: f32,
    computed_angle: f32,
    fired: bool,
    last_target_pos: Vec2,
    rate: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Level)
                .with_system(Self::spawn_cursor)
                .with_system(Self::spawn_player),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(Self::process_input)
                .with_system(Self::process_mouse_movement)
                .with_system(Self::process_mouse_input)
                .with_system(
                    Self::cursor_fire_cooldown.after(Self::process_mouse_input),
                ),
        );
    }
}

impl PlayerPlugin {
    fn spawn_cursor(
        mut commands: Commands,
        mut windows: ResMut<Windows>,
        ts: Res<TexturesSheets>,
    ) {
        let window = windows.get_primary_mut().unwrap();

        window.set_cursor_visibility(false);

        let cursor_pos = Vec2::new(WIDTH / 2., HEIGHT / 2.);

        window.set_cursor_position(cursor_pos);

        let sprite = TextureAtlasSprite::new(11);

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: ts.ui.clone(),
                transform: Transform::from_xyz(
                    cursor_pos.x,
                    cursor_pos.y + 95.,
                    2.,
                ),
                ..Default::default()
            })
            .insert(Name::new("Cursor"))
            .insert(Cursor {
                last_target_pos: cursor_pos,
                rate: Timer::from_seconds(0.48, true),
                ..Default::default()
            });
    }

    fn spawn_player(mut commands: Commands, ts: Res<TexturesSheets>) {
        let sprite = TextureAtlasSprite::new(8);
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: ts.entities.clone(),
                transform: Transform::from_xyz(WIDTH / 2., HEIGHT / 2., 1.),
                ..Default::default()
            })
            .insert(Velocity { vx: 325., vy: 325. })
            .insert(Collider {
                width: 99.,
                height: 75.,
                ..Default::default()
            })
            .insert(Player { just_moved: true })
            .insert(Name::new("Player"));
    }

    fn cursor_fire_cooldown(
        mut cursor_query: Query<&mut Cursor>,
        time: Res<Time>,
    ) {
        let mut cursor = cursor_query.single_mut();
        if cursor.fired {
            cursor.rate.tick(time.delta());
            if cursor.rate.finished() {
                cursor.fired = false;
            }
        }
    }

    fn process_input(
        keyboard: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut cursor_query: Query<&mut Cursor>,
        mut player_query: Query<(
            &mut Transform,
            &Velocity,
            &Collider,
            &mut Player,
        )>,
        obstacles_query: Query<
            (&Transform, &Collider),
            (Without<Player>, With<Obstacle>),
        >,
    ) {
        let mut cursor = cursor_query.single_mut();

        Self::handle_input(
            &keyboard,
            time.delta_seconds(),
            &mut player_query,
            &obstacles_query,
        );

        let (transform, ..) = player_query.single();

        cursor.last_target_pos = transform.translation.truncate();
    }

    fn handle_input(
        keyboard: &Input<KeyCode>,
        dt: f32,
        player_query: &mut Query<(
            &mut Transform,
            &Velocity,
            &Collider,
            &mut Player,
        )>,
        obstacles_query: &Query<
            (&Transform, &Collider),
            (Without<Player>, With<Obstacle>),
        >,
    ) {
        let (mut transform, velocity, collider, mut player) =
            player_query.single_mut();

        player.just_moved = false;

        let mut offset_y = 0.;
        let mut offset_x = 0.;

        let distance_y = velocity.vy * dt;
        let distance_x = velocity.vx * dt;

        if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
            offset_y += distance_y
        }

        if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            offset_y -= distance_y
        }

        if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            offset_x -= distance_x;
        }

        if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            offset_x += distance_x;
        }

        let target = transform.translation + Vec3::new(offset_x, 0., 0.);
        if !Self::check_collisions(target, *collider, &obstacles_query) {
            transform.translation = target;
            if offset_x != 0. {
                player.just_moved = true;
            }
        }

        let target = transform.translation + Vec3::new(0., offset_y, 0.);
        if !Self::check_collisions(target, *collider, &obstacles_query) {
            transform.translation = target;
            if offset_y != 0. {
                player.just_moved = true;
            }
        }
    }

    fn check_collisions(
        target_pos: Vec3,
        target_collider: Collider,
        obstacles_query: &Query<
            (&Transform, &Collider),
            (Without<Player>, With<Obstacle>),
        >,
    ) -> bool {
        for (transform, collider) in obstacles_query.iter() {
            let collision = collide(
                target_pos,
                target_collider.into(),
                transform.translation,
                collider.clone().into(),
            )
            .is_some();

            if collision {
                return true;
            }
        }
        false
    }

    fn process_mouse_input(
        buttons: Res<Input<MouseButton>>,
        ts: Res<TexturesSheets>,
        mut cursor_query: Query<&mut Cursor>,
        input_target_query: Query<&Collider, With<Player>>,
        mut commands: Commands,
    ) {
        let mut cursor = cursor_query.single_mut();

        if !cursor.fired && buttons.pressed(MouseButton::Left) {
            cursor.fired = true;

            let damage = 10.;

            let sprite = TextureAtlasSprite::new(0);

            let target_collider = input_target_query.single();

            let direction = Vec2::new(
                cursor.computed_angle.cos(),
                cursor.computed_angle.sin(),
            );

            let mut transform: Transform = Transform::from_xyz(
                cursor.last_target_pos.x
                    + direction.x * (target_collider.width / 2.),
                cursor.last_target_pos.y
                    + direction.y * (target_collider.height / 2.),
                1.,
            );

            transform.rotation = Quat::from_rotation_z(cursor.actual_angle);

            let width = 9.;
            let height = 37.;

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite,
                    texture_atlas: ts.projectiles.clone(),
                    transform,
                    ..Default::default()
                })
                .insert(Bullet {
                    damage,
                    direction,
                    duration: Timer::from_seconds(1.7, false),
                })
                .insert(Collider {
                    height,
                    width,
                    offset: Some(Vec2::new(width / 2., height / 2.)),
                })
                .insert(Velocity { vx: 200., vy: 200. })
                .insert(Name::new("Bullet"));
        }
    }

    fn process_mouse_movement(
        mut cursor_query: Query<(&mut Transform, &mut Cursor)>,
        mut input_target_query: Query<
            &mut Transform,
            (Without<Cursor>, With<Player>),
        >,
        mut cursor_evr: EventReader<CursorMoved>,
    ) {
        for ev in cursor_evr.iter() {
            Self::handle_mouse_movement(
                &mut cursor_query,
                &mut input_target_query,
                ev.position,
            );
        }
    }

    fn handle_mouse_movement(
        cursor_query: &mut Query<(&mut Transform, &mut Cursor)>,
        input_target_query: &mut Query<
            &mut Transform,
            (Without<Cursor>, With<Player>),
        >,
        mouse_pos: Vec2,
    ) {
        let (mut pos, mut cursor) = cursor_query.single_mut();
        pos.translation = mouse_pos.extend(2.);

        let mut target = input_target_query.single_mut();

        let target_pos = target.translation.truncate();
        cursor.last_target_pos = target_pos;

        cursor.computed_angle =
            f32::atan2(mouse_pos.y - target_pos.y, mouse_pos.x - target_pos.x);

        let diff = mouse_pos - target_pos;
        let y_axis = Vec2::new(0.0, 1.0);
        let angle = y_axis.angle_between(diff);
        cursor.actual_angle = angle;

        target.rotation = Quat::from_rotation_z(angle);
    }
}
