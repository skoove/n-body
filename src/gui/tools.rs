use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::particle::ParticleBundle;

pub struct ToolsGuiPlugin;

impl Plugin for ToolsGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tools_gui).init_resource::<Tool>();
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

fn tools_gui(mut contexts: EguiContexts, mut commands: Commands, mut tool: ResMut<Tool>) {
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
            Tool::SpawnParticle => spawn_particle_gui(commands, ui),
            Tool::SpawnRandom => todo!(),
            Tool::SpawnHose => todo!(),
            Tool::Attract => todo!(),
            Tool::Repel => todo!(),
        }
    });
}

fn spawn_particle_gui(mut commands: Commands, ui: &mut egui::Ui) {}
