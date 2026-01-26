mod ui;
mod window;

use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(window::window_plugin)
        .add_plugins(ui::plugin)
        .run();
}
