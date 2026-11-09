use bevy::prelude::*;

mod main_menu;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scene {
    #[default]
    MainMenu,
    Settings,
    Level1a,
    Level1b,
    Level1c,
}

pub struct ArtifactGame;

impl Plugin for ArtifactGame {
    fn build(&self, app: &mut App) {
        app.init_state::<Scene>();
        app.add_systems(Startup, setup_camera);
        app.add_plugins(main_menu::MainMenuPlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
