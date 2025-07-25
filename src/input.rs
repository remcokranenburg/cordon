use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

// Event for exiting the game
#[derive(Debug, Clone, Event)]
pub struct ExitGameEvent;

// System to send ExitGameEvent on keyboard shortcuts
pub fn exit_game_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit_event_writer: EventWriter<ExitGameEvent>,
) {
    // Ctrl+Q or Ctrl+W
    let ctrl = keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    if ctrl
        && (keyboard_input.just_pressed(KeyCode::KeyQ)
            || keyboard_input.just_pressed(KeyCode::KeyW))
    {
        exit_event_writer.write(ExitGameEvent);
    }
}

// System to handle ExitGameEvent and exit the app
pub fn exit_game_event_system(
    mut exit_event_reader: EventReader<ExitGameEvent>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if exit_event_reader.read().next().is_some() {
        app_exit_events.write(AppExit::Success);
    }
}

// Setup function for input actions, matching the style in states/menu.rs
pub fn plugin(app: &mut App) {
    app.add_event::<ExitGameEvent>()
        .add_systems(Update, (exit_game_input_system, exit_game_event_system));
}
