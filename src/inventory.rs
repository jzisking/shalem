use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(700.0),
                    height: Val::Px(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Px(10.)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for i in 0..9 {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(64.),
                                height: Val::Px(64.),
                                margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },

                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(asset_server.load("slot.png")),
                    ));
                }
            });
    });
}
