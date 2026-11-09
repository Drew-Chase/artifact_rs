use bevy::app::{App, Plugin};
use bevy::color::Color;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin { config: fps_overlay_config() });
        app.add_systems(Update, toggle_fps_overlay);
    }
}

fn toggle_fps_overlay(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<FpsOverlayConfig>) {
    if keys.just_pressed(KeyCode::F2) {
        settings.enabled = !settings.enabled;
        // Only enable the frame graph if you press the left shift when enabling the fps counter
        if keys.pressed(KeyCode::ShiftLeft) && settings.enabled {
            settings.frame_time_graph_config.enabled = true;
        } else if settings.frame_time_graph_config.enabled {
            settings.frame_time_graph_config.enabled = false;
        }
    }
}
fn fps_overlay_config() -> FpsOverlayConfig {
    FpsOverlayConfig {
        enabled: false,
        text_config: TextFont { font_size: FontSize::Px(18.0), font: default(), font_smoothing: FontSmoothing::default(), ..default() },
        text_color: Color::srgb(0.0, 1.0, 0.0),
        refresh_interval: core::time::Duration::from_millis(100),
        frame_time_graph_config: FrameTimeGraphConfig { enabled: false, min_fps: 30.0, target_fps: 60.0 },
    }
}
