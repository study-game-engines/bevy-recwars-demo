//! Debugging tools for logging and visualizing what is going on.
//! Use the `dbg_*` macros since they don't need to be imported explicitly (at least in the lib)
//! and they allow a primitive version of overloading.
//! The fns and structs are only public because the macros need them.
//!
//! Note that there are 3 different framerates: server bookkeeping, server gamelogic, client rendering.
//! These debug tools are mainly meant for use in gamelogic - some get cleared each gamelogic step.
//! This might result in missing or duplicated messages when the framerates are no the same
//! (especially when the game is paused and nothing is cleared).
//! LATER For each message, save which frame type it came from, only clear it at the start of the same frame type.

#![allow(dead_code)]

use std::cell::RefCell;

use crate::map::Vec2f;

#[macro_export]
macro_rules! soft_assert {
    ($cond:expr $(,)?) => {
        soft_assert!($cond, stringify!($cond));
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            println!("soft assertion failed: {}, {}:{}:{}", format!($($arg)+), file!(), line!(), column!());
        }
    };
}

#[derive(Debug, Clone)]
pub struct WorldText {
    pub msg: String,
    pub pos: Vec2f,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub begin: Vec2f,
    pub end: Vec2f,
    /// Time left (decreases every frame)
    pub time: f64,
    pub color: &'static str,
}

#[derive(Debug, Clone)]
pub struct Cross {
    pub point: Vec2f,
    /// Time left (decreases every frame)
    pub time: f64,
    pub color: &'static str,
}

thread_local! {
    /// Lines of text to be printed onto the screen, cleared after printing.
    pub static DEBUG_TEXTS: RefCell<Vec<String>> = RefCell::new(Vec::new());
    pub static DEBUG_TEXTS_WORLD: RefCell<Vec<WorldText>> = RefCell::new(Vec::new());
    pub static DEBUG_LINES: RefCell<Vec<Line>> = RefCell::new(Vec::new());
    pub static DEBUG_CROSSES: RefCell<Vec<Cross>> = RefCell::new(Vec::new());
}

/// Print text into the console. Uses `println!(..)`-style formatting.
#[macro_export]
macro_rules! dbg_logf {
    ( $( $t:tt )* ) => {
        // Use info so it shows up by default in chromium, debug doesn't.
        macroquad::logging::info!( $( $t )* );
    };
}

/// Print variables into console formatted as `var1: value1, var2: value2`.
#[macro_export]
macro_rules! dbg_logd {
    ( $( $e:expr ),* ) => {
        let s = $crate::__format_pairs!( $( $e ),* );
        macroquad::logging::info!("{}", s);
    };
}

/// Print text onto the screen. Uses `println!(..)`-style formatting.
///
/// Useful for printing debug info each frame.
#[macro_export]
macro_rules! dbg_textf {
    ( $( $t:tt )* ) => {
        let s = format!( $( $t )* );
        $crate::debugging::DEBUG_TEXTS.with(|texts| {
            texts.borrow_mut().push(s)
        });
    };
}

/// Print variables onto the screen formatted as `var1: value1, var2: value2`.
///
/// Useful for printing debug info each frame.
#[macro_export]
macro_rules! dbg_textd {
    ( $( $e:expr ),* ) => {
        let s = $crate::__format_pairs!( $( $e ),* );
        $crate::debugging::DEBUG_TEXTS.with(|texts| {
            texts.borrow_mut().push(s)
        });
    };
}

/// Print text onto the screen at the given world coordinates.
///
/// Useful for printing debug info next to game entities each frame.
#[macro_export]
macro_rules! dbg_world_textf {
    ( $pos:expr, $( $t:tt )* ) => {
        let s = format!( $( $t )* );
        let text = $crate::debugging::WorldText {
            msg: s,
            pos: $pos,
        };
        $crate::debugging::DEBUG_TEXTS_WORLD.with(|texts| {
            texts.borrow_mut().push(text)
        });
    };
}

/// Print variables onto the screen at the given world coordinates formatted as `var1: value1, var2: value2`.
///
/// Useful for printing debug info next to game entities each frame.
#[macro_export]
macro_rules! dbg_world_textd {
    ( $pos:expr, $( $e:expr ),* ) => {
        let s = $crate::__format_pairs!( $( $e ),* );
        let text = $crate::debugging::WorldText {
            msg: s,
            pos: $pos,
        };
        $crate::debugging::DEBUG_TEXTS_WORLD.with(|texts| {
            texts.borrow_mut().push(text)
        });
    };
}

/// Private helper to print the name and value of each given variable.
/// Not meant to be used directly.
#[macro_export]
macro_rules! __format_pairs {
    ( $e:expr ) => {
        format!("{}: {:.6?}", stringify!($e), $e)
    };
    ( $e:expr, $( $rest:expr ),+ ) => {
        format!(
            "{}, {}",
            $crate::__format_pairs!($e),
            $crate::__format_pairs!( $( $rest ),+ )
        )
    };
}

/// Draw a line between world coordinates.
/// Optionally specify
/// - how long it lasts in seconds (default 1 frame)
/// - color
#[macro_export]
macro_rules! dbg_line {
    ($begin:expr, $end:expr, $time:expr, $color:expr) => {
        $crate::debugging::debug_line($begin, $end, $time, $color);
    };
    ($begin:expr, $end:expr, $time:expr) => {
        $crate::dbg_line!($begin, $end, $time, "red");
    };
    ($begin:expr, $end:expr) => {
        $crate::dbg_line!($begin, $end, 0.0);
    };
}

/// Helper function, prefer `dbg_line!()` instead.
pub fn debug_line(begin: Vec2f, end: Vec2f, time: f64, color: &'static str) {
    DEBUG_LINES.with(|lines| {
        let line = Line {
            begin,
            end,
            time,
            color,
        };
        lines.borrow_mut().push(line);
    });
}

/// Draw a small cross at world coordinates.
/// Optionally specify
/// - how long it lasts in seconds (default is 0.0 which means 1 frame)
/// - color
#[macro_export]
macro_rules! dbg_cross {
    ($point:expr, $time:expr, $color:expr) => {
        $crate::debugging::debug_cross($point, $time, $color);
    };
    ($point:expr, $time:expr) => {
        $crate::dbg_cross!($point, $time, "red");
    };
    ($point:expr) => {
        $crate::dbg_cross!($point, 0.0);
    };
}

/// Helper function, prefer `dbg_cross!()` instead.
pub fn debug_cross(point: Vec2f, time: f64, color: &'static str) {
    DEBUG_CROSSES.with(|crosses| {
        let cross = Cross { point, time, color };
        crosses.borrow_mut().push(cross);
    });
}

/// Count how many times in iterator returned `Some`
/// and print it when it's done.
///
/// # Examples
/// ```ignore
/// for x in [1, 2, 3].iter().dbg_count("element count") { /* loop body */ }
/// ```
pub trait DbgCount<T>
where
    T: Iterator,
{
    fn dbg_count(self, name: &'static str) -> DbgCounter<T>;
}

impl<T> DbgCount<T> for T
where
    T: Iterator,
{
    fn dbg_count(self, name: &'static str) -> DbgCounter<T> {
        DbgCounter {
            name,
            iterator: self,
            cnt: 0,
        }
    }
}

#[derive(Debug)]
pub struct DbgCounter<T>
where
    T: Iterator,
{
    name: &'static str,
    iterator: T,
    cnt: usize,
}

impl<T> Iterator for DbgCounter<T>
where
    T: Iterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(item) => {
                self.cnt += 1;
                Some(item)
            }
            None => {
                dbg_textf!("{}: {}", self.name, self.cnt);
                None
            }
        }
    }
}

pub fn cleanup() {
    DEBUG_LINES.with(|lines| lines.borrow_mut().retain(|line| line.time > 0.0));
    DEBUG_CROSSES.with(|crosses| crosses.borrow_mut().retain(|cross| cross.time > 0.0));
    DEBUG_TEXTS.with(|texts| texts.borrow_mut().clear());
    DEBUG_TEXTS_WORLD.with(|texts| texts.borrow_mut().clear());
}
