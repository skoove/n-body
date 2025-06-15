use bevy::{diagnostic::DiagnosticsStore, input::mouse::MouseWheel, prelude::*};
use bevy_egui::{egui, EguiContextPass, EguiPlugin};

mod performance;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin {
            enable_multipass_for_primary_context: true,
        },))
            .add_systems(EguiContextPass, egui_system)
            .add_systems(
                PreUpdate,
                absorb_egui_inputs
                    .after(bevy_egui::input::write_egui_input_system)
                    .before(bevy_egui::begin_pass_system),
            );
    }
}

fn egui_system(mut contexts: bevy_egui::EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("menu_bar")
        .resizable(false)
        .show(ctx, |ui| ui.label("n-body"));

    // left side panel
    egui::SidePanel::left("left_panel")
        .min_width(250.0)
        .show(ctx, |ui| {
            egui_box(ui, "performance", true, |ui| {
                performance::ui(ui, &diagnostics);
            });
        });
}

fn egui_box(ui: &mut egui::Ui, title: &str, open: bool, contents: impl FnOnce(&mut egui::Ui)) {
    egui::CollapsingHeader::new(title)
        .default_open(open)
        .show(ui, |ui| contents(ui));
    ui.separator();
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
