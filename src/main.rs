mod bush;
mod debug;
mod inventory;
mod item_registry;
mod pickable_item;
mod player;

use crate::bush::spawn_bush;
use crate::debug::DebugPlugin;
use crate::inventory::InventoryPlugin;
use crate::item_registry::{ItemDesc, ItemRegistry};
use crate::pickable_item::{collect_pickable_items, spawn_pickable_item};
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
            InventoryPlugin,
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, collect_pickable_items)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut item_registry = ItemRegistry::default();

    item_registry.register_item(
        "bottle",
        ItemDesc {
            name: "Bottle".to_string(),
            texture: "bottle.png".to_string(),
            max_stack_size: 16,
        },
    );

    spawn_pickable_item(
        &mut commands,
        &asset_server,
        &item_registry,
        "bottle",
        1,
        Vec2::new(-200., -300.),
        Vec2::ONE,
    );

    spawn_pickable_item(
        &mut commands,
        &asset_server,
        &item_registry,
        "bottle",
        1,
        Vec2::new(300., -50.),
        Vec2::ONE,
    );

    commands.insert_resource(item_registry);

    spawn_bush(&mut commands, &asset_server, Vec2::new(250., 0.));
}
