use crate::config::{AppState, GameState, MenuState, BUTTON_COLOR, BUTTON_HOVER_COLOR, BUTTON_PRESSED_COLOR};
use bevy::prelude::*;

pub fn build(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(AppState::Menu(MenuState::Main)).with_system(startup));
    app.add_system_set(SystemSet::on_exit(AppState::Menu(MenuState::Main)).with_system(shutdown));
    app.add_system_set(SystemSet::on_update(AppState::Menu(MenuState::Main)).with_system(update));
}

// Marker components for UI elements
#[derive(Component)]
struct MenuUI;
#[derive(Component)]
struct TitleText;
#[derive(Component)]
struct GymButton;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::ColumnReverse,
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
                        margin: UiRect::all(Val::Auto),
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
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: BUTTON_COLOR.into(),
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

fn shutdown(query: Query<Entity, With<MenuUI>>, mut commands: Commands) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn update(
    mut game_state: ResMut<State<AppState>>,
    mut gym_button_query: Query<(&Interaction, &mut UiColor, &Children), Changed<Interaction>>,
) {
    for (interaction, mut color, _) in &mut gym_button_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(AppState::Game(GameState::Gym)).unwrap();
                *color = BUTTON_PRESSED_COLOR.into();
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR.into();
            }
        }
    }
}
