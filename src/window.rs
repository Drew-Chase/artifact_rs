use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_mini_fps::fps_plugin;

pub fn window_plugin(app: &mut App) {
    app.add_plugins((
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
    .add_systems(Startup, configure_frame_rate);
}

fn configure_frame_rate(mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(60.0); // Limit to 60 FPS
}
