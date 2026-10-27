// ===========================================================================
// Tests — these are your specification. Read them! Don't edit them.
//
// `lib.rs` declares this file with `#[cfg(test)] mod tests;`, so it is only
// compiled by `cargo test`, and `use super::*` sees everything in `lib.rs`.
// ===========================================================================

use super::*;

#[test]
fn adapters() {
    assert_eq!(evens_squared(&[1, 2, 3, 4]), vec![4, 16]);
    assert_eq!(evens_squared(&[]), Vec::<i32>::new());
    assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(dot(&[1, 2, 3], &[4, 5, 6]), 32);
}

#[test]
fn frequencies() {
    let f = word_frequencies("The cat. the HAT! ... cat");
    assert_eq!(f.len(), 3);
    assert_eq!(f["the"], 2);
    assert_eq!(f["cat"], 2);
    assert_eq!(f["hat"], 1);
    assert!(word_frequencies("   ").is_empty());
}

#[test]
fn top_words() {
    let text = "b a c b a b d";
    assert_eq!(
        top_n_words(text, 3),
        vec![
            ("b".to_string(), 3),
            ("a".to_string(), 2),
            ("c".to_string(), 1)
        ]
    );
    assert_eq!(top_n_words(text, 100).len(), 4);
}

#[test]
fn grouping() {
    let g = group_by_length(&["hi", "yo", "rust", "crab", "é"]);
    assert_eq!(g[&2], vec!["hi", "yo"]);
    assert_eq!(g[&4], vec!["rust", "crab"]);
    assert_eq!(g[&1], vec!["é"]); // 1 char, even though 2 bytes!
}

#[test]
fn sets() {
    assert_eq!(unique_sorted(&[3, 1, 3, 2, 1]), vec![1, 2, 3]);
    assert!(is_anagram("Dormitory", "dirty room"));
    assert!(is_anagram("listen", "SILENT"));
    assert!(!is_anagram("rust", "trusty"));
}

#[test]
fn matrices() {
    let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(transpose(&m), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    assert_eq!(transpose(&[]), Vec::<Vec<i32>>::new());
    assert_eq!(
        positive_flat(&[vec![-1, 2], vec![], vec![3, 0]]),
        vec![2, 3]
    );
}

#[test]
fn closures() {
    assert_eq!(apply_n(|x| x * 2, 3, 1), 8);
    assert_eq!(apply_n(|x| x + 1, 0, 5), 5);

    let mut counter = make_counter();
    assert_eq!(counter(), 1);
    assert_eq!(counter(), 2);
    assert_eq!(counter(), 3);
    let mut other = make_counter(); // independent state
    assert_eq!(other(), 1);

    let add5 = make_adder(5);
    assert_eq!(add5(10), 15);
    assert_eq!(apply_n(&add5, 2, 0), 10); // &F also implements Fn
}

#[test]
fn sorting() {
    let mut people = vec![
        ("Zoe".to_string(), 30),
        ("Adam".to_string(), 25),
        ("Bob".to_string(), 30),
    ];
    sort_people(&mut people);
    let names: Vec<&str> = people.iter().map(|p| p.0.as_str()).collect();
    assert_eq!(names, vec!["Adam", "Bob", "Zoe"]);
}

#[test]
fn custom_iterators() {
    assert_eq!(Countdown::new(3).collect::<Vec<_>>(), vec![3, 2, 1]);
    assert_eq!(Countdown::new(0).count(), 0);
    assert_eq!(Countdown::new(10).sum::<u32>(), 55);
    assert_eq!(
        fib().take(8).collect::<Vec<_>>(),
        vec![0, 1, 1, 2, 3, 5, 8, 13]
    );
    let first_big = fib().find(|&n| n > 1000);
    assert_eq!(first_big, Some(1597));
}
