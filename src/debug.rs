use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub struct DebugPlugin;

#[derive(Resource)]
pub struct DebugState {
    enabled: bool,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugState {
            #[cfg(debug_assertions)]
            enabled: true,
            #[cfg(not(debug_assertions))]
            enabled: false,
        }).add_systems(Update, gui);
    }
}

fn gui(mut contexts: EguiContexts) {
    egui::Window::new("Shalem Debug menu").show(contexts.ctx(), |ui| {
        ui.label("This is a test");
    });
}