mod debug;
mod player;

use crate::player::PlayerPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::debug::DebugPlugin;

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
            DebugPlugin
        ))
        .run();
}
