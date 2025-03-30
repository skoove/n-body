use crate::simulation::SimSettings;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct SimulationControlsGuiPlugin;

impl Plugin for SimulationControlsGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sim_controls_gui);
    }
}

fn sim_controls_gui(mut contexts: EguiContexts, mut sim_settings: ResMut<SimSettings>) {
    egui::Window::new("simulation controls").show(contexts.ctx_mut(), |ui| {
        if sim_settings.paused {
            if ui.button("play").clicked() {
                sim_settings.toggle_pause()
            };
        } else if ui.button("pause").clicked() {
            sim_settings.toggle_pause();
        }

        // gravity constant
        ui.add(
            egui::Slider::new(&mut sim_settings.gravity_constant, 0.0..=10.0)
                .text("gravity constant"),
        );

        // collision substeps
        ui.checkbox(&mut sim_settings.enable_collisions, "enable collisions");
        if sim_settings.enable_collisions {
            ui.add(
                egui::Slider::new(&mut sim_settings.collision_substeps, 1..=16)
                    .text("collision substeps"),
            );
        }
    });
}
