use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::egui;

pub fn ui(ui: &mut egui::Ui, diagnostics: &DiagnosticsStore) {
    ui.label(format!("fps: {}", get_fps(diagnostics)));
    ui.label(format!("mspf: {} ms", get_frame_time(diagnostics)));
    ui.label(format!("frame: {}", get_frame_count(diagnostics)));
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
