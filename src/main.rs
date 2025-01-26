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

use leptos::{
    ev::{fullscreenchange, keydown},
    html::Canvas,
    logging::log,
    prelude::*,
};
use leptos_use::{
    use_document, use_event_listener, use_interval_fn, use_timeout_fn, use_window, utils::Pausable,
    UseTimeoutFnReturn,
};
use std::f64;
use web_sys::{wasm_bindgen::JsCast, CanvasRenderingContext2d, HtmlCanvasElement};

fn toggle_fullscreen() {
    let document = use_document();

    if document.fullscreen().expect("Failed to get fullscreen") {
        log!("Exiting fullscreen");
        document.as_ref().unwrap().exit_fullscreen();
    } else {
        log!("Entering fullscreen");
        let body = document.body().expect("Failed to get body");
        body.request_fullscreen()
            .expect("Failed to request fullscreen");
    }
}

#[component]
fn App() -> impl IntoView {
    let (debug_mode, set_debug_mode) = signal(false);
    let (is_fullscreen, set_is_fullscreen) = signal(use_document().fullscreen().unwrap());
    let (game_state, set_game_state) = signal::<game::GameState>(Default::default());
    let game_phase = move || game_state.get().phase;

    let _cleanup = use_event_listener(use_document().body(), keydown, move |e| {
        // on Ctrl+D toggle debug mode
        if e.ctrl_key() && e.key() == "d" {
            set_debug_mode.set(!debug_mode.get());
            e.prevent_default();
            return;
        }

        // Player keyboard input
        if game_phase() == game::Phase::Step {
            set_game_state.update(|game_state| {
                match e.key().as_str() {
                    "w" => game_state.players[0].action = game::Direction::North,
                    "a" => game_state.players[0].action = game::Direction::West,
                    "s" => game_state.players[0].action = game::Direction::South,
                    "d" => game_state.players[0].action = game::Direction::East,
                    "ArrowUp" => game_state.players[1].action = game::Direction::North,
                    "ArrowLeft" => game_state.players[1].action = game::Direction::West,
                    "ArrowDown" => game_state.players[1].action = game::Direction::South,
                    "ArrowRight" => game_state.players[1].action = game::Direction::East,
                    _ => (),
                }
                e.prevent_default();
                log!("Player 0: {:?}", game_state.players[0].action);
                log!("Player 1: {:?}", game_state.players[1].action);
            });
        }
    });

    let _cleanup = use_event_listener(use_document(), fullscreenchange, move |_| {
        set_is_fullscreen.set(use_document().fullscreen().unwrap());
    });

    Effect::new(move || match game_phase() {
        game::Phase::Step => {
            use_interval_fn(
                move || {
                    log!("Step");
                    let mut game_state = set_game_state.write();
                    let next_phase = game_state.step();
                    game_state.phase = next_phase;
                },
                200,
            );
        }
        game::Phase::Score => {
            let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
                move |_: ()| {
                    set_game_state.update(|game_state| game_state.next_round());
                },
                2000.0,
            );

            start(());
        }
        game::Phase::GameOver => {
            log!("Game Over");
        }
    });

    view! {
        <div>
            <div class="window-controls">
                <h1>"Blockade 1976, a retro remake"</h1>
                <button on:click={move |_| toggle_fullscreen()}>
                    {move || if is_fullscreen.get() { "Exit Fullscreen" } else { "Fullscreen" }}
                </button>
            </div>
            <Board game_state={game_state} debug_mode={debug_mode} />
        </div>
    }
}

fn draw_wall(
    wall_type: &game::WallType,
    color: &game::Color,
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
        game::WallType::Horizontal => {
            c.begin_path();
            c.move_to(x, y + half_height);
            c.line_to(x + width, y + half_height);
            c.stroke();
        }
        game::WallType::Vertical => {
            c.begin_path();
            c.move_to(x + half_width, y);
            c.line_to(x + half_width, y + height);
            c.stroke();
        }
        game::WallType::CornerTopLeft => {
            c.begin_path();
            c.move_to(x + half_width, y + height);
            c.line_to(x + half_width, y + half_height);
            c.line_to(x + width, y + half_height);
            c.stroke();
        }
        game::WallType::CornerTopRight => {
            c.begin_path();
            c.move_to(x, y + half_height);
            c.line_to(x + half_width, y + half_height);
            c.line_to(x + half_width, y + height);
            c.stroke();
        }
        game::WallType::CornerBottomLeft => {
            c.begin_path();
            c.move_to(x + half_width, y);
            c.line_to(x + half_width, y + half_height);
            c.line_to(x + width, y + half_height);
            c.stroke();
        }
        game::WallType::CornerBottomRight => {
            c.begin_path();
            c.move_to(x, y + half_height);
            c.line_to(x + half_width, y + half_height);
            c.line_to(x + half_width, y);
            c.stroke();
        }
    }
}

fn draw_board(
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
                game::Cell::Wall(wall_type, color) => {
                    draw_wall(wall_type, color, c, x, y_high, cell_width, cell_height)
                }
                game::Cell::Player(player_id) => {
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
                        game::Direction::North => {
                            c.move_to(x + 2.0, y);
                            c.line_to(x_mid, y_high + 2.0);
                            c.line_to(x_high - 2.0, y);
                        }
                        game::Direction::South => {
                            c.move_to(x + 2.0, y_high);
                            c.line_to(x_mid, y - 2.0);
                            c.line_to(x_high - 2.0, y_high);
                        }
                        game::Direction::West => {
                            c.move_to(x_high, y + 2.0);
                            c.line_to(x, y_mid);
                            c.line_to(x_high, y_high - 2.0);
                        }
                        game::Direction::East => {
                            c.move_to(x, y + 2.0);
                            c.line_to(x_high, y_mid);
                            c.line_to(x, y_high - 2.0);
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
fn Board(game_state: ReadSignal<game::GameState>, debug_mode: ReadSignal<bool>) -> impl IntoView {
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
        <Show when=move || !debug_mode.get()
            fallback=move || view! {
                <div>
                    <pre style="text-align:left">{format!("{:#?}", game_state.get())}</pre>
                </div>
            }>
            <div class="board">
                <canvas class="cell" node_ref={canvas_ref} width="640" height="560"></canvas>
                <div class="cell">
                    <div class="rounds">{game_state.get().rounds}</div>
                </div>
            </div>
        </Show>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(move || {
        view! {
            <App />
        }
    });
}
