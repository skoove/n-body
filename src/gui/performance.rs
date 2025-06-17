use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::egui;

use crate::particle::ParticleCount;

pub fn ui(ui: &mut egui::Ui, diagnostics: &DiagnosticsStore, particle_count: &ParticleCount) {
    egui::Grid::new("perf_stats_grid")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("fps");
            ui.label(get_fps(diagnostics));
            ui.end_row();

            ui.label("mspf")
                .on_hover_text_at_pointer("miliseconds per frame");
            ui.label(get_frame_time(diagnostics));
            ui.end_row();

            ui.label("frame");
            ui.label(get_frame_count(diagnostics));
            ui.end_row();

            ui.label("particles");
            ui.label(format!("{}", particle_count.0));
        });
}

fn get_fps(diagnostics: &DiagnosticsStore) -> String {
    if let Some(diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = diagnostic.average() {
            return format!("{:.1}", fps);
        } else {
            return "could not get fps average".to_string();
        }
    } else {
        return "could not get fps diagnostic".to_string();
    };
}

fn get_frame_time(diagnostics: &DiagnosticsStore) -> String {
    if let Some(diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time) = diagnostic.average() {
            return format!("{:.1}", frame_time);
        } else {
            return "could not get frame time average".to_string();
        }
    } else {
        return "could not get frame time diagnostic".to_string();
    };
}

fn get_frame_count(diagnostics: &DiagnosticsStore) -> String {
    if let Some(diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
        if let Some(frame_count) = diagnostic.value() {
            return format!("{}", frame_count);
        } else {
            return "could not get frame count".to_string();
        }
    } else {
        return "could not get frame count diagnostic".to_string();
    };
}
