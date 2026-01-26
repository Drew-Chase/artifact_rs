use bevy::prelude::*;

#[derive(Component)]
struct PlayButton;
#[derive(Component)]
struct ExitButton;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_ui)
        .add_systems(Update, (play_button_system, exit_button_system));
}

fn setup_ui(mut commands: Commands) {
    // Container for the main menu
    commands
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    PlayButton,
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Play"));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    ExitButton,
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Exit"));
                });
        });
}

fn play_button_system(query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>) {
    for interaction in query.iter() {
        match interaction {
            Interaction::Pressed => println!("Pressed on Play button!"),
            Interaction::Hovered => println!("Hovered on Play button!"),
            Interaction::None => {}
        }
    }
}

fn exit_button_system(query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>) {
    for interaction in query.iter() {
        match interaction {
            Interaction::Hovered => println!("Hovered on Exit button!"),
            Interaction::Pressed => {
                std::process::exit(0);
            },
            Interaction::None => {}
        }
    }
}
