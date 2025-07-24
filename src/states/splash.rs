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

use std::f64::consts::PI;

use crate::{CORDON_GREEN, despawn_screen, states::AppState};
use bevy::prelude::*;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub fn plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `AppState::Splash`
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(AppState::Splash), setup)
        // While in this state, run the `countdown` system
        .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(OnExit(AppState::Splash), despawn_screen::<OnSplashScreen>);
}

#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setup(mut commands: Commands) {
    // Display the logo
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        OnSplashScreen,
        children![
            (
                Text::new("Remco Kranenburg"),
                TextColor(CORDON_GREEN),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ),
            (Text::new("Presents"), TextColor(CORDON_GREEN),),
        ],
    ));
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(4.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    query: Query<&mut TextColor>,
) {
    if timer.tick(time.delta()).finished() {
        app_state.set(AppState::Menu);
    }

    for mut color in query {
        let elapsed_secs = timer.elapsed_secs_f64();
        *color = CORDON_GREEN
            .mix(
                &Color::BLACK,
                1.0 - f32::sin((elapsed_secs * PI / 4.0) as f32),
            )
            .into();
    }
}
