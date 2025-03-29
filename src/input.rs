use crate::simulation::SimSettings;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sim_settings_actions);
    }
}

fn sim_settings_actions(mut settings: ResMut<SimSettings>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        settings.toggle_pause();
    }
}
