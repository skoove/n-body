use bevy_egui::egui;

use crate::simulation::SimSettings;

impl SimSettings {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("clear all particles").clicked() {
            self.should_clear_all_particles = true;
        }

        match self.paused {
            true => {
                if ui.button("⏸").clicked() {
                    self.toggle_pause();
                }
            }
            false => {
                if ui.button("▶").clicked() {
                    self.toggle_pause();
                }
            }
        }

        egui::Grid::new("settings_grid")
            .striped(true)
            .show(ui, |ui| {
                ui.label("collisions");
                ui.checkbox(&mut self.enable_collisions, "");
                ui.end_row();

                ui.label("gravity constant");
                ui.add(egui::DragValue::new(&mut self.gravity_constant).speed(1.0));
                self.gravity_constant = self.gravity_constant.max(0.0);
                ui.end_row();

                ui.label("collision steps");
                ui.add(egui::DragValue::new(&mut self.collision_steps).speed(0.1));
                self.collision_steps = self.collision_steps.max(1)
            });
    }
}
