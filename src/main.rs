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

mod common;
mod game;
mod layout;
mod render;

use leptos::{
    ev::{fullscreenchange, keydown},
    html::Canvas,
    logging::log,
    prelude::*,
};
use leptos_use::{use_document, use_event_listener, use_interval_fn, use_window};
use web_sys::{wasm_bindgen::JsCast, CanvasRenderingContext2d, KeyboardEvent};

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

fn handle_action(e: &KeyboardEvent, player: &mut game::Player, direction: common::Direction) {
    player.set_direction(direction);
    e.stop_propagation();
    e.prevent_default();
}

#[component]
fn App() -> impl IntoView {
    let (debug_mode, set_debug_mode) = signal(false);
    let (is_fullscreen, set_is_fullscreen) = signal(use_document().fullscreen().unwrap());
    let (game_state, set_game_state) = signal::<game::GameState>(Default::default());
    let game_phase = Memo::new(move |_| game_state.get().phase);

    Effect::new(move || match game_phase.get() {
        game::Phase::Step => {
            use_interval_fn(
                move || {
                    set_game_state.update(|s| s.tick());
                },
                150,
            );
        }
        game::Phase::Score => {
            let num_explosion_frames = 20;
            let (explosion_frame, set_explosion_frame) = signal(0);

            use_interval_fn(
                move || {
                    if explosion_frame.get() >= num_explosion_frames {
                        set_game_state.update(|s| s.tick());
                    } else {
                        set_explosion_frame.update(|x| *x += 1);
                    }
                },
                100,
            );
        }
        game::Phase::GameOver => {
            log!("Game Over");
        }
    });

    let _cleanup = use_event_listener(use_window(), keydown, move |e| {
        // on Ctrl+D toggle debug mode
        if e.ctrl_key() && e.key() == "d" {
            set_debug_mode.set(!debug_mode.get());
            e.prevent_default();
            return;
        }

        // Player keyboard input
        if game_phase.get() == game::Phase::Step {
            set_game_state.update(|game_state| match e.key().as_str() {
                "w" => handle_action(&e, &mut game_state.players[0], common::Direction::North),
                "a" => handle_action(&e, &mut game_state.players[0], common::Direction::West),
                "s" => handle_action(&e, &mut game_state.players[0], common::Direction::South),
                "d" => handle_action(&e, &mut game_state.players[0], common::Direction::East),
                "ArrowUp" => {
                    handle_action(&e, &mut game_state.players[1], common::Direction::North)
                }
                "ArrowLeft" => {
                    handle_action(&e, &mut game_state.players[1], common::Direction::West)
                }
                "ArrowDown" => {
                    handle_action(&e, &mut game_state.players[1], common::Direction::South)
                }
                "ArrowRight" => {
                    handle_action(&e, &mut game_state.players[1], common::Direction::East)
                }
                _ => (),
            });
        }
    });

    let _cleanup = use_event_listener(use_document(), fullscreenchange, move |_| {
        set_is_fullscreen.set(use_document().fullscreen().unwrap());
    });

    view! {
        <div>
            <div class="window-controls">
                <h1>"Cordon"</h1>
                <button on:click={move |_| toggle_fullscreen()}>
                    {move || if is_fullscreen.get() { "Exit Fullscreen" } else { "Fullscreen" }}
                </button>
            </div>
            <Board game_state={game_state} debug_mode={debug_mode} />
        </div>
    }
}

#[component]
fn Board(game_state: ReadSignal<game::GameState>, debug_mode: ReadSignal<bool>) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let width = game_state.get().grid_width;
    let height = game_state.get().grid_height;
    let mut grid = layout::Grid::new(width, height, &game_state.get());

    Effect::new(move || {
        if let Some(canvas) = canvas_ref.get() {
            let c = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            // update grid with game state
            // TODO: don't replace the whole grid on every update
            grid.reset(&game_state.get());

            render::draw_board(&c, grid.get_data(), &canvas);
        }
    });

    view! {
        <Show when=move || !debug_mode.get()
            fallback=move || view! {
                <div>
                    <p>max_score: {game_state.get().max_score}</p>
                    <pre style="text-align:left">{format!("{:#?}", layout::Grid::new(width, height, &game_state.get()))}</pre>
                    <p>active_player: {game_state.get().active_player}</p>
                    <p>phase: {format!("{:?}", game_state.get().phase)}</p>
                </div>
            }>
            <div class="board">
                <canvas class="cell" node_ref={canvas_ref} width="640" height="560"></canvas>
                <div class="cell">
                    <div class="rounds">{game_state.get().max_score}</div>
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
