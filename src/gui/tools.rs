use std::fmt::Display;

use bevy::prelude::*;
use bevy_egui::egui;

use crate::camera::CursorWorldCoords;
use crate::particle::spawners::SpawnRandomParticles;
use crate::particle::{self, ParticleBundle};

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
    max_random_velocity: f32,
    amount: u32,
    inner_radius: f32,
    outer_radius: f32,
}

impl Tool {
    fn ui(&self, state: &mut ToolState, ui: &mut egui::Ui) {
        egui::Grid::new("tool_grid")
            .num_columns(3)
            .striped(false)
            .show(ui, |ui| match self {
                Tool::SpawnParticle => {
                    state.mass_ui(ui);
                    state.radius_ui(ui);
                }
                Tool::SpawnRandomParticles => {
                    state.mass_ui(ui);
                    state.radius_ui(ui);
                    state.amount_ui(ui);
                    state.max_random_velocity_ui(ui);
                    state.inner_radius_ui(ui);
                    state.outer_radius_ui(ui);
                }
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
        selected_tool.ui(self, ui);
    }

    /// spawn particle using the config
    fn spawn_particle(&self, commands: &mut Commands) {
        ParticleBundle::new()
            .radius(self.radius)
            .mass(self.mass)
            .position(self.position)
            .velocity(self.velocity)
            .spawn(commands);
    }

    /// spawn random particles using the config in the state
    fn spawn_random_particles(&self, commands: &mut Commands) {
        SpawnRandomParticles::new()
            .position(self.position)
            .radius(self.radius)
            .mass(self.mass)
            .velocity(self.max_random_velocity)
            .inner_radius(self.inner_radius)
            .outer_radius(self.outer_radius)
            .amount(self.amount)
            .spawn(commands);
    }

    /// gizmo preview for random particles tool
    fn preview_random_particles(&self, gizmos: &mut Gizmos, cursor_coords: Vec2) {
        gizmos.circle_2d(cursor_coords, self.inner_radius, Color::WHITE);
        gizmos.circle_2d(cursor_coords, self.outer_radius, Color::WHITE);
    }

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
        self.radius = self.radius.max(0.0);
    }

    fn inner_radius_ui(&mut self, ui: &mut egui::Ui) {
        value_editor_row(
            ui,
            &mut self.inner_radius,
            1.0,
            "inner radius",
            "inner radius of random particle spawning",
        );
        self.inner_radius = self.inner_radius.max(0.0);
    }

    fn outer_radius_ui(&mut self, ui: &mut egui::Ui) {
        value_editor_row(
            ui,
            &mut self.outer_radius,
            1.0,
            "outer radius",
            "outer radius of random particle spawning",
        );
        self.outer_radius = self.outer_radius.max(self.inner_radius)
    }

    fn max_random_velocity_ui(&mut self, ui: &mut egui::Ui) {
        value_editor_row(
            ui,
            &mut self.max_random_velocity,
            0.01,
            "max velocity",
            "maximum value of random velociteis",
        );
        self.max_random_velocity = self.max_random_velocity.max(0.0);
    }

    fn amount_ui(&mut self, ui: &mut egui::Ui) {
        let mut amount_f32 = self.amount as f32;
        value_editor_row(
            ui,
            &mut amount_f32,
            5.0,
            "amount",
            "amount of particles to spawn",
        );
        self.amount = amount_f32 as u32;
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
            radius: 5.0,
            max_random_velocity: 0.0,
            amount: 100,
            inner_radius: 0.0,
            outer_radius: 100.0,
        }
    }
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

    if !pressed {
        match tool_state.selected_tool {
            Tool::SpawnParticle => {
                gizmos.circle_2d(cursor_coords, tool_state.radius, Color::WHITE);
            }
            Tool::SpawnRandomParticles => {
                tool_state.preview_random_particles(&mut gizmos, cursor_coords)
            }
        }
    }

    if just_pressed {
        match tool_state.selected_tool {
            Tool::SpawnParticle => tool_state.position = cursor_coords,
            Tool::SpawnRandomParticles => {
                tool_state.position = cursor_coords;
                tool_state.spawn_random_particles(&mut commands)
            }
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
            Tool::SpawnRandomParticles => {
                tool_state.preview_random_particles(&mut gizmos, cursor_coords);
            }
        }
    }

    if just_released {
        match tool_state.selected_tool {
            Tool::SpawnParticle => tool_state.spawn_particle(&mut commands),
            Tool::SpawnRandomParticles => {
                tool_state.preview_random_particles(&mut gizmos, cursor_coords);
            }
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
