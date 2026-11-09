#![cfg_attr(not(debug_assertions), windows_subsystem)]

pub mod game;
pub mod plugins;

use bevy::{prelude::*, window::PresentMode};
use bevy_asset_packer::{AssetBundle, BundledAssetPlugin};
use bevy_wicon::WindowIconPlugin;
use game::ArtifactGame;
use plugins::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins((
            BundledAssetPlugin::new(AssetBundle::from_bytes(include_bytes!("../target/assets.pack"), None)?),
            WindowIconPlugin::new(include_bytes!("../assets/app-icon.png")),
            DefaultPlugins.set(window_settings()),
            DebugPlugin,
            ArtifactGame,
        ))
        .run();

    Ok(())
}

fn window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Artifact - The Journey Unraveled".into(),
            name: Some("artifact.app".into()),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }
}
