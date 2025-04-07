use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_gui);
    }
}

fn debug_gui(mut contexts: EguiContexts, mut commands: Commands) {
    egui::Window::new("debug").show(contexts.ctx_mut(), |ui| {
        if ui.button("build quadtree").clicked() {
            let id = commands.register_system(crate::simulation::quadtree::quadtree_system);
            // just to make logs easier to read when debugging
            print!("quadtree\nquadtree\nquadtree\nquadtree\nquadtree\n");
            commands.run_system(id);
        }
    });
}
