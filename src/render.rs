use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{common, game, layout};

fn draw_wall(
    wall_type: &layout::WallType,
    color: &common::Color,
    c: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let r = color.r * 255.0;
    let g = color.g * 255.0;
    let b = color.b * 255.0;

    c.set_fill_style_str(&format!("rgb({r}, {g}, {b})"));
    c.fill_rect(x, y, width, height);

    c.set_stroke_style_str("rgb(0, 0, 0)");
    c.set_line_width(4.0);

    let half_width = width * 0.5;
    let half_height = height * 0.5;

    match wall_type {
        layout::WallType::Horizontal => {
            c.begin_path();
            c.move_to(x, y + half_height);
            c.line_to(x + width, y + half_height);
            c.stroke();
        }
        layout::WallType::Vertical => {
            c.begin_path();
            c.move_to(x + half_width, y);
            c.line_to(x + half_width, y + height);
            c.stroke();
        }
        layout::WallType::CornerTopLeft => {
            c.begin_path();
            c.move_to(x + half_width, y + height);
            c.line_to(x + half_width, y + half_height);
            c.line_to(x + width, y + half_height);
            c.stroke();
        }
        layout::WallType::CornerTopRight => {
            c.begin_path();
            c.move_to(x, y + half_height);
            c.line_to(x + half_width, y + half_height);
            c.line_to(x + half_width, y + height);
            c.stroke();
        }
        layout::WallType::CornerBottomLeft => {
            c.begin_path();
            c.move_to(x + half_width, y);
            c.line_to(x + half_width, y + half_height);
            c.line_to(x + width, y + half_height);
            c.stroke();
        }
        layout::WallType::CornerBottomRight => {
            c.begin_path();
            c.move_to(x, y + half_height);
            c.line_to(x + half_width, y + half_height);
            c.line_to(x + half_width, y);
            c.stroke();
        }
    }
}

pub fn draw_board(
    c: &CanvasRenderingContext2d,
    game_state: &game::GameState,
    canvas: &HtmlCanvasElement,
) {
    let cell_width = canvas.width() as f64 / game_state.grid.data[0].len() as f64;
    let cell_height = canvas.height() as f64 / game_state.grid.data.len() as f64;

    c.set_fill_style_str("rgb(0, 0, 0)");
    c.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    for (row_i, row) in game_state.grid.data.iter().enumerate() {
        for (cell_i, cell) in row.iter().enumerate() {
            let x = cell_i as f64 * cell_width;
            let x_mid = x + cell_width * 0.5;
            let x_high = x + cell_width;
            let y = row_i as f64 * cell_height + cell_height;
            let y_mid = y - cell_height * 0.5;
            let y_high = y - cell_height;

            match cell {
                layout::Cell::Wall(wall_type, color) => {
                    draw_wall(wall_type, color, c, x, y_high, cell_width, cell_height)
                }
                layout::Cell::Player(player_id) => {
                    let player = &game_state.players[*player_id];
                    let color = player.color;
                    let r = color.r * 255.0;
                    let g = color.g * 255.0;
                    let b = color.b * 255.0;
                    let direction = player.direction;

                    c.set_line_width(4.0);
                    c.set_stroke_style_str(&format!("rgb({r}, {g}, {b})"));
                    c.begin_path();

                    match direction {
                        common::Direction::North => {
                            c.move_to(x + 2.0, y);
                            c.line_to(x_mid, y_high + 2.0);
                            c.line_to(x_high - 2.0, y);
                        }
                        common::Direction::South => {
                            c.move_to(x + 2.0, y_high);
                            c.line_to(x_mid, y - 2.0);
                            c.line_to(x_high - 2.0, y_high);
                        }
                        common::Direction::West => {
                            c.move_to(x_high, y + 2.0);
                            c.line_to(x, y_mid);
                            c.line_to(x_high, y_high - 2.0);
                        }
                        common::Direction::East => {
                            c.move_to(x, y + 2.0);
                            c.line_to(x_high, y_mid);
                            c.line_to(x, y_high - 2.0);
                        }
                    }

                    c.stroke();
                }
                layout::Cell::Explosion(explosion) => {
                    if *explosion {
                        c.set_fill_style_str("rgb(255, 0, 0)");
                        c.fill_rect(x, y_high, cell_width, cell_height);
                    }
                }
                layout::Cell::Empty => {}
            }
        }
    }
}
