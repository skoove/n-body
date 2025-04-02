use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::camera::{self};
use crate::particle::{spawners, ParticleBundle};

pub struct ToolsGuiPlugin;

impl Plugin for ToolsGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tools_gui, tool_behaviours))
            .init_resource::<Tool>()
            .init_resource::<ToolSettings>();
    }
}

#[derive(Resource, PartialEq)]
enum Tool {
    SpawnParticle,
    SpawnRandom,
    SpawnHose,
    Attract,
    Repel,
}

#[derive(Resource)]
struct ToolSettings {
    mass: f32,
    radius: f32,
    velocity: Vec2,
    position: Vec2,

    inner_radius: f32,
    outer_radius: f32,
    amount_to_spawn: f32,
    scalar_velocity: f32,
    value_variation: bool,
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            mass: 10000.0,
            radius: 10.0,
            velocity: Vec2::ZERO,
            position: Vec2::ZERO,

            inner_radius: 0.0,
            outer_radius: 100.0,
            amount_to_spawn: 100.0,
            scalar_velocity: 0.0,
            value_variation: false,
        }
    }
}

impl Default for Tool {
    fn default() -> Self {
        Self::SpawnParticle
    }
}

impl std::fmt::Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tool::SpawnParticle => write!(f, "spawn particle"),
            Tool::SpawnRandom => write!(f, "spawn random particles"),
            Tool::SpawnHose => write!(f, "spawn particle hose"),
            Tool::Attract => write!(f, "attract particles"),
            Tool::Repel => write!(f, "repel particles"),
        }
    }
}

fn tools_gui(mut contexts: EguiContexts, mut tool: ResMut<Tool>, settings: ResMut<ToolSettings>) {
    egui::Window::new("tools").show(contexts.ctx_mut(), |ui| {
        egui::ComboBox::from_label("select a tool")
            .selected_text(format!("{}", *tool))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut *tool,
                    Tool::SpawnParticle,
                    format!("{}", Tool::SpawnParticle),
                )
                .on_hover_text("spawn a single particle");

                ui.selectable_value(
                    &mut *tool,
                    Tool::SpawnRandom,
                    format!("{}", Tool::SpawnRandom),
                )
                .on_hover_text("spawn random particles");

                ui.selectable_value(&mut *tool, Tool::SpawnHose, format!("{}", Tool::SpawnHose))
                    .on_hover_text("create a hose that makes a constant stream of particles");

                ui.selectable_value(&mut *tool, Tool::Attract, format!("{}", Tool::Attract))
                    .on_hover_text("attract particles");

                ui.selectable_value(&mut *tool, Tool::Repel, format!("{}", Tool::Repel))
                    .on_hover_text("repel particles");
            });
        match *tool {
            Tool::SpawnParticle => spawn_particle_gui(ui, settings),
            Tool::SpawnRandom => spawn_random_gui(ui, settings),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        }
    });
}

fn drag_value_with_multiply_buttons(
    ui: &mut egui::Ui,
    value: &mut f32,
    speed: f32,
    label: &str,
    hover_text: &str,
) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value).speed(speed))
            .on_hover_text_at_pointer(hover_text);
        ui.separator();
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
}

fn spawn_particle_gui(ui: &mut egui::Ui, mut settings: ResMut<ToolSettings>) {
    drag_value_with_multiply_buttons(
        ui,
        &mut settings.mass,
        50.0,
        "mass:",
        "mass of the spawned particle",
    );
    drag_value_with_multiply_buttons(
        ui,
        &mut settings.radius,
        0.1,
        "radius:",
        "radius of the spawned particle",
    );
}

fn spawn_random_gui(ui: &mut egui::Ui, mut settings: ResMut<ToolSettings>) {
    // make sure amount to spawn can be properly converted to u32
    settings.amount_to_spawn = settings.amount_to_spawn.abs().round();
    // make sure that the inner and outer radius are not greater or smaller than eachother, could panic otherwise
    settings.inner_radius = settings.inner_radius.min(settings.outer_radius);
    settings.outer_radius = settings.outer_radius.max(settings.inner_radius);
    drag_value_with_multiply_buttons(
        ui,
        &mut settings.amount_to_spawn,
        1.0,
        "amount:",
        "amount of particles to spawn",
    );
    drag_value_with_multiply_buttons(
        ui,
        &mut settings.mass,
        50.0,
        "mass:",
        "mass of the spawned particles",
    );
    drag_value_with_multiply_buttons(
        ui,
        &mut settings.scalar_velocity,
        50.0,
        "velocity range:",
        "velocity of particls will be between 0 and this value",
    );
    drag_value_with_multiply_buttons(
        ui,
        &mut settings.radius,
        0.1,
        "radius:",
        "radius of the spawned particles",
    );
    drag_value_with_multiply_buttons(
        ui,
        &mut settings.inner_radius,
        1.0,
        "inner radius:",
        "inner radius of the particle cloud, having this above zero makes a doughnut shape",
    );
    drag_value_with_multiply_buttons(
        ui,
        &mut settings.outer_radius,
        1.0,
        "outer radius:",
        "outer radius of the particle cloud",
    );
    ui.checkbox(&mut settings.value_variation, "value variation").on_hover_text_at_pointer("make the values of spawned particles different, meaning they will be lighter and darker than eachother");
}

/// things done when the left mouse button is released
fn tool_behaviours(
    mut commands: Commands,
    selected_tool: Res<Tool>,
    mut settings: ResMut<ToolSettings>,
    inputs: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<camera::CursorWorldCoords>,
    mut gizmos: Gizmos,
) {
    // for when the button is released, useually exectutes the function of the tool
    if inputs.just_released(MouseButton::Left) {
        match *selected_tool {
            Tool::SpawnParticle => {
                settings.velocity = (settings.position - cursor_coords.0) / 25.0;
                ParticleBundle::new()
                    .position(settings.position)
                    .velocity(settings.velocity)
                    .radius(settings.radius)
                    .mass(settings.mass)
                    .spawn(&mut commands)
            }
            Tool::SpawnRandom => spawners::SpawnRandomParticles::new()
                .position(cursor_coords.0)
                .velocity(settings.scalar_velocity)
                .outer_radius(settings.outer_radius)
                .inner_radius(settings.inner_radius)
                .amount(settings.amount_to_spawn as u32)
                .mass(settings.mass)
                .value_variation(settings.value_variation)
                .radius(settings.radius)
                .spawn(&mut commands),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        };
    };

    // for when the button has just been pressed, probably unused on most things
    if inputs.just_pressed(MouseButton::Left) {
        match *selected_tool {
            Tool::SpawnParticle => {
                settings.position = cursor_coords.0;
            }
            Tool::SpawnRandom => (),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        };
    };

    // for when the button is held down, probably unused on most things
    if inputs.pressed(MouseButton::Left) {
        match *selected_tool {
            Tool::SpawnParticle => {
                let direction = settings.position - cursor_coords.0;
                let arrow_end = settings.position + direction;
                gizmos.arrow_2d(settings.position, arrow_end, Color::WHITE);
                gizmos.circle_2d(
                    Isometry2d::new(settings.position, Rot2::IDENTITY),
                    settings.radius,
                    Color::WHITE,
                );
            }
            Tool::SpawnRandom => (),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        };
    };

    // for when the button is not pressed but the tool is selected, for stuff like previews
    if !inputs.pressed(MouseButton::Left) {
        match *selected_tool {
            Tool::SpawnParticle => {
                gizmos
                    .circle_2d(
                        Isometry2d::new(cursor_coords.0, Rot2::IDENTITY),
                        settings.radius,
                        Color::WHITE,
                    )
                    .resolution(32);
            }
            Tool::SpawnRandom => {
                gizmos.circle_2d(
                    Isometry2d::new(cursor_coords.0, Rot2::IDENTITY),
                    settings.outer_radius,
                    Color::WHITE,
                );
                gizmos.circle_2d(
                    Isometry2d::new(cursor_coords.0, Rot2::IDENTITY),
                    settings.inner_radius,
                    Color::WHITE,
                );
            }
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        };
    };
}
