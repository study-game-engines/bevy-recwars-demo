use std::fmt::{self, Debug, Formatter};

use fnv::FnvHashMap;
use rand::prelude::*;
use rand_distr::Uniform;
use thunderdome::{Arena, Index};

use crate::{
    entities::{Ai, Player, Projectile, Vehicle},
    map::Vec2f,
};

/// Things that change during the game
/// and might need to be taken back during frame interpolation / reconciliation.
///
/// TODO How to do frame interpolation / server reconcilliation?
/// Ralith (hecs author) says to make all components a Vec if past positions
/// but that requires all code to be aware of interpolation.
/// What does veloren do?
#[derive(Debug, Clone)]
pub struct GameState {
    /// The RNG for all gamelogic
    pub rng: SmallRng,

    /// Inclusive range [-1.0, 1.0].
    /// Creating it once and saving it here might be faster than using gen_range according to docs.
    pub range_uniform11: Uniform<f64>,

    /// This gamelogic frame's time in seconds. Affected by d_speed and pause.
    pub game_time: f64,

    /// The previous gamelogic frame's time in seconds. Affected by d_speed and pause.
    pub game_time_prev: f64,

    /// Delta time since last gamelogic frame in seconds
    pub dt: f64,

    /// Map of projectile handles to vehicle handles.
    /// Prevents rail hitting the same vehicle twice
    /// when one segment ends inside the hitbox and the next starts inside it the next frame.
    /// This can for now only happen with railguns since all other projectiles get removed on hit.
    /// TODO This is still not perfect since one segment can hit multiple vehicles in any order
    /// and there's no guarantee the last vehicle is the one where the beam ends.
    /// LATER This is a can of worms:
    ///     1) Make sure (add test) one beam can kill the player and hit him again if he's unlucky enough to respawn in its path.
    ///     2) Remove the entry after the projectile exits the hitbox - e.g. guided missiles that can pass through several times.
    ///     3) Make sure the HashMap doesn't grow indefinitely in case we forgot to remove in some cases.
    pub rail_hits: FnvHashMap<Index, Index>,

    pub rail_beams: Vec<RailBeam>,
    pub bfg_beams: Vec<(Vec2f, Vec2f)>,
    pub explosions: Vec<Explosion>,
    pub ais: Arena<Ai>,
    pub players: Arena<Player>,
    pub vehicles: Arena<Vehicle>,
    pub projectiles: Arena<Projectile>,

    /// Inputs of players last frame.
    pub inputs_prev: InputsPrev,
}

impl GameState {
    pub fn new(rng: SmallRng) -> Self {
        Self {
            rng,
            range_uniform11: Uniform::new_inclusive(-1.0, 1.0),
            game_time: 0.0,
            game_time_prev: 0.0,
            dt: 0.0,
            rail_beams: Vec::new(),
            rail_hits: FnvHashMap::default(),
            bfg_beams: Vec::new(),
            explosions: Vec::new(),
            ais: Arena::new(),
            players: Arena::new(),
            vehicles: Arena::new(),
            projectiles: Arena::new(),
            inputs_prev: InputsPrev(FnvHashMap::default()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RailBeam {
    pub begin: Vec2f,
    pub end: Vec2f,
    pub start_time: f64,
}

impl RailBeam {
    pub fn new(begin: Vec2f, end: Vec2f, start_time: f64) -> Self {
        Self {
            begin,
            end,
            start_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Explosion {
    pub pos: Vec2f,
    pub scale: f64,
    pub start_time: f64,
    pub bfg: bool,
}

impl Explosion {
    pub fn new(pos: Vec2f, scale: f64, start_time: f64, bfg: bool) -> Self {
        Self {
            pos,
            scale,
            start_time,
            bfg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputsPrev(FnvHashMap<Index, Input>);

impl InputsPrev {
    /// The player's input last frame or empty input if the player wasn't connected last frame.
    pub fn get(&self, player_handle: Index) -> Input {
        if let Some(input) = self.0.get(&player_handle) {
            *input
        } else {
            Input::new()
        }
    }

    pub fn snapshot(&mut self, players: &Arena<Player>) {
        self.0.clear();
        for (handle, player) in players.iter() {
            self.0.insert(handle, player.input);
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub turret_left: bool,
    pub turret_right: bool,
    pub prev_weapon: bool,
    pub next_weapon: bool,
    pub fire: bool,
    pub mine: bool,
    pub self_destruct: bool,
    pub horn: bool,
    pub chat: bool,
    pub pause: bool,
    // ^ when adding fields, also add them to Debug
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_up() -> Self {
        Self {
            up: true,
            ..Self::default()
        }
    }

    pub fn merged(&self, other: Input) -> Self {
        Self {
            left: self.left | other.left,
            right: self.right | other.right,
            up: self.up | other.up,
            down: self.down | other.down,
            turret_left: self.turret_left | other.turret_left,
            turret_right: self.turret_right | other.turret_right,
            prev_weapon: self.prev_weapon | other.prev_weapon,
            next_weapon: self.next_weapon | other.next_weapon,
            fire: self.fire | other.fire,
            mine: self.mine | other.mine,
            self_destruct: self.self_destruct | other.self_destruct,
            horn: self.horn | other.horn,
            chat: self.chat | other.chat,
            pause: self.pause | other.pause,
        }
    }

    pub fn right_left(&self) -> f64 {
        self.right as i32 as f64 - self.left as i32 as f64
    }

    pub fn up(&self) -> f64 {
        self.up as i32 as f64
    }

    pub fn down(&self) -> f64 {
        self.down as i32 as f64
    }

    /// Subset of inputs to control the missile
    pub fn missile_while_guiding(&self) -> Self {
        Self {
            up: true,
            down: false,
            ..*self
        }
    }

    /// Subset of inputs to control the vehicle while guiding a missile
    pub fn vehicle_while_guiding(&self) -> Self {
        // Original RW allowed everything except movement.
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
            ..*self
        }
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Input {{ ")?;
        if self.left {
            write!(f, "left ")?;
        }
        if self.right {
            write!(f, "right ")?;
        }
        if self.up {
            write!(f, "up ")?;
        }
        if self.down {
            write!(f, "down ")?;
        }
        if self.turret_left {
            write!(f, "turret_left ")?;
        }
        if self.turret_right {
            write!(f, "turret_right ")?;
        }
        if self.prev_weapon {
            write!(f, "prev_weapon ")?;
        }
        if self.next_weapon {
            write!(f, "next_weapon ")?;
        }
        if self.fire {
            write!(f, "fire ")?;
        }
        if self.mine {
            write!(f, "mine ")?;
        }
        if self.self_destruct {
            write!(f, "self_destruct ")?;
        }
        if self.horn {
            write!(f, "horn ")?;
        }
        if self.chat {
            write!(f, "chat ")?;
        }
        if self.pause {
            write!(f, "pause ")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

pub trait ArenaExt {
    /// Collect the handles (`thunderdome::Index`) into a `Vec`.
    ///
    /// This is borrowck dance to allow iterating through the collection without keeping the arena borrowed.
    /// You can reborrow each iteration of the loop by indexing the arena using the handle
    /// and release the borrow if you need to pass the arena (or usually whole `GameState`) into another function.
    fn iter_handles(&self) -> Vec<Index>;
}

impl<T> ArenaExt for Arena<T> {
    fn iter_handles(&self) -> Vec<Index> {
        self.iter().map(|(handle, _)| handle).collect()
    }
}
