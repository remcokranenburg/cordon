use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    common::{self, Color},
    layout,
};

fn draw_wall(
    wall_type: &layout::WallType,
    color: &common::Color,
    c: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    c.set_fill_style_str(&color.to_string());
    c.fill_rect(x, y, width, height);

    c.set_stroke_style_str(&Color::black().to_string());
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
    grid_data: &Vec<Vec<layout::Cell>>,
    canvas: &HtmlCanvasElement,
) {
    let canvas_width = canvas.width() as f64;
    let canvas_height = canvas.height() as f64;

    // Account for the fact that the canvas is not a perfect multiple of the grid size
    let draw_width = (canvas_width / 32.0).floor() * 32.0;
    let draw_height = (canvas_height / 28.0).floor() * 28.0;

    let cell_width = draw_width / grid_data[0].len() as f64;
    let cell_height = draw_height / grid_data.len() as f64;

    c.set_fill_style_str(&Color::black().to_string());
    c.fill_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);

    for (row_i, row) in grid_data.iter().enumerate() {
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
                layout::Cell::Player(direction, color) => {
                    let line_width = 4.0;
                    let margin = line_width / 2.0;
                    c.set_line_width(line_width);
                    c.set_stroke_style_str(&color.to_string());
                    c.begin_path();

                    match direction {
                        common::Direction::North => {
                            c.move_to(x + margin, y);
                            c.line_to(x_mid, y_high + margin);
                            c.line_to(x_high - margin, y);
                        }
                        common::Direction::South => {
                            c.move_to(x + margin, y_high);
                            c.line_to(x_mid, y - margin);
                            c.line_to(x_high - margin, y_high);
                        }
                        common::Direction::West => {
                            c.move_to(x_high, y - margin);
                            c.line_to(x + margin, y_mid);
                            c.line_to(x_high, y_high + margin);
                        }
                        common::Direction::East => {
                            c.move_to(x, y - margin);
                            c.line_to(x_high - margin, y_mid);
                            c.line_to(x, y_high + margin);
                        }
                    }

                    c.stroke();
                }
                layout::Cell::Collision => {
                    c.set_fill_style_str(&Color::yellow().to_string());
                    c.fill_rect(x, y_high, cell_width, cell_height);
                }
                layout::Cell::Empty => {}
                layout::Cell::Letter(letter, color) => {
                    c.set_fill_style_str(&color.to_string());
                    c.fill_rect(x, y_high, cell_width, cell_height);

                    c.set_stroke_style_str(&Color::black().to_string());
                    c.set_line_width(4.0);

                    c.set_font("10px sans-serif");
                    c.set_text_align("center");
                    c.set_text_baseline("middle");

                    c.stroke_text_with_max_width(&letter.to_string(), x_mid, y_mid, cell_width)
                        .unwrap();
                }
            }
        }
    }
}
