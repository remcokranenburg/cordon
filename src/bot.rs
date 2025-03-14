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

use crate::{common::Direction, game::GameState};
use web_sys::js_sys::Math;

/// Drunk lamppost bot. This bot will randomly choose a direction to go to, but
/// will avoid collisions. It will also try to keep the current direction if
/// possible. This is actually not really how a drunk would behave around a
/// lamppost, but it's a little less crashy than a completely random bot.
pub fn drunk_lamppost_next(game_state: &GameState) -> Direction {
    let current_direction = game_state.players[game_state.active_player].segments.back().unwrap().1;
    let mut acceptable_directions = Vec::new();

    // find directions that don't result in a collision
    for direction in Direction::ALL {
        let mut cloned_state = game_state.clone();
        cloned_state.players[cloned_state.active_player].set_direction(direction);
        cloned_state._step();

        if !cloned_state.has_collision() {
            acceptable_directions.push(direction);
        }
    }

    // if we're going to crash anyway, keep the current direction
    if acceptable_directions.is_empty() {
        return current_direction;
    }

    // if current direction is acceptable, keep it most of the time
    if acceptable_directions.contains(&current_direction) && Math::random() > 0.1 {
        return current_direction;
    }

    // otherwise, pick a random direction from acceptable directions
    let random_direction = (Math::random() * (acceptable_directions.len()) as f64).floor() as usize;
    acceptable_directions[random_direction]
}
