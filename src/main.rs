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

mod game;

use leptos::{ev::fullscreenchange, html::Canvas, prelude::*};
use leptos_use::{use_document, use_event_listener, use_interval_fn};
use std::f64;
use web_sys::{wasm_bindgen::JsCast, CanvasRenderingContext2d, HtmlCanvasElement};

fn toggle_fullscreen() {
    let document = use_document();
    let body = document.body().expect("Failed to get body");
    if document
        .fullscreen()
        .expect("Failed to get fullscreen state")
    {
        document.as_ref().unwrap().exit_fullscreen();
    } else {
        body.request_fullscreen()
            .expect("Failed to request fullscreen");
    }
}

#[component]
fn App() -> impl IntoView {
    let fullscreen_visible = || use_document().fullscreen_enabled().unwrap();
    let (is_fullscreen, set_is_fullscreen) = signal(use_document().fullscreen().unwrap());
    let fullscreen_handled = move || is_fullscreen.get() || !fullscreen_visible();

    let _cleanup = use_event_listener(use_document(), fullscreenchange, move |_| {
        *set_is_fullscreen.write() = use_document().fullscreen().unwrap();
    });

    let (game_state, set_game_state) = signal::<game::GameState>(Default::default());

    let _pausable = use_interval_fn(
        move || {
            set_game_state.write().step();
        },
        200,
    );

    view! {
        <div>
            <div class="window-controls">
                <h1>"Blockade 1976, a retro remake"</h1>
                <button on:click={move |_| toggle_fullscreen()}>"Fullscreen"</button>
            </div>
            <Board game_state={game_state} />
        </div>
    }
}

fn draw_board(c: &CanvasRenderingContext2d, game_state: &game::GameState, canvas: &HtmlCanvasElement) {
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
            let y_high = y - cell_height;

            match cell {
                game::Cell::Wall(_wall_type, color) => {
                    let r = color.r * 255.0;
                    let g = color.g * 255.0;
                    let b = color.b * 255.0;

                    c.set_fill_style_str(&format!("rgb({r}, {g}, {b})"));
                    c.fill_rect(x, y_high, cell_width, cell_height);
                }
                game::Cell::Player(player_id) => {
                    let player = &game_state.players[*player_id];
                    let color = player.color;
                    let r = color.r * 255.0;
                    let g = color.g * 255.0;
                    let b = color.b * 255.0;
                    let direction = player.direction;

                    c.set_line_width(2.0);
                    c.set_stroke_style_str(&format!("rgb({r}, {g}, {b})"));
                    c.begin_path();

                    match direction {
                        game::Direction::North => {
                            c.move_to(x, y);
                            c.line_to(x_mid, y_high);
                            c.line_to(x_high, y);
                        }
                        game::Direction::South => {
                            c.move_to(x, y_high);
                            c.line_to(x_mid, y);
                            c.line_to(x_high, y_high);
                        }
                        game::Direction::West => {
                            c.move_to(x, y);
                            c.line_to(x + cell_width, y);
                        }
                        game::Direction::East => {
                            c.move_to(x, y);
                            c.line_to(x - cell_width, y);
                        }
                    }

                    c.stroke();
                }
                game::Cell::Empty => {}
            }
        }
    }
}

#[component]
fn Board(game_state: ReadSignal<game::GameState>) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();

    Effect::new(move || {
        if let Some(canvas) = canvas_ref.get() {
            let game_state = game_state.get();
            let c = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            draw_board(&c, &game_state, &canvas);
        }
    });

    view! {
        <canvas node_ref={canvas_ref} width="640" height="560"></canvas>
        <pre style="text-align:left">{move || format!("{:#?}", game_state.get())}</pre>
    }
}

fn main() {
    leptos::mount::mount_to_body(move || {
        view! {
            <App />
        }
    });
}
