use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::camera::{self, CursorWorldCoords};
use crate::particle::ParticleBundle;

pub struct ToolsGuiPlugin;

impl Plugin for ToolsGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tools_gui,
                tool_just_released,
                tool_just_pressed,
                tool_drag,
                tool_not_pressed,
            ),
        )
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
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            mass: 10000.0,
            radius: 10.0,
            velocity: Vec2::ZERO,
            position: Vec2::ZERO,
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
            Tool::SpawnRandom => todo!(),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        }
    });
}

fn spawn_particle_gui(ui: &mut egui::Ui, mut settings: ResMut<ToolSettings>) {
    ui.horizontal(|ui| {
        ui.label("mass:");
        ui.add(egui::DragValue::new(&mut settings.mass))
            .on_hover_text_at_pointer("the mass of the spawned particle")
    });
    ui.horizontal(|ui| {
        ui.label("radius:");
        ui.add(egui::DragValue::new(&mut settings.radius))
            .on_hover_text_at_pointer("the radius of the spawned particle")
    });
}

/// things done when the left mouse button is released
fn tool_just_released(
    mut commands: Commands,
    selected_tool: Res<Tool>,
    mut settings: ResMut<ToolSettings>,
    inputs: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<camera::CursorWorldCoords>,
) {
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
            Tool::SpawnRandom => todo!(),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        };
    };
}

/// things done on the frame that the left mouse button is pressed
fn tool_just_pressed(
    selected_tool: Res<Tool>,
    mut settings: ResMut<ToolSettings>,
    inputs: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<camera::CursorWorldCoords>,
) {
    if inputs.just_pressed(MouseButton::Left) {
        match *selected_tool {
            Tool::SpawnParticle => {
                settings.position = cursor_coords.0;
            }
            Tool::SpawnRandom => todo!(),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        }
    }
}

/// things done if the left mouse button is down
fn tool_drag(
    selected_tool: Res<Tool>,
    mut settings: ResMut<ToolSettings>,
    inputs: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<camera::CursorWorldCoords>,
    mut gizmos: Gizmos,
) {
    if inputs.pressed(MouseButton::Left) {
        match *selected_tool {
            Tool::SpawnParticle => {
                let direction = settings.position - cursor_coords.0;
                let arrow_end = settings.position + direction;
                gizmos
                    .arrow_2d(settings.position, arrow_end, Color::WHITE)
                    .with_tip_length(20.0);
                gizmos.circle_2d(
                    Isometry2d::new(settings.position, Rot2::IDENTITY),
                    settings.radius,
                    Color::WHITE,
                );
            }
            Tool::SpawnRandom => todo!(),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        }
    }
}

/// things done any time there is no input other than mouse movement
fn tool_not_pressed(
    selected_tool: Res<Tool>,
    inputs: Res<ButtonInput<MouseButton>>,
    settings: Res<ToolSettings>,
    cursor_coords: Res<camera::CursorWorldCoords>,
    mut gizmos: Gizmos,
) {
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
            Tool::SpawnRandom => todo!(),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        }
    }
}
