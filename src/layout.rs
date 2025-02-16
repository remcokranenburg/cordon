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

use crate::common::{Color, Direction};
use std::fmt::{self, Debug, Formatter};

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
    Explosion(bool),
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

    fn init_data(width: usize, height: usize) -> Vec<Vec<Cell>> {
        let data = vec![vec![Cell::Empty; width]; height];
        data
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
                    Cell::Explosion(x) => write!(f, "{}", if *x { "X" } else { " " })?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
