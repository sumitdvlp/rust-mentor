// ===========================================================================
// Tests — these are your specification. Read them! Don't edit them.
//
// `lib.rs` declares this file with `#[cfg(test)] mod tests;`, so it is only
// compiled by `cargo test`, and `use super::*` sees everything in `lib.rs`.
// ===========================================================================

use super::*;

#[test]
fn celsius() {
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    assert!((celsius_to_fahrenheit(-40.0) - -40.0).abs() < f64::EPSILON);
}

#[test]
fn fizzbuzz_values() {
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(98), "98");
}

#[test]
fn squares() {
    assert_eq!(sum_of_squares(0), 0);
    assert_eq!(sum_of_squares(1), 1);
    assert_eq!(sum_of_squares(3), 14);
    assert_eq!(sum_of_squares(10), 385);
}

#[test]
fn fib() {
    let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    for (n, want) in expected.iter().enumerate() {
        assert_eq!(fibonacci(n as u32), *want, "fib({n})");
    }
    assert_eq!(fibonacci(90), 2_880_067_194_370_816_120);
}

#[test]
fn primes() {
    let primes: Vec<u64> = (0..30).filter(|&n| is_prime(n)).collect();
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    assert!(is_prime(7919));
    assert!(!is_prime(7917));
}

#[test]
fn collatz() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(2), 1);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(27), 111);
}

#[test]
fn grades() {
    assert_eq!(grade(100), 'A');
    assert_eq!(grade(90), 'A');
    assert_eq!(grade(89), 'B');
    assert_eq!(grade(75), 'C');
    assert_eq!(grade(60), 'D');
    assert_eq!(grade(0), 'F');
    assert_eq!(grade(101), '?');
    assert_eq!(grade(255), '?');
}

#[test]
fn minmax() {
    assert_eq!(min_max([3, 1, 4, 1, 5]), (1, 5));
    assert_eq!(min_max([-7, -2, -9, 0, -1]), (-9, 0));
    assert_eq!(min_max([42; 5]), (42, 42));
}

#[test]
fn vowels() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("Rust"), 1);
    assert_eq!(count_vowels("AEIOU aeiou"), 10);
    assert_eq!(count_vowels("rhythm"), 0);
}

#[test]
fn overflow() {
    assert_eq!(safe_add(100, 100), Some(200));
    assert_eq!(safe_add(200, 100), None);
    assert_eq!(safe_add(255, 0), Some(255));
}

#[test]
fn shadowing() {
    assert_eq!(parse_and_double("21"), 42);
    assert_eq!(parse_and_double("  -5 \n"), -10);
}
