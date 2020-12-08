<div align="center">
    <h1>RecWars</h1>
    <i>Recreational Warfare .rs</i>
    <br />
    A multiplayer top-down tank shooter - Rust/WASM port of an old Windows game called RecWar.
</div>
<br />

[![Discord](https://img.shields.io/discord/770013530593689620?label=discord)](https://discord.gg/9BQVVgV)

<!-- Note to my future OCD: The ideal image width for github is 838 pixels -->
[![Gameplay](media/screenshot.jpg)](https://martin-t.gitlab.io/gitlab-pages/rec-wars/web)

_**[Play Online](https://martin-t.gitlab.io/gitlab-pages/rec-wars/web)**_

RecWars is a free and open source clone of [RecWar](#the-original-game) - you control a vehicle and fight other vehicles in a variety of game modes using an arsenal of several distinct weapons. You can play against bots, in splitscreen and over the network.

RecWars aims to have gameplay similar but not identical to RecWar. I suspect RecWar was balanced for playing against bots and might result in annoying strats being the most effective when people start [playing to win](http://www.sirlin.net/articles/playing-to-win). However, almost everything in RecWars is [configurable](#cvars) and you can switch to the original RecWar balance to [compare](#RecWars vs RecWar differences).

**Currently this is very much a work-in-progress**: only some weapons work, the driving physics don't feel right, there are no collisions between vehicles, ...

The ultimate goal is to create a moddable multiplayer game playable in the browser and natively on Linux, Windows and macOS. This might be tricky since WASM in the browser doesn't allow UDP. I have some ideas how to solve that.

(Planned) Features
------------------

- [ ] Bots
- [ ] Multiplayer
    - [ ] Splitscreen
    - [ ] Network
    - [ ] Combination of both (plus bots)
- [x] [Browser client](https://martin-t.gitlab.io/gitlab-pages/rec-wars/web)
- [ ] Native client
- [ ] Game modes
    - [ ] Free For All
    - [ ] Team War
    - [ ] Capture The Cow
- [x] [Highly configurable](#cvars)

Cvars
-----

Cvars are *console variables* - configuration settings which control everything in the game like physics, weapon behavior, AI, HUD layout, etc.

There are two ways to change them:
- Edit the `cvars` object using the browser console - e.g. `cvars.g_armor = 100`.
- Set them using URL parameters - e.g. [https://martin-t.gitlab.io/gitlab-pages/rec-wars/web/?g_armor=100](https://martin-t.gitlab.io/gitlab-pages/rec-wars/web/?g_armor=100)

The entire list of cvars is in [src/cvars.rs](src/cvars.rs).

Dependencies
------------

- [wasm-pack](https://github.com/rustwasm/wasm-pack) - use the [installer](https://rustwasm.github.io/wasm-pack/installer/)

Compiling
---------

- build with `wasm-pack build --target web --dev`
    - you can replace `--dev` with `--profiling` or `--release` if perf is an issue (see [Cargo.toml](Cargo.toml) for more info)
- host with `python3 -m http.server` (or any other web server, simply opening `index.html` will *not* work though)
- open http://localhost:8000/web/

Contributing
------------

If you notice a bug or have a suggestion, please [open an Issue](https://github.com/martin-t/rec-wars/issues/new).

If you'd like to improve RecWars, feel free to make a [Pull Request](https://github.com/martin-t/rec-wars/pulls). I want to make RecWars highly configurable with many different gamemodes and balance settings votable by players and anybody will be able to host their own server (if technically possible even from the browser). If you have a gameplay idea and don't suffer from the NIH syndrome, I'd be very happy to help you test it in RecWars.

### Architecture Overview

Most of the code is commented to be understandable to anyone with a vague idea of how a game works. If it's not clear why a particular piece of code exists or why it needs to be written the way it is, I consider that a bug which should be fixed by either rewriting the code more clearly or adding comments explaining it.

RecWars is written in Rust with a small bit of JS glue. It does *not* depend on NPM. Currently, it draws directly to an HTML5 canvas using the 2D API which turns out to be too slow to redraw the entire screen at 60Hz. I am still deciding between [macroquad](https://github.com/not-fl3/macroquad), [luminance](https://github.com/phaazon/luminance-rs) and [wgpu-rs](https://github.com/gfx-rs/wgpu-rs).

Currently, most game state is managed by generational arenas from the [thunderdome](https://github.com/LPGhatguy/thunderdome) crate to make the code type-safe and readable. Previously, RecWars used the [legion](https://github.com/amethyst/legion) ECS. However it was cumbersome to use and WASM didn't get any benefits from parallelism. The only reason I was using ECS was so I could have references between entities and for this I was paying by having all entities dynamicly typed which lead to bugs. It's a Rust tradition to start writing a game and end up writing a game engine or ECS so I am considering creating an ECS crate that would satisfy my standards of clean API and static typing. For now arenas seem to be close enough.

The Original Game
-----------------

RecWar by Willem Janssen:
- homepage: http://recreationalwarfare.atspace.com/index_willem.html (the game's download is broken but still hosts extra maps)
- unofficial homepage: http://www.recwar.50webs.com/
- archive.org download: https://archive.org/details/recwar_201903
- archive.org download with extra maps: https://archive.org/details/RecWar

The original RecWar only contains a Windows .exe but runs ok-ish wine (sometimes freezes on map load). It includes a map editor. The binaries in both archive.org links are identical to what I got on an old CD so should be safe.

### RecWars vs RecWar differences

RecWar would probably be impossible to replicate exactly without decompiling the binary (which doesn't even contain debug symbols), though if a fan of the original finds this project, I am not gonna stop them from trying.

Additionally, when playing against people instead of bots, I suspect RecWar's original balance would lead to annoying and boring strats like making the cow inaccessible with mines or just simple camping. For example, experience from poorly designed games shows large areas will be dominated by instant-hit weapons (in RecWar the railgun) and there might simply be no way to get across the map alive. Therefore I made the railgun a very fast projectile in RecWars. I might make more balance changes based on how the online gameplay evolves.

The two balance presets are available here:
- https://martin-t.gitlab.io/gitlab-pages/rec-wars/web/?balance=recwars
- https://martin-t.gitlab.io/gitlab-pages/rec-wars/web/?balance=recwar

Known ways RecWars differs from RecWar:
- Speeds, accelerations, turning, inertia of vehicles and weapons - I will make best effort here but it won't be exact
- Push force of mines and railguns
- Tank in RecWar turned around turret swivel point, not center of chassis
- Weapons
    - Damage - Cluster bomb and BFG beam are hard to measure exactly
    - Spreads - Cluster bombs and MG are hard to measure exactly
    - Railgun - RecWars uses a very fast projectile because hitscan weapons ruin large maps
- Self destruct damage and range - it appears to be the only explosion in RecWar with damage decreasing by distance and it's really hard to measure exactly.

Maps
----

- `maps/` - Maps from the original RecWar
- `maps/extra/` - Extra maps from the official homepage
- `maps/extra2/` - Extra maps from archive.org

Currently the map is picked randomly by default, however, you can select one manually by using the `map` URL parameter, for example [https://martin-t.gitlab.io/gitlab-pages/rec-wars/web/?map=Castle Islands (4)](https://martin-t.gitlab.io/gitlab-pages/rec-wars/web/?map=Castle%20Islands%20(4)).

License
-------

<!-- When updating this, also update LICENSE and Cargo.toml -->
All code is available under [AGPL-v3](agpl-3.0.txt) or newer.

All assets (maps, textures, sounds, etc.) are taken from the original RecWar by Willem Janssen which is freely available online.
