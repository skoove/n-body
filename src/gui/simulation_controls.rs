use std::time::Duration;

use crate::{particle, simulation::SimSettings};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct SimulationControlsGuiPlugin;

impl Plugin for SimulationControlsGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sim_controls_gui)
            .insert_resource(MaxDelta(32));
    }
}

#[derive(Resource)]
/// controls the amount of time we will wait for fixedupate until we start slowing down time
struct MaxDelta(u64);

fn sim_controls_gui(
    mut contexts: EguiContexts,
    mut sim_settings: ResMut<SimSettings>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut max_delta: ResMut<MaxDelta>,
    commands: Commands,
    particles: Query<Entity, With<particle::Particle>>,
    particle_count: Res<particle::ParticleCount>,
) {
    egui::Window::new("simulation controls").show(contexts.ctx_mut(), |ui| {
        if sim_settings.paused {
            if ui.button("play").clicked() {
                sim_settings.toggle_pause()
            };
        } else if ui.button("pause").clicked() {
            sim_settings.toggle_pause();
        }

        ui.horizontal(|ui| {
            ui.label(format!("particles: {}", particle_count.0));
            if ui.button("despawn particles").clicked() {
                particle::despawn_particles(commands, particles);
            };
        });

        ui.add(
            egui::Slider::new(&mut max_delta.0, 10..=100)
                .text("max delta")
                .suffix(" ms"),
        )
        .on_hover_text_at_pointer(
            "how low the fps will get before we start slowing down time to perserve frame rate",
        );
        let max_delta_dur = Duration::from_millis(max_delta.0);
        virtual_time.set_max_delta(max_delta_dur);

        // gravity constant
        ui.add(
            egui::Slider::new(&mut sim_settings.gravity_constant, 1.0..=10000.0)
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
