mod common;
mod enemies;
mod graphics;
mod obstacles;
mod player;
mod projectiles;
mod ray;
mod state;

#[cfg(debug_assertions)]
mod debug;
mod utils;

use bevy::{
    prelude::*,
    render::{camera::WindowOrigin, texture::ImageSettings},
    window::WindowMode,
};
use enemies::EnemiesPlugin;
use graphics::GraphicsPlugin;
use obstacles::ObstaclePlugin;
use player::PlayerPlugin;
use projectiles::ProjectilesPlugin;
use state::GameState;

#[cfg(debug_assertions)]
use debug::DebugPlugin;

pub const HEIGHT: f32 = 720.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const WIDTH: f32 = HEIGHT * RESOLUTION;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::hex(graphics::BG_COLOR).unwrap()))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Shooter".to_string(),
            resizable: false,
            mode: WindowMode::Windowed,
            scale_factor_override: Some(1.0),
            ..Default::default()
        })
        .add_state(GameState::Level)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_plugin(GraphicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilesPlugin)
        .add_plugin(ObstaclePlugin)
        .add_plugin(EnemiesPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.window_origin = WindowOrigin::BottomLeft;

    commands.spawn_bundle(camera);
}
