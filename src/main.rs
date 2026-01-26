mod ui;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_mini_fps::fps_plugin;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,

                    title: "Artifact: The Journey Unraveled (Bevy)".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            |app: &mut App| {
                app.world_mut().spawn((
                    Camera2d,
                    Camera {
                        clear_color: ClearColorConfig::Custom(Color::srgb(0f32, 0.5, 0.2)),
                        ..Default::default()
                    },
                ));
            },
            fps_plugin!(),
            FramepacePlugin,
        ))
        .add_plugins(ui::plugin)
        .add_systems(Startup, setup_framepace)
        .run();
}

fn setup_framepace(mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(60.0); // Limit to 60 FPS
}
