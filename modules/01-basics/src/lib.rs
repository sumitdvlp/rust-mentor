//! # Module 01 — Basics
//!
//! Replace every `todo!()` with a real implementation.
//! Run the tests:        cargo test -p m01-basics
//! Run just one test:    cargo test -p m01-basics fizzbuzz
//!
//! Read `README.md` in this folder first. Rules of the game:
//!   * Make the tests pass WITHOUT editing the tests.
//!   * Then run `cargo clippy -p m01-basics` and fix what it suggests.
//!   * Then ask your mentor for a review.

// Silences "unused variable" noise while functions are still `todo!()`.
// Delete this line once you've finished the module.
#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

// ---------------------------------------------------------------------------
// Exercise 1 — functions & expressions
// ---------------------------------------------------------------------------

// Your version:
// use std::{iter::Sum, num::ParseIntError};
//
// Why changed: neither import is used any more (`Sum` never was; the
// `#![allow(unused_imports)]` above was hiding that warning).

/// Convert Celsius to Fahrenheit: F = C × 9/5 + 32.
///
/// Challenge: write it as a single expression, no `return` keyword.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    // Your version:
    // c * f64::from(9.0 / 5.0) + 32.0
    //
    // Why changed: `9.0 / 5.0` is already an f64, so `f64::from` converted
    // f64 → f64, which does nothing (clippy: useless_conversion).
    c * (9.0 / 5.0) + 32.0
}

// ---------------------------------------------------------------------------
// Exercise 2 — `if` is an expression
// ---------------------------------------------------------------------------

/// Return "Fizz" for multiples of 3, "Buzz" for multiples of 5,
/// "FizzBuzz" for both, otherwise the number itself as a String.
///
/// Hint: `n.to_string()` and `String::from("Fizz")`.
pub fn fizzbuzz(n: u32) -> String {
    // Your version:
    // let label = if n % 15 == 0 {
    //     String::from("FizzBuzz")
    // } else if n % 5 == 0 {
    //     String::from("Buzz")
    // } else if n % 3 == 0 {
    //     String::from("Fizz")
    // } else {
    //     n.to_string()
    // };
    // label
    //
    // Why changed: the `if` is already an expression, so it can be the
    // function's value directly, with no `label` binding (clippy:
    // let_and_return). `is_multiple_of` names what `n % 15 == 0` means
    // (clippy: manual_is_multiple_of).
    if n.is_multiple_of(15) {
        String::from("FizzBuzz")
    } else if n.is_multiple_of(5) {
        String::from("Buzz")
    } else if n.is_multiple_of(3) {
        String::from("Fizz")
    } else {
        n.to_string()
    }
}

// ---------------------------------------------------------------------------
// Exercise 3 — `for` loops and ranges
// ---------------------------------------------------------------------------

/// Sum of squares 1² + 2² + ... + n².  Return 0 for n = 0.
///
/// Try it twice: once with a `for` loop and a `let mut` accumulator,
/// then with an iterator: `(1..=n).map(...).sum()`.
pub fn sum_of_squares(n: u64) -> u64 {
    // Your version (the `for` loop half of the exercise):
    // let mut sum = 0;
    // for i in 1..n + 1 {
    //     sum = sum + i * i;
    // }
    // sum
    //
    // Why changed: `1..=n` is the inclusive range, so there's no `n + 1`
    // (which would overflow at n = u64::MAX). The iterator chain is the
    // second half of the exercise and needs no `mut` accumulator.
    (1..=n).map(|i| i * i).sum()
}

// ---------------------------------------------------------------------------
// Exercise 4 — mutability & tuples
// ---------------------------------------------------------------------------

/// The n-th Fibonacci number: fib(0)=0, fib(1)=1, fib(n)=fib(n-1)+fib(n-2).
///
/// Do it iteratively (no recursion). Tuple destructuring is your friend:
/// `(a, b) = (b, a + b);`
pub fn fibonacci(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    // let mut tmp = 0;
    // Your version: for i in 1..n + 1 {
    // Why changed: `i` is never used, and `_` says so (the allow at the top
    // was hiding that warning). `0..n` still runs n times, without the `+ 1`.
    for _ in 0..n {
        // tmp = a+b; // this would have been right too
        // a = b;
        // b = tmp;
        (a, b) = (b, a + b);
    }
    a
}

// ---------------------------------------------------------------------------
// Exercise 5 — `while` / early return
// ---------------------------------------------------------------------------

/// Is `n` prime? (0 and 1 are not.)
///
/// Only test divisors up to √n: loop while `i * i <= n`.
pub fn is_prime(n: u64) -> bool {
    // Your version:
    // if n < 2 {
    //     return false;
    // }
    // let mut i = 2;
    // let mut sq = 4;
    // while sq <= n {
    //     if n % i == 0 {
    //         return false;
    //     }
    //     i += 1;
    //     sq = i * i;
    // }
    // true
    //
    // Why changed: `sq` only repeats `i * i`, so that can go straight into
    // the condition. Also, once 2 is ruled out no even number can divide n,
    // so start at 3 and step by 2: half as many loop iterations.
    if n < 2 {
        return false;
    }
    if n.is_multiple_of(2) {
        return n == 2;
    }
    let mut i = 3;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 2;
    }
    true
}

// ---------------------------------------------------------------------------
// Exercise 6 — `loop` can return a value with `break value`
// ---------------------------------------------------------------------------

/// Collatz: if n is even → n/2, if odd → 3n+1. Count steps until n == 1.
/// collatz_steps(1) == 0, collatz_steps(6) == 8.
///
/// Requirement: use `loop` and `break steps;` (a loop is an expression!).
pub fn collatz_steps(n: u64) -> u32 {
    // Your version:
    // let mut cntr = 0;
    // let mut pro = n;
    // while pro >= 2 {
    //     if pro % 2 == 0 {
    //         pro = pro / 2
    //     } else {
    //         pro = 3 * pro + 1
    //     }
    //     cntr += 1;
    // }
    // cntr
    //
    // Why changed: the exercise asks for `loop` + `break steps;`, which makes
    // the loop itself the function's return value. `let mut n = n` shadows
    // the parameter, so `pro` isn't needed, and an `if` expression replaces
    // the two assignments. Your "stop at <= 1" check is kept on purpose:
    // with `== 1`, n = 0 would loop forever (0 / 2 == 0).
    let mut n = n;
    let mut steps = 0;
    loop {
        if n <= 1 {
            break steps;
        }
        n = if n.is_multiple_of(2) {
            n / 2
        } else {
            3 * n + 1
        };
        steps += 1;
    }
}

// ---------------------------------------------------------------------------
// Exercise 7 — `match` with ranges
// ---------------------------------------------------------------------------

/// Map a score to a letter grade:
/// 90..=100 → 'A', 80..=89 → 'B', 70..=79 → 'C', 60..=69 → 'D',
/// 0..=59 → 'F', anything above 100 → '?'.
///
/// Notice: the compiler forces you to handle *every* possible u8.
pub fn grade(score: u8) -> char {
    // Your version:
    // let size = match score {
    //     90..=100 => 'A',
    //     80..=89 => 'B',
    //     70..=79 => 'C',
    //     60..=69 => 'D',
    //     0..=59 => 'F',
    //     _ => '?',
    // };
    // size
    //
    // Why changed: same as fizzbuzz. `match` is an expression, so it can be
    // the function's value directly (clippy: let_and_return).
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    }
}

// ---------------------------------------------------------------------------
// Exercise 8 — arrays & returning tuples
// ---------------------------------------------------------------------------

/// Return (min, max) of a fixed-size array.
///
/// Arrays have a size known at compile time: `[i32; 5]` is a *different
/// type* from `[i32; 6]`. (Go has the same idea; Java/Python don't.)
pub fn min_max(nums: [i32; 5]) -> (i32, i32) {
    // Your version:
    // let mut mn = 100000;
    // let mut mx = -10000;
    // for num in nums {
    //     if num < mn {
    //         mn = num;
    //     }
    //     if num > mx {
    //         mx = num;
    //     }
    // }
    // (mn, mx)
    //
    // Why changed (bug): the made-up starting values are wrong for numbers
    // outside them: min_max([200_000; 5]) returned (100000, 200000).
    // Starting from a real element fixes that. `nums[0]` can't panic: the
    // type guarantees 5 elements. `mn.min(num)` does what your `if` did.
    let mut mn = nums[0];
    let mut mx = nums[0];
    for num in nums {
        mn = mn.min(num);
        mx = mx.max(num);
    }
    (mn, mx)
}

// ---------------------------------------------------------------------------
// Exercise 9 — strings & chars (sneak peek at `&str`)
// ---------------------------------------------------------------------------

/// Count vowels (a, e, i, o, u — case-insensitive).
///
/// Hint: `s.chars()` gives you an iterator of `char`.
/// `c.to_ascii_lowercase()` and `matches!(c, 'a' | 'e' | ...)` are handy.
pub fn count_vowels(s: &str) -> usize {
    // Your version:
    // let mut cnt = 0;
    // for i in s.chars() {
    //     if matches!(i.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
    //         cnt += 1;
    //     }
    // }
    // cnt
    //
    // Why changed: "count the items that match" is exactly `filter` +
    // `count`, so the counter variable goes away. (`i` also reads like an
    // index; `c` is the usual name for a char.)
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

// ---------------------------------------------------------------------------
// Exercise 10 — integer overflow is NOT silent
// ---------------------------------------------------------------------------

/// Add two u8 values. Return `Some(sum)` if it fits in a u8, else `None`.
///
/// In debug builds `a + b` would *panic* on overflow (unlike Go/Java which
/// wrap silently). Look up `u8::checked_add` in the docs.
pub fn safe_add(a: u8, b: u8) -> Option<u8> {
    // Your version:
    // u8::checked_add(a, b)
    //
    // Why changed: it's the same call, but Rust code normally uses method
    // syntax, `a.checked_add(b)`.
    a.checked_add(b)
}

// ---------------------------------------------------------------------------
// Exercise 11 — shadowing
// ---------------------------------------------------------------------------

/// Parse a string like " 21 " into a number and double it.
///
/// Requirement: use shadowing — re-declare `input` with `let` as you
/// transform it (trim → parse → double). Use `.unwrap()` for now;
/// Module 04 teaches proper error handling.
pub fn parse_and_double(input: &str) -> i64 {
    // Your version:
    // let res  = str::trim(input);
    // let n: Result<i64,ParseIntError>    = str::parse(res);
    // n.unwrap()*2
    //
    // Why changed: the exercise asks for shadowing, which means reusing the
    // name `input` at each step even as its type changes (&str → i64).
    // `let input: i64` tells `parse` which type to produce, so the
    // `Result<..>` annotation and the `ParseIntError` import aren't needed.
    let input = input.trim();
    let input: i64 = input.parse().unwrap();
    input * 2
}

// ===========================================================================
// Tests — your specification lives in `src/tests.rs`. Read it! Don't edit it.
// ===========================================================================
#[cfg(test)]
mod tests;
