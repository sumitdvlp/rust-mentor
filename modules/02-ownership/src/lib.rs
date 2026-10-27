//! # Module 02 — Ownership & Borrowing
//!
//! cargo test -p m02-ownership
//!
//! The signatures here are deliberate — notice which functions take
//! `String` (ownership), `&str` / `&[T]` (shared borrow) or `&mut T`
//! (exclusive borrow). Before implementing each one, ask yourself:
//! "after this call, can the caller still use its value?"

#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

// ---------------------------------------------------------------------------
// Exercise 1 — returning a borrowed slice
// ---------------------------------------------------------------------------

/// Return the first word of `s` (everything before the first space).
/// If there is no space, return the whole string.
///
/// The result *borrows* from the input — no allocation, no copy.
/// Hint: `s.split(' ').next()` or `s.find(' ')` + slicing `&s[..i]`.
pub fn first_word(s: &str) -> &str {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2 — borrow in, owned out
// ---------------------------------------------------------------------------

/// Build "Hello, <name>!" — a brand new String owned by the caller.
///
/// Hint: `format!` works like Go's `fmt.Sprintf` / Python f-strings.
pub fn make_greeting(name: &str) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3 — mutate through an exclusive borrow
// ---------------------------------------------------------------------------

/// Append "!" to the caller's String *in place*. Returns nothing.
///
/// (Until you implement it, clippy suggests `&mut str`. Ignore that:
/// a `&mut str` can't grow — only a `&mut String` can.)
pub fn append_exclamation(s: &mut String) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4 — take ownership, give it back
// ---------------------------------------------------------------------------

/// Take ownership of `v`, push 42 onto it, and hand ownership back.
///
/// Notice `v` isn't declared `mut` in the signature. You can write
/// `mut v: Vec<i32>` — because you *own* it, you decide.
pub fn take_and_return(v: Vec<i32>) -> Vec<i32> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5 — slices: borrow a view of contiguous data
// ---------------------------------------------------------------------------

/// Sum the numbers. `&[i32]` accepts a borrowed Vec, an array, or a sub-slice.
pub fn sum_slice(nums: &[i32]) -> i32 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 6 — returning a reference *into* the input
// ---------------------------------------------------------------------------

/// Return a reference to the largest element, or `None` if empty.
///
/// Why can we return `&i32` here without lifetime annotations?
/// (Answer: "lifetime elision" — one input reference, so the output must
/// borrow from it. Module 07 goes deep on this.)
pub fn largest(nums: &[i32]) -> Option<&i32> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 7 — mutable slice
// ---------------------------------------------------------------------------

/// Double every element in place.
/// Hint: `for n in nums.iter_mut() { *n *= 2; }` — note the `*` dereference.
pub fn double_in_place(nums: &mut [i32]) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 8 — consuming a String, producing owned pieces
// ---------------------------------------------------------------------------

/// Split "First Last" into ("First", "Last") as two *owned* Strings.
/// If there's no space, return (full, "").
///
/// The input is moved in — the caller can't use it afterward.
/// Hint: `full.split_once(' ')` returns `Option<(&str, &str)>`;
///       convert with `.to_string()`.
pub fn split_name(full: String) -> (String, String) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 9 — borrow a slice of owned things, build something new
// ---------------------------------------------------------------------------

/// Join all parts with a single space: ["a","b","c"] → "a b c".
/// Do it manually with `String::new()` and `push_str` (no `.join()`!),
/// so you feel how `&String` derefs to `&str`.
pub fn concat_all(parts: &[String]) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 10 — when you *need* a copy: Clone
// ---------------------------------------------------------------------------

/// Return a new Vec containing every string from `words` that is longer
/// than `min_len`. The caller keeps its original `words` untouched.
///
/// You only have `&String`s but must return owned `String`s → `.clone()`.
/// Cloning is explicit in Rust: you always *see* the cost.
pub fn long_words(words: &[String], min_len: usize) -> Vec<String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 11 — moving out of a Vec
// ---------------------------------------------------------------------------

/// Consume `v` and return its elements in reverse order, *without cloning*.
///
/// Because you own `v`, you can move elements out of it.
/// Hint: `v.into_iter()` yields owned `String`s; or `v.pop()` in a loop.
pub fn reverse_owned(v: Vec<String>) -> Vec<String> {
    todo!()
}

// ===========================================================================
// Tests — your specification lives in `src/tests.rs`. Read it! Don't edit it.
// ===========================================================================
#[cfg(test)]
mod tests;
