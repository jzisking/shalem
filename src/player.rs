use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Update, movement);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("player.png"),
            ..Default::default()
        })
        .insert(PlayerComponent);
}

const SPEED: f32 = 500.0;
fn movement(
    mut query: Query<&mut Transform, With<PlayerComponent>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform: Mut<Transform> = query.single_mut();

    if keys.pressed(KeyCode::W) {
        transform.translation.y += SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S) {
        transform.translation.y -= SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::A) {
        transform.translation.x -= SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::D) {
        transform.translation.x += SPEED * time.delta_seconds();
    }
}
