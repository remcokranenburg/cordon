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

use crate::common::{Color, Direction, Position};
use std::{collections::VecDeque, fmt::Debug};

#[derive(Clone, Debug)]
pub struct Player {
    pub color: Color,
    pub score: u32,
    pub segments: VecDeque<(Position, Direction)>,
}

impl Player {
    pub fn new(color: Color, position: Position, direction: Direction) -> Self {
        Player {
            color: color,
            score: 0,
            segments: VecDeque::from(vec![(position, direction)]),
        }
    }

    /// Set direction of the head segment of the specified player. This function
    /// is called by the input handling logic to set the direction of the
    /// player.
    pub fn set_direction(&mut self, direction: Direction) {
        let last = self.segments.back_mut();

        if let Some(s) = last {
            s.1 = direction;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Phase {
    Step,
    Score,
    GameOver,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub phase: Phase,
    pub grid_width: usize,
    pub grid_height: usize,
    pub active_player: usize,
    pub players: Vec<Player>,
    pub obstacles: Vec<Position>,
    pub max_score: u32,
}

impl GameState {
    pub fn new(_num_players: usize, max_score: u32) -> Self {
        let width = 32;
        let height = 28;

        GameState {
            phase: Phase::Step,
            active_player: 0,
            players: vec![
                Player::new(Color::red(), Position { x: 10, y: 10 }, Direction::South),
                Player::new(Color::blue(), Position { x: 20, y: 20 }, Direction::North),
            ],
            max_score: max_score,
            grid_width: width,
            grid_height: height,
            obstacles: generate_wall(width, height),
        }
    }

    // Advance the game one step, by moving the active player in its direction.
    // If the player hits a wall, the player is eliminated and the other players
    // score a point. If a player scores the required number of points, the game
    // is over. This function returns an event in the game, which is used
    // by the layout logic to update the state of the world.
    pub fn tick(&mut self) {
        match self.phase {
            Phase::Step => {
                // while we are stepping, a tick progresses player movement and
                // calculates the consequence
                self.step();

                if self.has_collision() {
                    self.score();
                    if self.is_game_over() {
                        self.phase = Phase::GameOver;
                    } else {
                        self.phase = Phase::Score;
                    }
                } else {
                    self.set_next_player();
                    self.phase = Phase::Step;
                }
            }
            Phase::Score => {
                // while scoring, the next tick resets the players, allowing for
                // an animation in between
                self.reset_players();
                self.phase = Phase::Step;
            }
            Phase::GameOver => {
                // while the game is over, ticks do nothing
                return;
            }
        }
    }

    /// Advance the game one step, by moving the active player in its direction.
    fn step(&mut self) {
        let (new_position, direction) = {
            let (position, direction) = self.players[self.active_player]
                .segments
                .back()
                .expect(&format!("Player {} has no segments", self.active_player));

            (
                position.next(direction, self.grid_width, self.grid_height),
                *direction,
            )
        };

        self.players[self.active_player]
            .segments
            .push_back((new_position, direction));
    }

    /// Check whether the active player has collided with a wall or another player.
    pub fn has_collision(&self) -> bool {
        let current_player = &self.players[self.active_player];
        let (position, _) = current_player
            .segments
            .back()
            .expect(&format!("Player {} has no segments", self.active_player));

        for obstacle in &self.obstacles {
            if obstacle == position {
                return true;
            }
        }

        for (i, player) in self.players.iter().enumerate() {
            for (j, (p, _)) in player.segments.iter().enumerate() {
                if p == position {
                    if self.active_player == i && j == player.segments.len() - 1 {
                        // own head: not a collision
                        continue;
                    }

                    return true;
                }
            }
        }

        false
    }

    fn score(&mut self) {
        for (i, player) in self.players.iter_mut().enumerate() {
            if i != self.active_player {
                player.score += 1;
            }
        }
    }

    fn reset_players(&mut self) {
        for (i, player) in self.players.iter_mut().enumerate() {
            if i == 0 {
                player.segments =
                    VecDeque::from(vec![(Position { x: 10, y: 10 }, Direction::South)]);
            } else if i == 1 {
                player.segments =
                    VecDeque::from(vec![(Position { x: 20, y: 20 }, Direction::North)]);
            } else {
                // TODO: position >2 players
            }
        }

        self.active_player = 0;
    }

    fn is_game_over(&self) -> bool {
        for player in &self.players {
            if player.score >= self.max_score {
                return true;
            }
        }

        return false;
    }

    fn set_next_player(&mut self) {
        self.active_player = (self.active_player + 1) % self.players.len();
    }
}

/// Generate a wall with the specified width and height. The wall starts at the
/// top middle and goes anti-clockwise around the grid.
fn generate_wall(width: usize, height: usize) -> Vec<Position> {
    let mut walls = vec![];

    for i in 1..(width - 1) {
        walls.push(Position {
            x: width - 1 - i,
            y: 0,
        });
    }

    for i in 0..height {
        walls.push(Position { x: 0, y: i });
    }

    for i in 1..(width - 1) {
        walls.push(Position {
            x: i,
            y: height - 1,
        });
    }

    for i in 0..height {
        walls.push(Position {
            x: width - 1,
            y: height - 1 - i,
        });
    }

    walls
}
