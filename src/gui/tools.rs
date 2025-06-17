use std::fmt::Display;

use bevy::{prelude::*, transform::commands};
use bevy_egui::egui;

use crate::{
    camera::CursorWorldCoords,
    particle::{self, Particle, ParticleBundle},
};

#[derive(PartialEq, Debug, Copy, Clone)]
enum Tool {
    SpawnParticle,
    SpawnRandomParticles,
}

#[derive(Resource)]
pub struct ToolState {
    selected_tool: Tool,
    position: Vec2,
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
    /// show the tool ui
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        commands: &mut Commands,
        particles: Query<Entity, With<particle::Particle>>,
    ) {
        if ui.button("clear particles").clicked() {
            particle::despawn_particles(commands, particles);
        }
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

    /// spawn particles using the config

    // functions for reusable ui widgets
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
            position: Vec2::ZERO,
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

/// Define actions for tools to do when clicking, dragging etc
pub fn tool_interactions_system(
    mut tool_state: ResMut<ToolState>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    cursor_coords: Res<CursorWorldCoords>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let cursor_coords = cursor_coords.0;

    let just_released = mouse_input.just_released(MouseButton::Left);
    let just_pressed = mouse_input.just_pressed(MouseButton::Left);
    let pressed = mouse_input.pressed(MouseButton::Left);

    // if lmb is not pressed
    if !pressed {
        match tool_state.selected_tool {
            Tool::SpawnParticle => {
                gizmos.circle_2d(cursor_coords, tool_state.radius, Color::WHITE);
            }
            Tool::SpawnRandomParticles => todo!(),
        }
    }

    // if lmb was pressed this frame
    if just_pressed {
        match tool_state.selected_tool {
            Tool::SpawnParticle => tool_state.position = cursor_coords,
            Tool::SpawnRandomParticles => (),
        }
    }

    if pressed {
        match tool_state.selected_tool {
            Tool::SpawnParticle => {
                let velocity = tool_state.position - cursor_coords;
                let arrow_end = tool_state.position + velocity;
                gizmos.circle_2d(tool_state.position, tool_state.radius, Color::WHITE);
                gizmos.arrow_2d(tool_state.position, arrow_end, Color::WHITE);
                tool_state.velocity = velocity * 0.05;
            }
            Tool::SpawnRandomParticles => todo!(),
        }
    }

    if just_released {
        match tool_state.selected_tool {
            Tool::SpawnParticle => {
                ParticleBundle::new()
                    .radius(tool_state.radius)
                    .mass(tool_state.mass)
                    .position(tool_state.position)
                    .velocity(tool_state.velocity)
                    .spawn(&mut commands);
            }
            Tool::SpawnRandomParticles => todo!(),
        }
    }
}
