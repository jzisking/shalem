mod bush;
mod debug;
mod player;

use crate::bush::spawn_bush;
use crate::debug::DebugPlugin;
use crate::player::PlayerPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Shalem".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            EguiPlugin,
            PlayerPlugin,
            DebugPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_bush(&mut commands, &asset_server, Vec2::new(250., 0.));
}
