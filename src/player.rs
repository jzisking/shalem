use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Update, (movement, camera_follow));
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

fn camera_follow(
    query: Query<&Transform, With<PlayerComponent>>,
    mut query_camera: Query<&mut Transform, (Without<PlayerComponent>, With<Camera>)>,
) {
    for player_transform in query.iter() {
        for mut camera_transform in query_camera.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
