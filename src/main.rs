// TODO MVP:
// [x] get simple.wasm working
// [x] compile rust to WASM, load that
// [x] canvas
// [x] render background
// [x] move around, check perf
//      https://github.com/mrdoob/stats.js/
//      [ ] make explosion sprite smaller
//      https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas
//          [x] round to whole pixels (orig RW does it)
//          [ ] webgl
// [x] pick a math lib, don't overthink it
// [ ] engine? rendering crate? nice if native and WASM backends
//      [x] check https://arewegameyet.rs/ecosystem/engines/
//      [x] check https://github.com/dasifefe/rust-game-development-frameworks
//      engines
//          godot
//          https://crates.io/crates/amethyst
//              WASM in progress: https://github.com/amethyst/amethyst/issues/2260
//          https://crates.io/crates/bevy
//              no WASM yet: https://github.com/bevyengine/bevy/issues/88
//          https://crates.io/crates/coffee
//              no WASM: https://github.com/hecrj/coffee/issues/135 (not even a reply)
//          https://crates.io/crates/ggez
//              WASM in progress: https://github.com/ggez/ggez/issues/71
//          https://crates.io/crates/macroquad
//              win/lin/mac + WASM + android
//              bad docs, some examples
//          https://crates.io/crates/piston
//              no WASM: https://github.com/PistonDevelopers/piston/issues/1131
//          https://crates.io/crates/quicksilver
//              win/lin/max + WASM
//              no audio in 0.4?
//                  https://github.com/ryanisaacg/quicksilver/issues/552
//                  https://github.com/RustyVolley/RustyVolleySrc in 0.3 has sound
//              8 example games in readme (mostly bad)
//          macroquad - no docs?
//      rendering only
//          https://crates.io/crates/luminance - mentions webgl/canvas
//          https://crates.io/crates/miniquad - should support WASM
//          wgpu?
//      profiling
//          https://github.com/EmbarkStudios/puffin
//          https://github.com/bombomby/optick-rs
//          list of tools: Instruction counts @ https://blog.mozilla.org/nnethercote/2020/09/08/how-to-speed-up-the-rust-compiler-one-last-time/
// [x] image fails to draw the first time after opening browser
//      bug in python server maybe? - doesn't happen with rhino
// [ ] server framerate when minimized - https://developer.mozilla.org/en-US/docs/Web/API/Window/setImmediate#Notes
//      postMessage / MessageChannel / https://github.com/YuzuJS/setImmediate polyfill
// [ ] extract/screenshot/record assets from RecWar or find alternatives
//      [ ] images
//          [x] weapon icons
//          [ ] weapons
//          [ ] vehicles + skins
//          [ ] cow, stolen effects
//          [x] wrecks
//      [ ] sounds
//          [ ] weapons, explosions, self destruct
//          [ ] cow
//          [ ] engine noise
//              [ ] how does it change with speed?
// [ ] CI - GH actions / travis, mirror to GL???
// [x] load maps
// [x] cvars
//      https://crates.io/crates/cvar
//          no tab completion in js console
//          https://crates.io/crates/amethyst-console - uses cvar and imgui
// [x] ECS
//      hecs
//          - can't delete while iterating
//          - no clone / snapshots
//          + ffa example
//          + nested queries (takes &World even for write access)
//          + slightly faster in WASM, especially debug mode
//          - no resources, commands
//      legion
//          - can't delete while iterating
//          + should be able to make a clone of the world (clone_from)
//          - bad docs / no examples / 0.3 transition
//          - how to do nested queries without copying? split world? maybe systems make this easier?
//          + slightly slower in WASM, especially debug mode
//          + resources, commands?
// [ ] render vehicles
//      [x] basic tank
//      [ ] skins, colors
//              canvas imageData?
//      [x] vehicle types
// [x] explosions
//      [x] sizes
// [ ] movement
//      [ ] tank
//      [ ] hovercraft
//      [ ] hummer + reverse steering
//          https://engineeringdotnet.blogspot.com/2010/04/simple-2d-car-physics-in-games.html
//          https://www.asawicki.info/Mirror/Car%20Physics%20for%20Games/Car%20Physics%20for%20Games.html
// [ ] collision detection with proper traces
//      [ ] walls
//      [ ] vehicles
// [ ] physics
//      [ ] surfaces
//      [ ] pushing wrecks
//      [ ] pushing moving vehicles
// [ ] hp/health
//      [ ] wrecks (vehicle turned into wreck immediately on hit)
//      [ ] configurable
// [ ] weapons
//      [ ] MG
//          [x] 50 per reload
//          [x] about 2.5 s to empty magazine -> refire 50 ms
//          [ ] shoot faster on hovercraft, more ammo too?
//          [ ] shooting to the side while moving looks awful
//      [ ] rail
//          [ ] push
//      [ ] CB
//          [x] 40 per shot
//          [x] next shot starts a few frames after last explosion disappears
//          [x] explosions happen on walls, just partially obscured
//          [ ] reflect off map edge
//          [ ] hummer - originate from entire width of turret
//          orig RW:
//              size 1, shadow offset 1
//              40 frames to first explosion, 58 to last
//              distance - 80-300 while moving slowly backwards
//      [ ] rockets
//          [ ] acceleration (judging by changing distance between them)
//          [x] 6 per reload
//          [x] about 1 s between first and last rocket -> refire 200 ms
//          [ ] hummer more + refire + entire width
//      [ ] homing missile
//      [ ] guided missile
//      [ ] BFG
//          [ ] shot delay + random dir when dead
//          [ ] beam range
//          [x] explosion animation goes backwards
//          My explosion spritesheet is slightly different from original RW, i experimented with GIMP and this is the closest i got.
//              Would be nice to figure out what exactly it did.
//          In orig RW, shooting at a wall could kill enemy on the other side but only if low hp - probably clipped into wall and beam killed him.
//          [ ] flashing color
//      [x] proper shot origins
//      [x] proper reload times
//      [ ] rockets?, missiles and bfg seem to have terminal velocity - force+friction?
//      [ ] avoid hitting self (orig RW had hummer chassis hardpoint more forward)
//          [ ] allow hitting self if missile comes back after clearing the hitbox
//      [ ] entity culling? faster to render?
//      [ ] check collision detection works if gun is inside another vehicle
// [ ] mines
//      [ ] not within radius of base and/or cow
// [x] turrets
//      [ ] 8 angles
//          original RW has a bug: quick left,left,right would result in turning the longer way around
//          better control scheme - changing direction cancels the queue, starts counting from current position
// [ ] shadows
//      [ ] HUD
//      [ ] vehicles (is turret "higher"?)
//      [ ] CB
// [ ] self destruct
//      [ ] bigger exposion
//      [ ] countdown
//      [ ] sounds
//      [ ] works through walls
// [ ] player separate from vehicle
// [ ] what happens to Entity handles after player disconnect
// [ ] UI
//      requirements
//          minimum: select mode, map, bots, start game
//          server list? quick join? start server?
//          tank customization (only pattern, colors, name - vehicle type should be changeable in-game)
//          binds (show keyboard image?)
//              comfortable for 2 players, discoverable
//              check all the downloaded RW configs, RW2 and RW3 (copy config / firejail)
//      maybe https://www.sitepoint.com/css-layouts-floats-flexbox-grid/
// [ ] fullscreen
// [ ] FFA
// [ ] TW
// [ ] CTC
//      [ ] cow movement
//      crab instead? easter-egg only (temporarily? - e.g. after making a circlearound)?
// [ ] bot AI
//      [ ] pathfinding - custom / from soko solver / crate?
//          not just 8 directions but any angle
//          look at what veloren does
//          https://old.reddit.com/r/rust_gamedev/comments/hr7m3j/which_lib_do_you_use_for_pathfinding_in_you_games/
//          https://github.com/BezPowell/blitz-path
//          https://github.com/Winsalot/AutumnRTS/
//      [ ] roles / commands
// [ ] decent default binds (2x for splitscreen)
// [ ] hosting
//      GH pages? - needs public repo first
//      domain? SSL?
//      [ ] GH Social preview
//      master server? cloudflare, heroku, google app engine?
//          needs to be stateful
//          https://github.com/Ralith/masterserve
//      if dedicated servers, need at least 2 - EU and US
// [x] icon
// [ ] network
//      https://arewegameyet.rs/ecosystem/networking/
//          https://crates.io/crates/quinn - what is QUIC?
//          https://crates.io/crates/laminar - mentions only UDP, not WASM
//      WASM doesn't allow UDP sockets
//          https://gafferongames.com/post/why_cant_i_send_udp_packets_from_a_browser/
//          https://www.html5rocks.com/en/tutorials/webrtc/datachannels/
//          TCP
//          webRTC
//          crazy idea: multiple TCP streams, rotate through them
//      https://arewegameyet.rs/ecosystem/networking/ or custom?
//      [ ] master server hosting
//      [ ] prediction / reconciliation
//          braid / jonathan blow - fps independent
//          https://github.com/pond3r/ggpo/tree/master/doc
// [ ] chat
// [ ] voting
//      [ ] mode
//      [ ] map
//      [ ] cvars
// [ ] analytics
//      https://simpleanalytics.com/sandspiel.club - paid only
//      [x] google analytics
//      [ ] master server
// [ ] FAQ - stuttering/tearing due to compositor - Alt+Shift+f12 - somehow this doesn't work anymore
//      note to self - renderdoc for graphics debugging
// [ ] version info
// [ ] focus convas on (re)load (e.g. after touching browser console and reloading)
// nice to have:
// [ ] GM - presing fire again switches back to tank
// [ ] record seed+input (WASM should be reproducible when avoiding NaNs) - replay, debugging
// [ ] allow MG to shoot down missiles
// [ ] make driving feel more real - simulate terrain unevenness? mild speed/angle changes?
// [ ] map editor - sharing maps, voting, recommended mode / number of bots
//      [ ] bots say hi/gg/sry/n1
// [ ] log of past games (to show activity even if nobody currently online)
// [ ] splitscreen
// [ ] figure out what webpack is and how to create a static site
//      probably best template: https://github.com/rustwasm/rust-webpack-template
//      alternative: https://github.com/thedodd/trunk
//      note to self for dealing with npm on debian:
//          https://stackoverflow.com/questions/16151018/npm-throws-error-without-sudo/24404451#24404451
//          put in the *beginning* of PATH or it uses old debian npm which breaks everything
// [ ] native binary
//      [ ] SmallRng might be platform dependant: https://docs.rs/rand/0.7.3/rand/rngs/index.html
// [ ] make parsing return errors instead of crashing
// [x] pause, variable speed
// [ ] frame debug mode - only render gamelogic frames, no interpolation
// [ ] shield pickups
// [ ] horn
//      [ ] sound, bind
//      [ ] make AI move out of the way
// [ ] cvar to set origin - tank in original RW turned around turret center
// [ ] easter eggs
//      [ ] server say for 0 deaths
//      [ ] bugfeatures from original RW:
//          [ ] BFG can shoot through walls touching on the corner (well, somewhat)
//              detect when doing multiple times and print a message
//                  private?
//                  server say? (lol no collision detection?)
//          [ ] BFG does a tiny bit of dmg when hitting a wall with a tank on the other side
//              probably BFG briefly enters wall before collision is detected and does proximity dmg
//          [ ] tank could shoot through a wall tile by putting the turret inside
// [ ] cleanup unused stuff from assets
// [ ] code cleanup
//      [ ] replace `as` with safer conversions (https://docs.rs/num_enum/0.5.1/num_enum/ instead of enumn?)
//      [ ] `unwrap` - they are all temporary from the prototyping phase
//              - review all of them and eliminate or replace with `except` (to mark as reviewed)
// [ ] all the LATERs - they mean something can be done better but marking it as a todo would be just noise when grepping

fn main() {
    println!("There is no native binary yet, compile to WASM instead (see readme)");
}
