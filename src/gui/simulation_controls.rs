use crate::SimSettings;
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
        ui.checkbox(&mut sim_settings.paused, "paused")
    });
}
