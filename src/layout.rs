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

use crate::common::{Direction, Position};
use crate::game::{GameState, Player};
use crate::window;
use crate::{CORDON_BLUE, CORDON_GREEN, CORDON_ORANGE, CORDON_PURPLE, CORDON_RED, CORDON_WHITE};
use bevy::prelude::*;
use std::{
    collections::VecDeque,
    fmt::{self, Debug, Formatter},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup.after(window::setup));
    app.add_systems(Update, update_board);
}

#[derive(Clone, Component)]
struct GridPosition {
    x: usize,
    y: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<GameState>,
) {
    // Load any assets or resources needed for rendering the board
    let rectangle_mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let cordon_explosion = materials.add(Color::srgb(1.0, 1.0, 0.0));

    place_obstacles(&mut commands, &mut meshes, &mut materials, &game_state);
    place_players(&mut commands, &mut meshes, &mut materials, &game_state);
}

fn update_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid_query: Query<(
        &GridPosition,
        &mut Transform,
    )>,
    game_state: Res<GameState>,
    camera: Single<&Camera>,
) {
    let Vec2 {
        x: width,
        y: height,
    } = camera
        .logical_viewport_size()
        .unwrap_or(Vec2::new(1152.0, 1008.0));
    let cell_width = width / 32.0;
    let cell_height = height / 28.0;

    grid_query
        .into_iter()
        .for_each(|(grid_pos, mut transform)| {
            let x = grid_pos.x as f32 - game_state.grid_width as f32 / 2.0 + 0.5;
            let y = grid_pos.y as f32 - game_state.grid_height as f32 / 2.0 + 0.5;

            *transform = {
                let scaled = Transform::from_scale(Vec3::new(cell_width, cell_height, 1.0));
                let translated = Transform::from_xyz(x, y, 0.0);
                scaled.mul_transform(translated)
            };
        });
}

#[derive(Debug)]
pub enum WallError {
    SelfCollision,
    NotAdjacent,
}

#[derive(Copy, Clone, Debug, Component)]
pub enum WallType {
    Horizontal,
    Vertical,
    CornerTopLeft,
    CornerTopRight,
    CornerBottomLeft,
    CornerBottomRight,
}

impl WallType {
    /// Calculate wall type from current and previous directions.
    pub fn calculate_from_directions(
        i: usize,
        segments: &VecDeque<(Position, Direction)>,
    ) -> Result<WallType, WallError> {
        if i == 0 {
            return Ok(WallType::Vertical);
        }

        let from = segments[i - 1].1;
        let to = segments[i].1;

        match (from, to) {
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
            (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::West, Direction::East)
            | (Direction::East, Direction::West) => Err(WallError::SelfCollision),
        }
    }

    // Calculate wall type from obstacles: the wall type is determined by the preceding and
    // following obstacles. For example, if the preceding obstacle is south of the current, and the
    // following obstacle is west of the current, the wall type is CornerTopRight.
    //
    // Note: the preceding obstacle of the first is the last, and the following obstacle of the
    // last is the first.
    pub fn calculate_from_positions(
        i: usize,
        obstacles: &Vec<Position>,
    ) -> Result<WallType, WallError> {
        let current = obstacles[i];
        let preceding = if i == 0 {
            obstacles[obstacles.len() - 1]
        } else {
            obstacles[i - 1]
        };
        let following = if i == obstacles.len() - 1 {
            obstacles[0]
        } else {
            obstacles[i + 1]
        };

        if preceding.x == current.x && following.x == current.x {
            Ok(WallType::Vertical)
        } else if preceding.y == current.y && following.y == current.y {
            Ok(WallType::Horizontal)
        } else if preceding.y > current.y && following.x < current.x {
            Ok(WallType::CornerTopRight)
        } else if preceding.x > current.x && following.y > current.y {
            Ok(WallType::CornerTopLeft)
        } else if preceding.x < current.x && following.y < current.y {
            Ok(WallType::CornerBottomRight)
        } else if preceding.y < current.y && following.x > current.x {
            Ok(WallType::CornerBottomLeft)
        } else {
            // log!("{:?} {:?} {:?}", preceding, current, following);
            Err(WallError::NotAdjacent)
        }
    }
}

pub fn player_to_color(player_index: usize) -> Color {
    match player_index {
        0 => CORDON_RED,
        1 => CORDON_BLUE,
        2 => CORDON_ORANGE,
        3 => CORDON_PURPLE,
        _ => CORDON_WHITE,
    }
}

#[derive(Copy, Clone, Debug, Component)]
pub enum Cell {
    Wall(WallType, Color),
    Player(Direction, Color),
    Collision,
    Letter(char, Color),
    Empty,
}

impl Cell {
    pub fn head_from_player(player: &Player) -> Self {
        let (_, direction) = player.segments.back().unwrap();
        Cell::Player(*direction, player_to_color(player.id))
    }
}

#[derive(Clone, Resource)]
pub struct Grid {
    data: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, game_state: &GameState) -> Self {
        let mut grid = Grid {
            data: Grid::init_data(width, height),
        };
        grid.place_objects(game_state);
        grid
    }

    pub fn reset(&mut self, game_state: &GameState) {
        self.data = Grid::init_data(self.data[0].len(), self.data.len());
        self.place_objects(game_state);
    }

    pub fn get_data(&self) -> &Vec<Vec<Cell>> {
        &self.data
    }

    fn place_objects(&mut self, game_state: &GameState) {
        self.place_obstacles(game_state);
        self.place_players(game_state);
        self.place_collision(game_state);
    }

    fn init_data(width: usize, height: usize) -> Vec<Vec<Cell>> {
        let data = vec![vec![Cell::Empty; width]; height];
        data
    }

    fn place_obstacles(&mut self, game_state: &GameState) {
        for (i, obstacle) in game_state.obstacles.iter().enumerate() {
            self.data[obstacle.y][obstacle.x] = Cell::Wall(
                WallType::calculate_from_positions(i, &game_state.obstacles)
                    .expect("should be contiguous"),
                CORDON_GREEN,
            );
        }
    }

    fn place_players(&mut self, game_state: &GameState) {
        for player in game_state.players.iter() {
            for (i, (position, _)) in player.segments.iter().enumerate() {
                if i == player.segments.len() - 1 {
                    self.data[position.y][position.x] = Cell::head_from_player(player);
                } else {
                    match WallType::calculate_from_directions(i, &player.segments) {
                        Ok(wall_type) => {
                            self.data[position.y][position.x] =
                                Cell::Wall(wall_type, player_to_color(player.id));
                        }
                        Err(_) => {
                            self.data[position.y][position.x] = Cell::Collision;
                        }
                    }
                }
            }
        }
    }

    fn place_collision(&mut self, game_state: &GameState) {
        if game_state.has_collision() {
            let (position, _) = game_state.players[game_state.active_player]
                .segments
                .back()
                .unwrap();
            self.data[position.y][position.x] = Cell::Collision;

            for player in &game_state.players {
                let (position, _) = player.segments.front().unwrap();
                let digit = format!("{}", player.score).chars().nth(0).unwrap();
                self.data[position.y][position.x] = Cell::Letter(digit, player_to_color(player.id));
            }
        }
    }
}

pub fn place_obstacles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    game_state: &GameState,
) {
    for (i, obstacle) in game_state.obstacles.iter().enumerate() {
        let x = obstacle.x as f32 - game_state.grid_width as f32 / 2.0;
        let y = obstacle.y as f32 - game_state.grid_height as f32 / 2.0;

        commands.spawn((
            WallType::calculate_from_positions(i, &game_state.obstacles)
                .expect("should be contiguous"),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CORDON_GREEN)),
            Transform::from_xyz(x, y, 0.0),
            GridPosition {
                x: obstacle.x,
                y: obstacle.y,
            },
        ));
    }
}

pub fn place_players(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    game_state: &GameState,
) {
    for player in game_state.players.iter() {
        for (i, (position, direction)) in player.segments.iter().enumerate() {
            let x = position.x as f32 - game_state.grid_width as f32 / 2.0;
            let y = position.y as f32 - game_state.grid_height as f32 / 2.0;

            let color = player_to_color(player.id);

            if i == player.segments.len() - 1 {
                // Head
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                    MeshMaterial2d(materials.add(color)),
                    *direction,
                    Transform::from_xyz(x, y, 0.0),
                    GridPosition {
                        x: position.x,
                        y: position.y,
                    },
                ));
            } else {
                // Body segment
                match WallType::calculate_from_directions(i, &player.segments) {
                    Ok(wall_type) => {
                        commands.spawn((
                            wall_type,
                            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                            MeshMaterial2d(materials.add(color)),
                            Transform::from_xyz(x, y, 0.0),
                            GridPosition {
                                x: position.x,
                                y: position.y,
                            },
                        ));
                    }
                    Err(_) => {
                        commands.spawn((
                            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                            MeshMaterial2d(materials.add(CORDON_ORANGE)),
                            Transform::from_xyz(x, y, 0.0),
                            GridPosition {
                                x: position.x,
                                y: position.y,
                            },
                        ));
                    }
                }
            }
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
                    Cell::Collision => write!(f, "X")?,
                    Cell::Letter(c, _) => write!(f, "{}", c)?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
