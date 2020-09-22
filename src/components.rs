use enumn::N;
use legion::Entity;

use crate::map::Vec2f;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Pos(pub(crate) Vec2f);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Vel(pub(crate) Vec2f);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Angle(pub(crate) f64);

#[derive(Debug, Clone, Copy)]
pub(crate) struct TurnRate(pub(crate) f64);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Time(pub(crate) f64);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Mg;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Cb;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Rocket;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bfg;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Owner(pub(crate) Entity);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Hitbox {
    pub(crate) mins: Vec2f,
    pub(crate) maxs: Vec2f,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, N)]
pub(crate) enum VehicleType {
    Tank,
    Hovercraft,
    Hummer,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Destroyed(pub(crate) bool);
