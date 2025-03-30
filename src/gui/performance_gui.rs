use std::{any::Any, collections::VecDeque};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Slider},
    EguiContexts,
};
use egui_plot::{Line, Plot, PlotPoints};

pub struct PreformanceGuiPlugin;

impl Plugin for PreformanceGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, performance_gui)
            .insert_resource(PerformanceData {
                frame_time: [0.0].into(),
                simulation_time: [0.0].into(),
            })
            .insert_resource(PerformanceGuiSettings {
                history_to_show: 100,
            });
    }
}

#[derive(Resource, Debug)]
pub struct PerformanceData {
    pub frame_time: VecDeque<f32>,
    pub simulation_time: VecDeque<f32>,
}

#[derive(Resource)]
struct PerformanceGuiSettings {
    history_to_show: i32,
}

fn performance_gui(
    mut contexts: EguiContexts,
    mut performance_data: ResMut<PerformanceData>,
    mut gui_settings: ResMut<PerformanceGuiSettings>,
    time: Res<Time>,
) {
    performance_data
        .frame_time
        .push_back(time.delta_secs() * 1000.0);

    while performance_data.frame_time.len() > gui_settings.history_to_show as usize {
        performance_data.frame_time.pop_front();
    }

    while performance_data.simulation_time.len() > gui_settings.history_to_show as usize {
        performance_data.simulation_time.pop_front();
    }

    let average_frame_time: f32 =
        performance_data.frame_time.iter().sum::<f32>() / performance_data.frame_time.len() as f32;

    let average_sim_time: f32 = performance_data.simulation_time.iter().sum::<f32>()
        / performance_data.simulation_time.len() as f32;

    egui::Window::new("performance").show(contexts.ctx_mut(), |ui| {
        let frame_time_plot: PlotPoints = performance_data
            .frame_time
            .iter()
            .enumerate()
            .map(|(i, frametime)| [i as f64, *frametime as f64])
            .collect();

        ui.label(format!(
            "frametime: {:.1}ms",
            performance_data
                .frame_time
                .back()
                .expect("there was no preformance data!")
        ));
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
            .x_axis_label("frame")
            .show(ui, |plot_fn| plot_fn.line(Line::new(frame_time_plot)));

        let simulation_time_plot: PlotPoints = performance_data
            .simulation_time
            .iter()
            .enumerate()
            .map(|(i, simtime)| [i as f64, *simtime as f64])
            .collect();

        ui.label(format!("average simtime: {:.1}ms", average_sim_time));
        ui.label("sim time plot");

        Plot::new("sim time plot")
            .height(75.0)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_drag(false)
            .show_x(false)
            .set_margin_fraction(egui::Vec2::new(0.0, 0.0))
            .include_x(0.0)
            .include_y(0.0)
            .x_axis_label("frame")
            .show(ui, |plot_fn| plot_fn.line(Line::new(simulation_time_plot)));

        ui.add(Slider::new(&mut gui_settings.history_to_show, 1..=500).text("history to show"))
    });
}
