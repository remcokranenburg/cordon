// Cordon
//
// Copyright 2025 Remco Kranenburg <remco@burgsoft.nl>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{states::AppState, CORDON_GREEN, CORDON_GREEN_HIGHLIGHT, despawn_screen};
use bevy::{app::AppExit, ecs::spawn::SpawnIter, prelude::*};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    NewGame,
    Settings,
    #[default]
    Disabled,
}

// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub fn plugin(app: &mut App) {
    app
        // At start, the menu is not enabled. This will be changed in `menu_setup` when
        // entering the `GameState::Menu` state.
        // Current screen in the menu is handled by an independent state from `GameState`
        .init_state::<MenuState>()
        .add_systems(OnEnter(AppState::Menu), menu_setup)
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        // Systems to handle the settings menu screen
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(AppState::Menu)),
        );
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    BackToMainMenu,
    About,
    Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SelectedOption>,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut color_query: Query<&mut TextColor>,
) {
    for (interaction, mut background_color, selected, children) in &mut interaction_query {
        let mut color = color_query.get_mut(children[0]).unwrap();
        match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                *color = Color::BLACK.into();
                *background_color = CORDON_GREEN_HIGHLIGHT.into()
            }
            (Interaction::Hovered, Some(_)) => {
                *color = Color::BLACK.into();
                *background_color = CORDON_GREEN_HIGHLIGHT.into()
            }
            (Interaction::Hovered, None) => {
                *color = Color::BLACK.into();
                *background_color = CORDON_GREEN.into()
            }
            (Interaction::None, None) => {
                *color = CORDON_GREEN.into();
                *background_color = Color::NONE.into()
            }
        }
    }
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            *previous_button_color = Color::NONE.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Percent(100.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 30.0,
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        children![(
            Node {
                width: Val::Px(400.0),
                height: Val::Px(600.0),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor(CORDON_GREEN),
            children![
                // Display the game name
                (
                    Text::new("Cordon"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(CORDON_GREEN),
                    Node {
                        margin: UiRect::vertical(Val::Px(40.0)),
                        ..default()
                    },
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(Color::NONE),
                    MenuButtonAction::Play,
                    children![(Text::new("New Game"), button_text_font.clone(),),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(Color::NONE),
                    MenuButtonAction::Settings,
                    children![(Text::new("Settings"), button_text_font.clone(),),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(Color::NONE),
                    MenuButtonAction::About,
                    children![(Text::new("About"), button_text_font.clone(),),]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(Color::NONE),
                    MenuButtonAction::Quit,
                    children![(Text::new("Quit"), button_text_font,),]
                ),
            ]
        )],
    ));
}

fn settings_menu_setup(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(CORDON_GREEN),
    );

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            Children::spawn(SpawnIter(
                [(MenuButtonAction::BackToMainMenu, "Back"),]
                    .into_iter()
                    .map(move |(action, text)| {
                        (
                            Button,
                            button_node.clone(),
                            BackgroundColor(Color::NONE),
                            action,
                            children![(Text::new(text), button_text_style.clone())],
                        )
                    })
            ))
        )],
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    #[cfg(not(target_arch = "wasm32"))]
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    app_state.set(AppState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::About => {
                    app_state.set(AppState::About);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}
