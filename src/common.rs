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

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn red() -> Self {
        Self {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn green() -> Self {
        Self {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }

    pub fn yellow() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn darken(&self) -> Self {
        Self {
            r: self.r * 0.5,
            g: self.g * 0.5,
            b: self.b * 0.5,
        }
    }

    pub fn lighten(&self) -> Self {
        Self {
            r: f64::min(self.r * 1.5, 255.0),
            g: f64::min(self.g * 1.5, 255.0),
            b: f64::min(self.b * 1.5, 255.0),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "rgb({}, {}, {})",
            self.r * 255.0,
            self.g * 255.0,
            self.b * 255.0
        )
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    /// Determine the next position based on the current position and direction. Wraps around when
    /// the position is outside the grid.
    pub fn next(&self, direction: &Direction, width: usize, height: usize) -> Self {
        match direction {
            Direction::North => Position {
                x: self.x,
                y: if self.y == 0 { height - 1 } else { self.y - 1 },
            },
            Direction::South => Position {
                x: self.x,
                y: if self.y == height - 1 { 0 } else { self.y + 1 },
            },
            Direction::West => Position {
                x: if self.x == 0 { width - 1 } else { self.x - 1 },
                y: self.y,
            },
            Direction::East => Position {
                x: if self.x == width - 1 { 0 } else { self.x + 1 },
                y: self.y,
            },
        }
    }
}
