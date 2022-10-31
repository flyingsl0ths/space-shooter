use bevy::prelude::*;

pub const BG_COLOR: &'static str = "272034";

pub struct GraphicsPlugin;

#[derive(Clone)]
pub struct TexturesSheets {
    pub entities: Handle<TextureAtlas>,
    pub obstacles: Handle<TextureAtlas>,
    pub obstacles2: Handle<TextureAtlas>,
    pub projectiles: Handle<TextureAtlas>,
    pub ui: Handle<TextureAtlas>,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            load_texture_sheets,
        );
    }
}

fn load_texture_sheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let entities = asset_server.load("entities.png");
    let obstacles = asset_server.load("obstacles.png");
    let obstacles2 = asset_server.load("obstacles-2.png");
    let projectiles = asset_server.load("projectiles.png");
    let ui = asset_server.load("ui.png");

    let entities = TextureAtlas::from_grid(entities, Vec2::new(99., 91.), 1, 9);
    let obstacles =
        TextureAtlas::from_grid(obstacles, Vec2::new(120., 98.), 1, 4);
    let obstacles2 =
        TextureAtlas::from_grid(obstacles2, Vec2::new(45., 43.), 1, 6);
    let projectiles =
        TextureAtlas::from_grid(projectiles, Vec2::new(13., 37.), 1, 2);
    let ui = TextureAtlas::from_grid(ui, Vec2::new(33., 26.), 1, 12);

    let entities = texture_atlases.add(entities);
    let obstacles = texture_atlases.add(obstacles);
    let obstacles2 = texture_atlases.add(obstacles2);
    let projectiles = texture_atlases.add(projectiles);
    let ui = texture_atlases.add(ui);

    commands.insert_resource(TexturesSheets {
        entities,
        obstacles,
        obstacles2,
        projectiles,
        ui,
    });
}
