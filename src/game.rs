// Blockade 1976, a Retro Remake
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

use std::{
    collections::VecDeque,
    fmt::{self, Debug, Formatter},
};

use leptos::logging::log;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.5,
            b: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub color: Color,
    pub score: u32,
    pub position: Position,
    pub direction: Direction,
    pub action: Direction,
    pub blockades: VecDeque<(Position, WallType)>,
}

#[derive(Copy, Clone, Debug)]
pub enum WallType {
    Horizontal,
    Vertical,
    CornerTopLeft,
    CornerTopRight,
    CornerBottomLeft,
    CornerBottomRight,
}

impl WallType {
    pub fn from_action(direction: Direction, action: Direction) -> Result<WallType, &'static str> {
        match (direction, action) {
            (Direction::North, Direction::North) => Ok(WallType::Vertical),
            (Direction::South, Direction::South) => Ok(WallType::Vertical),
            (Direction::West, Direction::West) => Ok(WallType::Horizontal),
            (Direction::East, Direction::East) => Ok(WallType::Horizontal),
            (Direction::North, Direction::West) => Ok(WallType::CornerTopRight),
            (Direction::North, Direction::East) => Ok(WallType::CornerTopLeft),
            (Direction::South, Direction::West) => Ok(WallType::CornerBottomRight),
            (Direction::South, Direction::East) => Ok(WallType::CornerBottomLeft),
            (Direction::East, Direction::North) => Ok(WallType::CornerBottomRight),
            (Direction::East, Direction::South) => Ok(WallType::CornerTopRight),
            (Direction::West, Direction::North) => Ok(WallType::CornerBottomLeft),
            (Direction::West, Direction::South) => Ok(WallType::CornerTopLeft),
            _ => return Err("Invalid wall placement"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Wall(WallType, Color),
    Player(usize),
    Empty,
}

#[derive(Clone)]
pub struct Grid {
    pub data: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid {
            data: Grid::init_data(width, height),
        };
        grid.place_walls();
        grid
    }

    pub fn reset(&mut self) {
        self.data = Grid::init_data(self.data[0].len(), self.data.len());
        self.place_walls();
    }

    fn init_data(width: usize, height: usize) -> Vec<Vec<Cell>> {
        let data = vec![vec![Cell::Empty; width]; height];
        data
    }

    fn place_walls(&mut self) {
        let width = self.data[0].len();
        let height = self.data.len();

        self.data[0][0] = Cell::Wall(WallType::CornerTopLeft, Default::default());
        self.data[0][width - 1] = Cell::Wall(WallType::CornerTopRight, Default::default());
        self.data[height - 1][0] = Cell::Wall(WallType::CornerBottomLeft, Default::default());
        self.data[height - 1][width - 1] =
            Cell::Wall(WallType::CornerBottomRight, Default::default());

        for i in 1..(width - 1) {
            self.data[0][i] = Cell::Wall(WallType::Horizontal, Default::default());
            self.data[height - 1][i] = Cell::Wall(WallType::Horizontal, Default::default());
        }

        for i in 1..height - 1 {
            self.data[i][0] = Cell::Wall(WallType::Vertical, Default::default());
            self.data[i][width - 1] = Cell::Wall(WallType::Vertical, Default::default());
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        writeln!(f)?;
        for row in self.data.iter() {
            write!(f, "  ")?;
            for cell in row.iter() {
                match cell {
                    Cell::Wall(..) => write!(f, "W")?,
                    Cell::Player(..) => write!(f, "P")?,
                    Cell::Empty => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "}}")?;
        Ok(())
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
    pub grid: Grid,
    pub player_turn: usize,
    pub players: Vec<Player>,
    pub rounds: u32,
}

impl GameState {
    pub fn next_player(&mut self) {
        self.player_turn = (self.player_turn + 1) % self.players.len();
    }

    pub fn next_round(&mut self) {
        self.player_turn = 0;
        self.grid.reset();
        self.reset_players();
        self.place_players();
        self.phase = Phase::Step;
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.player_turn]
    }

    pub fn current_player_id(&self) -> usize {
        self.player_turn
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.player_turn]
    }

    fn reset_players(&mut self) {
        for (i, player) in self.players.iter_mut().enumerate() {
            if i == 0 {
                player.position = Position { x: 10, y: 10 };
                player.direction = Direction::South;
                player.action = Direction::South;
            } else if i == 1 {
                player.position = Position { x: 20, y: 20 };
                player.direction = Direction::North;
                player.action = Direction::North;
            } else {
                // TODO: position >2 players
            }
        }
    }

    fn place_players(&mut self) {
        for (i, player) in self.players.iter_mut().enumerate() {
            let x = player.position.x;
            let y = player.position.y;
            println!("placing player {} in position {}x{}", i, x, y);
            self.grid.data[y][x] = Cell::Player(i);
        }
    }

    pub fn step(&mut self) -> Phase {
        let grid_width = self.grid.data[0].len();
        let grid_height = self.grid.data.len();
        let old_direction = self.current_player().direction;

        self.current_player_mut().direction = self.current_player().action;

        // find new position
        let player = self.current_player();
        let x = player.position.x;
        let y = player.position.y;

        let (dx, dy) = match player.direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        };

        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        assert!(nx < grid_width);
        assert!(ny < grid_height);

        // check the next cell for result of action
        match self.grid.data[ny][nx] {
            Cell::Wall(..) | Cell::Player(..) => {
                // other players score one point
                for (i, p) in self.players.iter_mut().enumerate() {
                    if i != self.player_turn {
                        p.score += 1;
                        println!("player {} score: {}", i, p.score);
                    }
                }

                for p in self.players.iter() {
                    // suggest next step
                    if p.score >= self.rounds {
                        return Phase::GameOver;
                    }
                }

                Phase::Score
            }
            Cell::Empty => {
                // place wall
                let player = self.current_player_mut();
                log!("old direction: {:?}, action: {:?}", old_direction, player.action);
                let wall_type = WallType::from_action(old_direction, player.action).unwrap();
                self.grid.data[y][x] = Cell::Wall(wall_type, player.color);

                // move forward
                let player = self.current_player_mut();
                player.position.x = nx;
                player.position.y = ny;
                self.place_players();
                self.next_player();

                // suggest next step
                Phase::Step
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        let mut state = GameState {
            phase: Phase::Step,
            grid: Grid::new(32, 28),
            player_turn: 0,
            players: vec![
                Player {
                    color: Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                    },
                    score: 0,
                    position: Position { x: 10, y: 10 },
                    direction: Direction::South,
                    action: Direction::South,
                    blockades: VecDeque::new(),
                },
                Player {
                    color: Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,
                    },
                    score: 0,
                    position: Position { x: 20, y: 20 },
                    direction: Direction::North,
                    action: Direction::North,
                    blockades: VecDeque::new(),
                },
            ],
            rounds: 6,
        };

        state.place_players();
        state
    }
}
