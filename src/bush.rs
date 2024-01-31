use bevy::prelude::*;
pub fn spawn_bush(commands: &mut Commands, asset_server: &AssetServer, position: Vec2) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bush.png"),
        transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
        ..Default::default()
    });
}
