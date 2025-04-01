use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::EguiPlugin;
use performance_gui::PreformanceGuiPlugin;
use simulation_controls::SimulationControlsGuiPlugin;
use tools::ToolsGuiPlugin;

pub mod performance_gui;
mod simulation_controls;
mod tools;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin,
            PreformanceGuiPlugin,
            SimulationControlsGuiPlugin,
            ToolsGuiPlugin,
        ))
        .add_systems(
            PreUpdate,
            absorb_egui_inputs
                .after(bevy_egui::input::write_egui_input_system)
                .before(bevy_egui::begin_pass_system),
        );
    }
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
