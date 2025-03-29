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
        let mut pause_button = ui.button("pause/play");
        pause_button = if !sim_settings.paused {
            pause_button.highlight()
        } else {
            pause_button
        };
        if pause_button.clicked() {
            debug!("pause button clicked");
            sim_settings.toggle_pause();
        }

        ui.add(
            egui::Slider::new(&mut sim_settings.gravity_constant, 0.0..=100000.0)
                .text("gravity constant"),
        )
    });
}
