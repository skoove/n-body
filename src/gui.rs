use bevy::input::mouse::MouseWheel;
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};
use egui_plot::{Plot, PlotPoints, Points};

use crate::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, preformance_gui)
            .add_systems(
                PreUpdate,
                absorb_egui_inputs
                    .after(bevy_egui::input::write_egui_input_system)
                    .before(bevy_egui::begin_pass_system),
            );
    }
}

fn preformance_gui(mut contexts: EguiContexts, time: Res<Time>) {
    egui::Window::new("preformance").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("frametime: {:.1}ms", 1000.0 * time.delta_secs()));
        ui.label(format!("fps: {:.0}", 1.0 / time.delta_secs()));
        ui.separator();

        let sin: PlotPoints = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();

        Plot::new("sin")
            .view_aspect(2.0)
            .show(ui, |plot_fn| plot_fn.points(Points::new(sin)))
    });
}

// source:
// https://github.com/vladbat00/bevy_egui/issues/47#issuecomment-2368811068
fn absorb_egui_inputs(
    mut contexts: bevy_egui::EguiContexts,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut mouse_wheel: ResMut<Events<MouseWheel>>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();
    if !(ctx.wants_pointer_input() || ctx.is_pointer_over_area()) {
        return;
    }
    let modifiers = [
        KeyCode::SuperLeft,
        KeyCode::SuperRight,
        KeyCode::ControlLeft,
        KeyCode::ControlRight,
        KeyCode::AltLeft,
        KeyCode::AltRight,
        KeyCode::ShiftLeft,
        KeyCode::ShiftRight,
    ];

    let pressed = modifiers.map(|key| keyboard.pressed(key).then_some(key));

    mouse.reset_all();
    mouse_wheel.clear();
    keyboard.reset_all();

    for key in pressed.into_iter().flatten() {
        keyboard.press(key);
    }
}
