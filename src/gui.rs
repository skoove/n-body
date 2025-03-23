use std::collections::VecDeque;

use bevy::{input::mouse::MouseWheel, render::render_resource::encase::private::Length};
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};
use egui_plot::{Line, Plot, PlotPoints, Points};

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
            )
            .insert_resource(PreformanceData {
                fps: [0.0].into(),
                frame_time: [0.0].into(),
            });
    }
}

#[derive(Resource, Debug)]
struct PreformanceData {
    fps: VecDeque<f32>,
    frame_time: VecDeque<f32>,
}

fn preformance_gui(
    mut contexts: EguiContexts,
    time: Res<Time>,
    mut preformance_data: ResMut<PreformanceData>,
) {
    preformance_data.fps.push_back(1.0 / time.delta_secs());
    preformance_data
        .frame_time
        .push_back(1000.0 * time.delta_secs());

    println!("{:#?}", preformance_data);

    if preformance_data.fps.length() > 100 {
        preformance_data.fps.pop_front();
    }

    if preformance_data.frame_time.length() > 100 {
        preformance_data.frame_time.pop_front();
    }

    egui::Window::new("preformance").show(contexts.ctx_mut(), |ui| {
        let fps_plot: PlotPoints = preformance_data
            .fps
            .iter()
            .enumerate()
            .map(|(i, fps)| [i as f64, *fps as f64])
            .collect();

        let frame_time_plot: PlotPoints = preformance_data
            .frame_time
            .iter()
            .enumerate()
            .map(|(i, frametime)| [i as f64, *frametime as f64])
            .collect();

        ui.horizontal(|ui| {
            ui.label(format!("frametime: {:.1}ms", 1000.0 * time.delta_secs()));

            Plot::new("fps plot")
                .view_aspect(2.0)
                .show(ui, |plot_fn| plot_fn.line(Line::new(fps_plot)));
        });

        ui.horizontal(|ui| {
            ui.label(format!("fps: {:.0}", 1.0 / time.delta_secs()));

            Plot::new("frame time plot")
                .view_aspect(2.0)
                .show(ui, |plot_fn| plot_fn.line(Line::new(frame_time_plot)));
        });
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
