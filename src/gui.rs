use std::collections::VecDeque;

use bevy::{input::mouse::MouseWheel, render::render_resource::encase::private::Length};
use bevy_egui::{
    egui::{self, Slider},
    EguiContexts, EguiPlugin,
};
use egui_plot::{Line, Plot, PlotPoints};

use crate::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, performance_gui)
            .add_systems(
                PreUpdate,
                absorb_egui_inputs
                    .after(bevy_egui::input::write_egui_input_system)
                    .before(bevy_egui::begin_pass_system),
            )
            .insert_resource(PerformanceData {
                frame_time: [0.0].into(),
            })
            .insert_resource(PerformanceGuiSettings {
                history_to_show: 100,
            });
    }
}

#[derive(Resource, Debug)]
struct PerformanceData {
    frame_time: VecDeque<f32>,
}

#[derive(Resource)]
struct PerformanceGuiSettings {
    history_to_show: i32,
}

fn performance_gui(
    mut contexts: EguiContexts,
    time: Res<Time>,
    mut performance_data: ResMut<PerformanceData>,
    mut gui_settings: ResMut<PerformanceGuiSettings>,
) {
    performance_data
        .frame_time
        .push_back(1000.0 * time.delta_secs());

    while performance_data.frame_time.length() > gui_settings.history_to_show as usize {
        performance_data.frame_time.pop_front();
    }

    let average_frame_time: f32 =
        performance_data.frame_time.iter().sum::<f32>() / performance_data.frame_time.len() as f32;

    egui::Window::new("performance").show(contexts.ctx_mut(), |ui| {
        let frame_time_plot: PlotPoints = performance_data
            .frame_time
            .iter()
            .enumerate()
            .map(|(i, frametime)| [i as f64, *frametime as f64])
            .collect();

        ui.label(format!("fps: {:.0}", 1.0 / time.delta_secs()));
        ui.label(format!("frametime: {:.1}ms", 1000.0 * time.delta_secs()));
        ui.label(format!("average frametime: {:.1}ms", average_frame_time));
        ui.label("frame time plot");

        Plot::new("frame time plot")
            .height(75.0)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_drag(false)
            .show_x(false)
            .set_margin_fraction(egui::Vec2::new(0.0, 0.0))
            .include_x(0.0)
            .include_y(20.0)
            .x_axis_label("frame")
            .show(ui, |plot_fn| plot_fn.line(Line::new(frame_time_plot)));

        ui.add(Slider::new(&mut gui_settings.history_to_show, 0..=500).text("history to show"))
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
