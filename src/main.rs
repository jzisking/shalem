mod player;

use crate::player::PlayerPlugin;
use bevy::prelude::*;

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
            PlayerPlugin,
        ))
        .run();
}
