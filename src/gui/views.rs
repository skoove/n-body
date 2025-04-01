use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::particle::Particle;
use crate::simulation::motion::OldPosition;

pub struct ViewsPlugin;

impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (views_gui, render_velocity_arrows))
            .init_resource::<ViewSettings>();
    }
}

#[derive(Resource)]
struct ViewSettings {
    velocity_arrows: bool,
}

#[allow(clippy::derivable_impls)] // clippy i seriously do not care
impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            velocity_arrows: false,
        }
    }
}

fn views_gui(mut contexts: EguiContexts, mut settings: ResMut<ViewSettings>) {
    egui::Window::new("views").show(contexts.ctx_mut(), |ui| {
        ui.checkbox(&mut settings.velocity_arrows, "velocity arrows")
            .on_hover_text_at_pointer("renders velocity arrows on particles")
    });
}

fn render_velocity_arrows(
    mut gizmos: Gizmos,
    settings: Res<ViewSettings>,
    particles: Query<(&Transform, &OldPosition), With<Particle>>,
) {
    if !settings.velocity_arrows {
        return;
    }
    for (pos, OldPosition(old_pos)) in particles.iter() {
        let current_position = pos.translation.truncate();
        let old_position = old_pos.translation.truncate();
        let velocity = current_position - old_position;

        gizmos
            .arrow_2d(
                current_position,
                current_position + velocity * 10.0,
                Color::hsv(0.0, 0.5, 1.0),
            )
            .with_tip_length(10.0);
    }
}
