//! # Module 05 — Collections, Iterators & Closures
//!
//! cargo test -p m05-collections-iterators
//!
//! Style goal for this module: prefer iterator chains over index loops.
//! (Loops are not wrong! But you should be fluent in both.)

#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

use std::collections::{BTreeSet, HashMap};

// ---------------------------------------------------------------------------
// Exercise 1 — iterator adapters
// ---------------------------------------------------------------------------

/// Square only the even numbers, keep order.  [1,2,3,4] → [4,16]
/// Requirement: one chain — `.iter().filter(..).map(..).collect()`.
pub fn evens_squared(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// Running (prefix) sums: [1,2,3,4] → [1,3,6,10]
/// Hint: `.scan(0, |acc, &x| { *acc += x; Some(*acc) })`
pub fn running_sum(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// Dot product of two equal-length slices. Hint: `zip`.
pub fn dot(a: &[i64], b: &[i64]) -> i64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2 — HashMap & the entry API
// ---------------------------------------------------------------------------

/// Count words, case-insensitive. Words are separated by whitespace;
/// strip leading/trailing ASCII punctuation from each word; skip empties.
/// "The cat. the HAT!" → {"the": 2, "cat": 1, "hat": 1}
///
/// Hint: `*map.entry(word).or_insert(0) += 1;`  (no double lookup!)
/// Hint: `w.trim_matches(|c: char| c.is_ascii_punctuation())`
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// The `n` most frequent words, sorted by count DESC, then word ASC.
/// Reuse `word_frequencies`.
/// Hint: collect into a Vec<(String, usize)> then
/// `sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)))`
pub fn top_n_words(text: &str, n: usize) -> Vec<(String, usize)> {
    todo!()
}

/// Group words by their length (in chars).
/// ["hi", "yo", "rust"] → {2: ["hi", "yo"], 4: ["rust"]}, keep input order.
/// Hint: `.entry(len).or_default().push(...)`
pub fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3 — sets & sorting
// ---------------------------------------------------------------------------

/// Unique values, ascending.  [3,1,3,2,1] → [1,2,3]
/// Try `BTreeSet` (sorted set) — `.iter().copied().collect::<BTreeSet<_>>()`
pub fn unique_sorted(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// Are `a` and `b` anagrams? Ignore case and spaces.
/// "Dormitory" / "dirty room" → true
pub fn is_anagram(a: &str, b: &str) -> bool {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4 — nested collections
// ---------------------------------------------------------------------------

/// Transpose a rectangular matrix. [[1,2,3],[4,5,6]] → [[1,4],[2,5],[3,6]]
/// Empty input → empty output.
pub fn transpose(m: &[Vec<i32>]) -> Vec<Vec<i32>> {
    todo!()
}

/// Flatten and keep only positive numbers. Hint: `flat_map` or `flatten`.
pub fn positive_flat(m: &[Vec<i32>]) -> Vec<i32> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5 — closures
// ---------------------------------------------------------------------------

/// Apply `f` to `x`, `n` times. apply_n(|x| x * 2, 3, 1) == 8
/// `impl Fn(i32) -> i32` accepts any closure or fn with that shape.
pub fn apply_n(f: impl Fn(i32) -> i32, n: usize, x: i32) -> i32 {
    todo!()
}

/// Return a closure that yields 1, 2, 3, ... on each call.
/// The closure must OWN its counter → `move` keyword. It mutates its
/// captured state → it's `FnMut`, not `Fn`.
///
/// The skeleton is half-written for you because `impl Trait` in return
/// position needs a concrete closure. Replace the `todo!()` inside it.
pub fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || todo!()
}

/// Return a closure that adds `n` to its argument (captures `n`).
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| todo!()
}

/// Sort people by age ascending, then by name ascending.
/// Hint: `sort_by_key(|p| (p.1, p.0.clone()))` or `sort_by` with `then_with`.
pub fn sort_people(people: &mut [(String, u32)]) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 6 — implement your own Iterator
// ---------------------------------------------------------------------------

/// Counts down from the start value to 1, then stops.
/// Countdown::new(3) yields 3, 2, 1.
pub struct Countdown {
    remaining: u32,
}

impl Countdown {
    pub fn new(from: u32) -> Self {
        Countdown { remaining: from }
    }
}

/// Implement ONE method (`next`) and you get ~70 adapters for free:
/// map, filter, sum, zip, take, collect, ...
impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// An infinite Fibonacci iterator: 0, 1, 1, 2, 3, 5, ...
/// (Callers use `.take(n)` to make it finite.)
pub struct Fib {
    curr: u64,
    next: u64,
}

pub fn fib() -> Fib {
    Fib { curr: 0, next: 1 }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

// ===========================================================================
// Tests — your specification lives in `src/tests.rs`. Read it! Don't edit it.
// ===========================================================================
#[cfg(test)]
mod tests;
