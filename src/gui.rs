use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};

use crate::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, preformance_gui);
    }
}

fn preformance_gui(mut contexts: EguiContexts, time: Res<Time>) {
    egui::Window::new("preformance")
        .show(contexts.ctx_mut(), |ui| {
            fps_widget(ui, time);
        });
}

fn fps_widget(ui: &mut Ui, time: Res<Time>) {
    ui.label(format!("frametime: {:.1}ms", 1000.0 * time.delta_secs()));
    ui.label(format!("fps: {:.0}", 1.0 / time.delta_secs()));
    ui.separator();
}
