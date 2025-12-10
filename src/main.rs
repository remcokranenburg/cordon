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

mod bot;
mod common;
mod game;
mod input;
mod layout;
mod render;
mod states;
mod window;

use bevy::prelude::*;

const CORDON_GREEN: Color = Color::srgb(0.0, 0.8, 0.0);
const CORDON_GREEN_HIGHLIGHT: Color = Color::srgb(0.0, 1.0, 0.0);
const CORDON_RED: Color = Color::srgb(0.8, 0.0, 0.0);
const CORDON_BLUE: Color = Color::srgb(0.0, 0.0, 0.8);
const CORDON_ORANGE: Color = Color::srgb(1.0, 0.5, 0.0);
const CORDON_PURPLE: Color = Color::srgb(0.5, 0.0, 0.5);
const CORDON_WHITE: Color = Color::srgb(0.8, 0.8, 0.8);
const CORDON_BLACK: Color = Color::srgb(0.0, 0.0, 0.0);

fn main() {
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins((
            window::plugin,
            game::plugin,
            layout::plugin,
            states::plugin,
            input::plugin,
        ))
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
