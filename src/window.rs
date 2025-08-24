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

use bevy::{
    prelude::*,
    render::camera::Viewport,
    window::{WindowResized, WindowResolution, WindowTheme},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            title: "Cordon".into(),
            window_theme: Some(WindowTheme::Dark),
            resolution: WindowResolution::new(1152.0, 1008.0),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.1)));
    app.add_systems(Startup, setup);
    app.add_systems(Update, window_resize_system);
}

fn calc_viewport_size(window_width: u32, window_height: u32) -> UVec2 {
    if window_width > window_height {
        UVec2 {
            x: window_height * 32 / 28,
            y: window_height,
        }
    } else {
        UVec2 {
            x: window_width,
            y: window_width * 28 / 32,
        }
    }
}

fn calc_viewport_position(window_size: UVec2, viewport_size: UVec2) -> UVec2 {
    UVec2 {
        x: (window_size.x / 2).saturating_sub(viewport_size.x / 2),
        y: (window_size.y / 2).saturating_sub(viewport_size.y / 2),
    }
}

pub fn setup(mut commands: Commands, window: Single<&Window>) {
    let window_size = window.resolution.physical_size();
    let viewport_size = calc_viewport_size(window_size.x, window_size.y);
    let viewport_position = calc_viewport_position(window_size, viewport_size);

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: viewport_position,
                physical_size: viewport_size,
                ..Default::default()
            }),
            ..Default::default()
        },
    ));
}

fn window_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut camera: Single<&mut Camera>,
    windows: Query<&Window>,
) {
    for event in resize_reader.read() {
        if let Ok(window) = windows.get(event.window) {
            let window_size = window.resolution.physical_size();
            let viewport_size = calc_viewport_size(window_size.x, window_size.y);
            let viewport_position = calc_viewport_position(window_size, viewport_size);
            camera.viewport.as_mut().map(|viewport| {
                viewport.physical_size = viewport_size;
                viewport.physical_position = viewport_position;
            });
        }
    }
}
