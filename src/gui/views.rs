use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::particle::Particle;
use crate::simulation::motion::{OldPosition, PreviousAcceleration};
use crate::simulation::quadtree::QuadTree;

pub struct ViewsPlugin;

impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                views_gui,
                render_velocity_arrows,
                render_acceleration_arrows,
                render_quad_tree,
            ),
        )
        .init_resource::<ViewSettings>();
    }
}

#[derive(Resource, Default)]
struct ViewSettings {
    quad_tree: bool,
    velocity_arrows: bool,
    acceleration_arrows: bool,
}

fn views_gui(mut contexts: EguiContexts, mut settings: ResMut<ViewSettings>) {
    egui::Window::new("views").show(contexts.ctx_mut(), |ui| {
        ui.checkbox(&mut settings.velocity_arrows, "velocity arrows (red)")
            .on_hover_text_at_pointer("renders velocity arrows on particles");
        ui.checkbox(
            &mut settings.acceleration_arrows,
            "acceleration arrows (green)",
        )
        .on_hover_text_at_pointer("renders acceleration arrows on particles");
        ui.checkbox(&mut settings.quad_tree, "quadtree")
    });
}

fn render_quad_tree(mut gizmos: Gizmos, settings: Res<ViewSettings>, qt: Res<QuadTree>) {
    if !settings.quad_tree {
        return;
    }
    qt.render(&mut gizmos);
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

        gizmos.arrow_2d(
            current_position,
            current_position + velocity * 20.0,
            Color::hsv(0.0, 0.5, 1.0),
        );
    }
}

fn render_acceleration_arrows(
    mut gizmos: Gizmos,
    settings: Res<ViewSettings>,
    particles: Query<(&Transform, &PreviousAcceleration), With<Particle>>,
) {
    if !settings.acceleration_arrows {
        return;
    }
    for (pos, PreviousAcceleration(acceleration)) in particles.iter() {
        let position = pos.translation.truncate();
        let arrow_length = acceleration.length().powf(0.3) * 5.0;
        gizmos.arrow_2d(
            position,
            position + acceleration.normalize_or_zero() * arrow_length,
            Color::hsv(60.0, 0.5, 1.0),
        );
    }
}
