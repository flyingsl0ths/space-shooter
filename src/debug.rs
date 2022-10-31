use crate::{common, player, projectiles};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_type::<player::Cursor>()
            .register_type::<projectiles::Bullet>()
            .register_type::<common::Collider>()
            .register_type::<common::Velocity>()
            .register_type::<player::Player>();
    }
}
