use crate::config::GameState;
use bevy::prelude::*;

pub fn initialize(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(startup));
    app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(update));
    app.add_system_set(SystemSet::on_exit(GameState::Menu).with_system(shutdown));
}

// Marker components for UI elements
#[derive(Component)]
struct MenuUI;
#[derive(Component)]
struct TitleText;
#[derive(Component)]
struct GymButton;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera3dBundle::default());

    // All this is just for spawning centered text.
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "PrismPals!",
                        TextStyle {
                            font: asset_server.load("fonts/Hind-Regular.otf"),
                            font_size: 96.,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(TitleText);
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::rgb(0.35, 0.75, 0.35).into(),
                    ..default()
                })
                .insert(GymButton)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            text: Text::from_section(
                                "Gym",
                                TextStyle {
                                    font: asset_server.load("fonts/Hind-Regular.otf"),
                                    font_size: 36.,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        })
                        .insert(TitleText);
                });
        })
        .insert(MenuUI);
}

fn update(
    mut game_state: ResMut<State<GameState>>,
    gym_button_query : Query<(&Interaction, &Children), (Changed<Interaction>, With<GymButton>)>,
) {
    for (interaction, _) in &mut gym_button_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::Gym).unwrap();
            },
            _ => {}
        }
    }
}

fn shutdown(query: Query<Entity, With<MenuUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}