use crate::item_registry::ItemRegistry;
use crate::player::PlayerComponent;
use bevy::prelude::*;

#[derive(Component)]
pub struct PickableItemComponent {
    ident: String,
    amount: u32,
}

pub fn spawn_pickable_item(
    commands: &mut Commands,
    asset_server: &AssetServer,
    item_registry: &ItemRegistry,
    ident: &str,
    amount: u32,
    pos: Vec2,
    scale: Vec2,
) {
    let item = item_registry.item(ident).unwrap();

    let texture = asset_server.load(&item.texture);

    let mut transform = Transform::default();
    transform.translation = Vec3::new(pos.x, pos.y, 0.);
    transform.scale = Vec3::new(scale.x, scale.y, 1.);

    commands
        .spawn(SpriteBundle {
            texture,
            transform,
            ..Default::default()
        })
        .insert(PickableItemComponent {
            ident: ident.to_owned(),
            amount: amount.min(item.max_stack_size),
        });
}

pub fn collect_pickable_items(
    mut commands: Commands,
    query: Query<(Entity, &PickableItemComponent, &Transform)>,
    player: Query<&Transform, With<PlayerComponent>>,
) {
    let player_transform = player.single();

    for (entity, pickable, transform) in query.iter() {
        let distance = player_transform.translation.distance(transform.translation);
        
        if distance < 60. {
            commands.entity(entity).despawn();
        }
    }
}
