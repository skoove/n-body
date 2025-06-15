use std::fmt::Display;

use bevy::prelude::*;
use bevy_egui::egui;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Tool {
    SpawnParticle,
    SpawnRandomParticles,
}

#[derive(Resource)]
pub struct ToolState {
    selected_tool: Tool,
    velocity: Vec2,
    mass: f32,
    radius: f32,
}

impl Tool {
    fn ui(&self, state: &mut ToolState, ui: &mut egui::Ui, commands: &mut Commands) {
        egui::Grid::new("tool_grid")
            .num_columns(3)
            .striped(false)
            .show(ui, |ui| match self {
                Tool::SpawnParticle => {
                    state.radius_ui(ui);
                    state.mass_ui(ui);
                }
                Tool::SpawnRandomParticles => todo!(),
            });
    }
}

impl ToolState {
    pub fn ui(&mut self, ui: &mut egui::Ui, commands: &mut Commands) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{}", self.selected_tool))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.selected_tool,
                    Tool::SpawnParticle,
                    format!("{}", Tool::SpawnParticle),
                );

                ui.selectable_value(
                    &mut self.selected_tool,
                    Tool::SpawnRandomParticles,
                    format!("{}", Tool::SpawnRandomParticles),
                )
            });

        let selected_tool = self.selected_tool;
        selected_tool.ui(self, ui, commands);
    }

    fn mass_ui(&mut self, ui: &mut egui::Ui) {
        value_editor_row(
            ui,
            &mut self.mass,
            1.0,
            "mass",
            "set the mass of the spawned particle",
        );
    }

    fn radius_ui(&mut self, ui: &mut egui::Ui) {
        value_editor_row(
            ui,
            &mut self.radius,
            1.0,
            "radius",
            "set the radius of the spawned particle",
        );
    }
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tool::SpawnParticle => write!(f, "spawn particle"),
            Tool::SpawnRandomParticles => write!(f, "spawn random particle"),
        }
    }
}

impl Default for ToolState {
    fn default() -> Self {
        ToolState {
            selected_tool: Tool::SpawnParticle,
            velocity: Vec2::ZERO,
            mass: 1000.0,
            radius: 10.0,
        }
    }
}

fn value_editor_row(ui: &mut egui::Ui, value: &mut f32, speed: f32, label: &str, hover_text: &str) {
    ui.label(label);

    ui.add(egui::DragValue::new(value).speed(speed))
        .on_hover_text_at_pointer(hover_text);

    ui.horizontal(|ui| {
        if ui.button("x0.01").clicked() {
            *value *= 0.01;
        }
        if ui.button("x0.1").clicked() {
            *value *= 0.1;
        }
        if ui.button("x10").clicked() {
            *value *= 10.0;
        }
        if ui.button("x100").clicked() {
            *value *= 100.0;
        }
    });

    ui.end_row();
}
