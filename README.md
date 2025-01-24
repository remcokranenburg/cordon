# Blockade 1976, a Retro Remake

Originally created by Gremlin, Blockade was an arcade game and the origin of
two genres: light cycle games (later popularized by Tron) and snake-like games
(later popularized by Nokia). Each player controls an arrow on the screen that
lays a piece of brick wall as it moves forward. You can make turns and you must
take care not to run into the edge of the board or the walls of the other
player.

Gremlin quickly published two sequels to Blockade: Comotion (1976) and
Hustle (1977). Comotion is a four-player variant of the original game with
slightly altered scoring rules, and Hustle is a two-player snake-like game.

## How to run

Make sure you have the right tooling:

```
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Start local dev server:

```
cd app_leptos
trunk serve --open
```

## Remco's Retro Remakes

I want to make video games as a hobby, and I like exploring history, so that's
how I come to the following project: remake old video games to make them easily
available on modern hardware and with modern amenities like network
multiplayer, high resolution graphics and controller support. Older games are
simpler than modern ones, making it feasible to do this as a hobby.

Some sub-goals I want to reach for each game:

1. Playable on modern hardware, installable with a single click (or playable
   from the web)
2. In-game history lesson about the game
3. High-resolution graphics (sharper fonts, better colors, hi-res textures),
   but same style
4. Modern controls (keyboard + mouse, gamepad)
5. Multiplayer: same-computer, LAN and online

Obviously, I don't want to cause trouble for the original creators: this is a
project of homage to the classics, so there are a few rules:

1. No copying of code or art assets: it's all self-made
2. 100% clarity that this is not the original game, but an unrelated remake
3. The original game must be older than 20 years, so we are not harming profits

It *is* allowed to faithfully recreate the game mechanics and the 'atmosphere'
of the game, because that is the essence of playing these old games. I also
believe that the rules of a game cannot be owned by anyone.

## License

Copyright 2025 Remco Kranenburg <remco@burgsoft.nl>

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

SPDX-License-Identifier: AGPL-3.0-or-later