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

mod bot;
mod common;
mod game;
mod layout;
mod render;

use game::GameState;
use leptos::{
    ev::{fullscreenchange, keydown},
    html::Canvas,
    logging::log,
    prelude::*,
};
use leptos_use::{
    use_active_element, use_document, use_event_listener, use_interval_fn, use_window,
};
use web_sys::{
    wasm_bindgen::JsCast, CanvasRenderingContext2d, HtmlElement, KeyboardEvent, NodeList,
};

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

fn start_game(
    num_players: usize,
    set_menu_page: WriteSignal<Option<MenuPage>>,
    set_game_state: WriteSignal<game::GameState>,
) {
    set_menu_page.set(None);
    set_game_state.set(GameState::new(num_players, 3));
}

#[derive(Debug, Clone)]
enum MenuPage {
    Main,
    NewGame,
    Settings,
    About,
}

#[component]
fn Menu(
    menu_page: ReadSignal<Option<MenuPage>>,
    set_menu_page: WriteSignal<Option<MenuPage>>,
    set_game_state: WriteSignal<game::GameState>,
    is_fullscreen: ReadSignal<bool>,
) -> impl IntoView {
    move || {
        match menu_page.get().expect("menu page should be set") {
        MenuPage::Main => view! {
            <div class="center">
                <div class="menu">
                    <h1>"Cordon"</h1>
                    <div class="items">
                        <button on:click={move |_| set_menu_page.set(Some(MenuPage::NewGame))}>
                            "New Game"
                        </button>
                        <button on:click={move |_| set_menu_page.set(Some(MenuPage::Settings))}>
                            "Settings"
                        </button>
                        <button on:click={move |_| set_menu_page.set(Some(MenuPage::About))}>
                            "About"
                        </button>
                    </div>
                </div>
            </div>
        }
        .into_any(),
        MenuPage::NewGame => view! {
            <div class="center">
                <div class="menu">
                    <h1>"New Game"</h1>
                    <div class="items">
                        <button on:click={move |_| start_game(1, set_menu_page, set_game_state)}>
                            "One Player"
                        </button>
                        <button on:click={move |_| start_game(2, set_menu_page, set_game_state)}>
                            "Two Players"
                        </button>
                    </div>
                    <button on:click={move |_| set_menu_page.set(Some(MenuPage::Main))}>
                        "Back"
                    </button>
                </div>
            </div>
        }
        .into_any(),
        MenuPage::Settings => view! {
            <div class="center">
                <div class="menu">
                    <h1>"Settings"</h1>
                    <div class="items">
                        <button on:click={move |_| toggle_fullscreen()}>
                            {move || if is_fullscreen.get() { "Exit Fullscreen" } else { "Fullscreen" }}
                        </button>
                    </div>
                    <button on:click={move |_| set_menu_page.set(Some(MenuPage::Main))}>
                        "Back"
                    </button>
                </div>
            </div>
        }
        .into_any(),
        MenuPage::About => view! {
            <div class="center">
                <div class="menu">
                    <h1>About</h1>

                    <div class="items">
                        <p>
                            <i>Cordon</i> " is a game where you draw walls to trap your opponent,
                            like Tron or Snake. You can play against a bot or another player."
                        </p>
                        <p>
                            "This is a remake of " <i>Blockade</i> " (1976), originally created by
                            Gremlin. Two sequels were made: " <i>CoMOTION</i> " (1976) and " <i>Hustle</i>
                            " (1977)."
                        </p>
                        <p>
                            "This game is free software, licensed under the GNU Affero General Public
                            License version 3 or later. The source code is available on "
                            <a href="https://github.com/remcokranenburg/cordon">"GitHub"</a> "."
                        </p>
                    </div>

                    <button on:click={move |_| set_menu_page.set(Some(MenuPage::Main))}>
                        "Back"
                    </button>
                </div>
            </div>
        }.into_any(),
    }
    }
}

#[component]
fn App() -> impl IntoView {
    // signals
    let (menu_page, set_menu_page) = signal(Some(MenuPage::Main));
    let (debug_mode, set_debug_mode) = signal(false);
    let (is_fullscreen, set_is_fullscreen) = signal(use_document().fullscreen().unwrap());
    let (game_state, set_game_state) = signal(GameState::new(0, 6));
    let game_phase = memo!(game_state.phase);
    let max_score = memo!(game_state.max_score);
    let active_player = memo!(game_state.active_player);

    // variables
    let canvas_ref = NodeRef::<Canvas>::new();
    let width = game_state.get().grid_width;
    let height = game_state.get().grid_height;
    let mut grid = layout::Grid::new(width, height, &game_state.get());

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
            use_interval_fn(move || set_game_state.update(|s| s.tick()), 2000);
        }
        game::Phase::GameOver => {
            set_menu_page.set(Some(MenuPage::Main));
            log!("Game Over");
        }
    });

    let _cleanup = use_event_listener(use_window(), keydown, move |e| {
        let nav_prev = ["ArrowUp", "w"];
        let nav_next = ["ArrowDown", "s"];
        let nav_keys = [nav_prev, nav_next].concat();
        let key = e.key();
        let ctrl = e.ctrl_key();

        if ctrl && key == "d" {
            // Debug mode: Ctrl + D
            set_debug_mode.set(!debug_mode.get());
            e.prevent_default();
            return;
        } else if menu_page.get().is_some() && nav_keys.contains(&key.as_str()) {
            // Menu keyboard input
            let menu_items: NodeList = use_document()
                .query_selector_all("button")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            let first_item: HtmlElement = menu_items
                .item(0)
                .expect("Menu should have at least one item")
                .dyn_into()
                .unwrap();

            'focus_next: {
                match use_active_element().get() {
                    Some(element) => {
                        // If the active element is in the list of buttons, go to next or previous
                        for (i, button) in menu_items.values().into_iter().enumerate() {
                            let i = i as u32;
                            if element == button.unwrap().dyn_into().unwrap() {
                                let next_button = menu_items
                                    .item(if nav_prev.contains(&key.as_str()) {
                                        i.checked_sub(1).unwrap_or(menu_items.length() - 1)
                                    } else if nav_next.contains(&key.as_str()) {
                                        i + 1
                                    } else {
                                        0
                                    })
                                    .unwrap_or(menu_items.item(0).unwrap())
                                    .dyn_into::<HtmlElement>()
                                    .unwrap();
                                next_button.focus().unwrap();
                                e.prevent_default();
                                break 'focus_next;
                            }
                        }

                        // If the active element is not in the menu, focus the first button
                        first_item.focus().unwrap();
                    }
                    None => {
                        // If there is no active element, focus the first button
                        first_item.focus().unwrap();
                    }
                }
            }
        } else if game_phase.get_untracked() == game::Phase::Step {
            // Player keyboard input
            set_game_state.update(|game_state| {
                for player in game_state.players.iter_mut() {
                    match player.controller {
                        game::Controller::Wasd => match e.key().as_str() {
                            "w" => handle_action(&e, player, common::Direction::North),
                            "a" => handle_action(&e, player, common::Direction::West),
                            "s" => handle_action(&e, player, common::Direction::South),
                            "d" => handle_action(&e, player, common::Direction::East),
                            _ => (),
                        },
                        game::Controller::Arrows => match e.key().as_str() {
                            "ArrowUp" => handle_action(&e, player, common::Direction::North),
                            "ArrowLeft" => handle_action(&e, player, common::Direction::West),
                            "ArrowDown" => handle_action(&e, player, common::Direction::South),
                            "ArrowRight" => handle_action(&e, player, common::Direction::East),
                            _ => (),
                        },
                        game::Controller::Bot => (),
                        _ => unimplemented!(),
                    }
                }
            });
        }
    });

    let _cleanup = use_event_listener(use_document(), fullscreenchange, move |_| {
        set_is_fullscreen.set(use_document().fullscreen().unwrap());
    });

    Effect::new(move || {
        if let Some(canvas) = canvas_ref.get() {
            let rect = canvas.get_bounding_client_rect();
            canvas.set_width(rect.width() as u32);
            canvas.set_height(rect.height() as u32);

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
                    <p>max_score: {max_score}</p>
                    <pre style="text-align:left">{format!("{:#?}", layout::Grid::new(width, height, &game_state.get()))}</pre>
                    <p>active_player: {active_player}</p>
                    <p>phase: {format!("{:?}", game_phase.get())}</p>
                </div>
            }>
                <canvas node_ref={canvas_ref}></canvas>
                <div>
                    <div class="rounds">{max_score}</div>
                </div>
                <Show when=move || menu_page.get().is_some()>
                    <div>
                        <Menu menu_page set_menu_page set_game_state is_fullscreen />
                    </div>
                </Show>
        </Show>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(move || {
        view! {
            <div class="layers">
                <App />
            </div>
        }
    });
}
