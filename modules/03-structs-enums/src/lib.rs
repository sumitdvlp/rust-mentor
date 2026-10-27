//! # Module 03 — Structs, Enums & Pattern Matching
//!
//! cargo test -p m03-structs-enums
//!
//! Struct and enum *definitions* are given. You implement the methods.

#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

// ---------------------------------------------------------------------------
// Exercise 1 — a struct with methods
// ---------------------------------------------------------------------------

/// `#[derive(...)]` auto-generates trait implementations:
/// Debug → `{:?}` printing, Clone → `.clone()`, PartialEq → `==`.
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Associated function (no `self`) — Rust's "constructor" convention.
    /// Called as `Rectangle::new(3, 4)`, like a static method in Java.
    pub fn new(width: u32, height: u32) -> Self {
        todo!()
    }

    /// Another associated function: a square of side `size`.
    pub fn square(size: u32) -> Self {
        todo!()
    }

    /// `&self` = borrow the instance immutably (like Go's value receiver,
    /// but without copying).
    pub fn area(&self) -> u32 {
        todo!()
    }

    /// Can `self` fully contain `other` (strictly larger in both dims)?
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    /// `&mut self` = modify in place (like Go's pointer receiver).
    pub fn scale(&mut self, factor: u32) {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Exercise 2 — enums carry data (algebraic data types)
// ---------------------------------------------------------------------------

/// Each variant can hold different data. Compare with a Java sealed
/// interface + records, or a Go interface with 3 struct implementations —
/// in Rust it's one type, and `match` is checked for exhaustiveness.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    /// Triangle area: Heron's formula. s = (a+b+c)/2, area = √(s(s-a)(s-b)(s-c))
    /// Use `std::f64::consts::PI` for circles.
    pub fn area(&self) -> f64 {
        todo!()
    }

    pub fn perimeter(&self) -> f64 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Exercise 3 — a state machine with `self` (consuming)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    /// Red → Green → Yellow → Red. Takes `self` by value (it's `Copy`).
    pub fn next(self) -> Self {
        todo!()
    }

    /// Red 30s, Green 25s, Yellow 5s.
    pub fn duration_secs(&self) -> u32 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Exercise 4 — newtype pattern (tuple struct)
// ---------------------------------------------------------------------------

/// A tuple struct wrapping u64. Zero-cost, but the type system now stops
/// you from accidentally adding Cents to, say, a user ID.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cents(pub u64);

impl Cents {
    /// Format as dollars: Cents(1234) → "$12.34", Cents(5) → "$0.05".
    /// Hint: `self.0` accesses the inner value; `{:02}` zero-pads.
    pub fn to_dollars(&self) -> String {
        todo!()
    }

    /// (Module 06 will let you write `a + b` instead, via `impl Add`.)
    pub fn plus(self, other: Cents) -> Cents {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Exercise 5 — parsing into an enum + Option
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Quit,
}

/// Parse a text command:
///   "quit"            → Some(Command::Quit)
///   "say hello there" → Some(Command::Say("hello there"))
///   "move 3 -4"       → Some(Command::Move { x: 3, y: -4 })
///   anything else / bad numbers → None
///
/// Hint: `input.split_once(' ')`, then `match` on the tuple/slice.
/// `"3".parse::<i32>().ok()` turns a Result into an Option.
/// The `?` operator also works on Option inside a fn returning Option!
pub fn parse_command(input: &str) -> Option<Command> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 6 — match guards, `if let`, `let ... else`
// ---------------------------------------------------------------------------

/// Describe an optional temperature:
///   None            → "no reading"
///   Some(t) t < 0   → "freezing"
///   Some(0..=15)    → "cold"
///   Some(16..=25)   → "pleasant"
///   Some(_)         → "hot"
pub fn describe_temp(reading: Option<i32>) -> String {
    todo!()
}

/// Return the length of the `Say` message, or 0 for any other command.
/// Requirement: use `if let` (or `let ... else`), not a full `match`.
pub fn say_len(cmd: &Command) -> usize {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 7 — putting it together: a bank account
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Account {
    owner: String, // private! only this module can touch fields
    balance: Cents,
    history: Vec<Transaction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Transaction {
    Deposit(Cents),
    Withdrawal(Cents),
}

impl Account {
    pub fn open(owner: &str) -> Self {
        todo!()
    }

    pub fn owner(&self) -> &str {
        todo!()
    }

    pub fn balance(&self) -> Cents {
        todo!()
    }

    pub fn deposit(&mut self, amount: Cents) {
        todo!()
    }

    /// Withdraw if funds suffice → true. Otherwise change nothing → false.
    /// (Module 04 will upgrade this to return a proper `Result`.)
    pub fn withdraw(&mut self, amount: Cents) -> bool {
        todo!()
    }

    /// Borrow the history as a slice — callers can read but not modify.
    pub fn history(&self) -> &[Transaction] {
        todo!()
    }
}

// ===========================================================================
// Tests — your specification lives in `src/tests.rs`. Read it! Don't edit it.
// ===========================================================================
#[cfg(test)]
mod tests;
