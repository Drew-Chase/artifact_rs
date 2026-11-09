use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;

pub struct MainMenuPlugin;

const SCENE: super::Scene = super::Scene::MainMenu;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SCENE), spawn_main_menu);
        app.add_systems(FixedUpdate, scroll_layers.run_if(in_state(SCENE)));
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_load_setting = |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest();
    let artifact_logo: Handle<Image> =
        asset_server.load_builder().with_settings(image_load_setting).load(r#"texture/menus/title_screen/artifact_logo.png"#);
    let sunny_mountains_far_back: Handle<Image> =
        asset_server.load_builder().with_settings(image_load_setting).load(r#"texture/menus/title_screen/sunny-mountains-far-back.png"#);
    let sunny_mountains_hills: Handle<Image> =
        asset_server.load_builder().with_settings(image_load_setting).load(r#"texture/menus/title_screen/sunny-mountains-hills.png"#);
    let sunny_mountains_sky: Handle<Image> =
        asset_server.load_builder().with_settings(image_load_setting).load(r#"texture/menus/title_screen/sunny-mountains-sky.png"#);
    let sunny_mountains_trees: Handle<Image> =
        asset_server.load_builder().with_settings(image_load_setting).load(r#"texture/menus/title_screen/sunny-mountains-trees.png"#);

    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),

                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(SCENE),
        ))
        .with_children(|root| {
            spawn_scroller(root, sunny_mountains_sky, 256., 224., 0.15);
        });
}

#[derive(Component)]
struct Scroller {
    speed: f32,
    width: f32,
    offset: f32,
}
fn spawn_scroller(commands: &mut RelatedSpawnerCommands<ChildOf>, image: Handle<Image>, width: f32, height: f32, speed: f32) {
    for i in 0..2 {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(i as f32 * width),
                top: px(0),
                width: auto(),
                aspect_ratio: Some(width / height),
                height: percent(100),
                ..default()
            },
            ImageNode::new(image.clone()),
            Scroller { speed, width, offset: 0.0 },
        ));
    }
}

fn scroll_layers(mut query: Query<(&mut Node, &mut Scroller)>) {
    for (mut node, mut scroller) in &mut query {
        scroller.offset += scroller.speed;
        if scroller.offset >= scroller.width {
            scroller.offset -= scroller.width;
        }
        node.left = px(-scroller.offset);
    }
}
