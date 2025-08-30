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

use crate::{
    bot,
    common::{Direction, Position},
    layout::{self, place_segment},
};
use bevy::{prelude::*, time::common_conditions::on_timer};
use std::{collections::VecDeque, fmt::Debug, time::Duration};

pub fn plugin(app: &mut App) {
    app.insert_resource(GameState::new(0, 6))
        .add_systems(Startup, setup)
        .add_systems(Update, update);
}

fn setup(mut commands: Commands, game_state: Res<GameState>) {}

fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    println!("Tick");
    match game_state.tick() {
        TickResult::SegmentAdded(player_id) => {
            println!("Player {} moved", player_id);
            // TODO: move head
            let segments = &game_state.players[player_id].segments;
            let i = segments.len() - 1;
            place_segment(
                &mut commands,
                &mut meshes,
                &mut materials,
                &game_state,
                segments,
                i,
                player_id,
            );
        }
        TickResult::Collision(position) => {
            println!(
                "Player {} collided at {:?}",
                game_state.active_player, position
            );
        }
        TickResult::NextRound => {
            println!("Next round");
        }
        TickResult::Noop => {
            println!("Nothing happens, game is paused or over");
        }
    }
}

#[derive(Clone, Debug)]
pub enum Controller {
    Wasd,
    Arrows,
    Gamepad(u32),
    Bot,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize,
    pub score: u32,
    pub segments: VecDeque<(Position, Direction)>,
    pub controller: Controller,
}

impl Player {
    pub fn new(
        id: usize,
        position: Position,
        direction: Direction,
        controller: Controller,
    ) -> Self {
        Player {
            id,
            score: 0,
            segments: VecDeque::from(vec![(position, direction)]),
            controller: controller,
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
    Paused,
}

#[derive(Clone, Debug, Resource)]
pub struct GameState {
    pub phase: Phase,
    pub grid_width: usize,
    pub grid_height: usize,
    pub active_player: usize,
    pub players: Vec<Player>,
    pub obstacles: Vec<Position>,
    pub max_score: u32,
}

pub enum TickResult {
    SegmentAdded(usize), // next tick will be another step
    Collision(Position), // next tick will be scoring or game over
    NextRound,           // next tick will be a step in a new round
    Noop,                // next tick nothing happens, game is paused or over
}

impl GameState {
    pub fn new(num_players: usize, max_score: u32) -> Self {
        let width = 32;
        let height = 28;

        let player0_controller = if num_players > 0 {
            Controller::Wasd
        } else {
            Controller::Bot
        };

        let player1_controller = if num_players > 1 {
            Controller::Arrows
        } else {
            Controller::Bot
        };

        GameState {
            phase: Phase::Step,
            active_player: 0,
            players: vec![
                Player::new(
                    0,
                    Position { x: 4, y: 4 },
                    Direction::South,
                    player0_controller,
                ),
                Player::new(
                    1,
                    Position {
                        x: width - 5,
                        y: height - 5,
                    },
                    Direction::North,
                    player1_controller,
                ),
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
    pub fn tick(&mut self) -> TickResult {
        match self.phase {
            Phase::Step => {
                if let Controller::Bot = self.players[self.active_player].controller {
                    let new_direction = bot::drunk_lamppost_next(self);
                    self.players[self.active_player].set_direction(new_direction);
                }

                // while we are stepping, a tick progresses player movement and
                // calculates the consequence
                self._step();

                if self.has_collision() {
                    self.score();
                    if self.is_game_over() {
                        self.phase = Phase::GameOver;
                    } else {
                        self.phase = Phase::Score;
                    }

                    TickResult::Collision(
                        self.players[self.active_player]
                            .segments
                            .back()
                            .expect("Player has no segments")
                            .0,
                    )
                } else {
                    let player_id = self.active_player;
                    self.set_next_player();
                    self.phase = Phase::Step;
                    TickResult::SegmentAdded(player_id)
                }
            }
            Phase::Score => {
                // while scoring, the next tick resets the players, allowing for
                // an animation in between
                self.reset_players();
                self.phase = Phase::Step;
                TickResult::NextRound
            }
            Phase::GameOver | Phase::Paused => {
                // while the game is not running, ticks do nothing
                TickResult::Noop
            }
        }
    }

    /// Advance the game one step, by moving the active player in its direction.
    pub fn _step(&mut self) {
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
                player.segments = VecDeque::from(vec![(Position { x: 4, y: 4 }, Direction::South)]);
            } else if i == 1 {
                player.segments = VecDeque::from(vec![(
                    Position {
                        x: self.grid_width - 5,
                        y: self.grid_height - 5,
                    },
                    Direction::North,
                )]);
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
