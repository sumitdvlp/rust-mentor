// ===========================================================================
// Tests — these are your specification. Read them! Don't edit them.
//
// `lib.rs` declares this file with `#[cfg(test)] mod tests;`, so it is only
// compiled by `cargo test`, and `use super::*` sees everything in `lib.rs`.
// ===========================================================================

use super::*;

#[test]
fn first_word_works() {
    let s = String::from("hello world");
    let w = first_word(&s);
    assert_eq!(w, "hello");
    assert_eq!(first_word("single"), "single");
    assert_eq!(first_word(""), "");
    // `s` is still usable: we only borrowed it.
    assert_eq!(s.len(), 11);
}

#[test]
fn greeting() {
    let name = String::from("Ferris");
    assert_eq!(make_greeting(&name), "Hello, Ferris!");
    assert_eq!(make_greeting("Gopher"), "Hello, Gopher!");
    assert_eq!(name, "Ferris"); // still ours
}

#[test]
fn exclaim() {
    let mut s = String::from("Wow");
    append_exclamation(&mut s);
    append_exclamation(&mut s);
    assert_eq!(s, "Wow!!");
}

#[test]
fn round_trip() {
    let v = vec![1, 2];
    let v = take_and_return(v); // shadow: we got ownership back
    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
fn slices() {
    let v = vec![1, 2, 3, 4, 5];
    assert_eq!(sum_slice(&v), 15);
    assert_eq!(sum_slice(&v[1..3]), 5); // sub-slice [2, 3]
    assert_eq!(sum_slice(&[10, 20]), 30); // array
    assert_eq!(sum_slice(&[]), 0);
}

#[test]
fn largest_works() {
    let v = vec![3, 9, 2, 9, 1];
    assert_eq!(largest(&v), Some(&9));
    assert_eq!(largest(&[-5]), Some(&-5));
    assert_eq!(largest(&[]), None);
}

#[test]
fn doubling() {
    let mut v = vec![1, -2, 3];
    double_in_place(&mut v);
    assert_eq!(v, vec![2, -4, 6]);
    let mut arr = [5, 5, 5, 5];
    double_in_place(&mut arr[..2]); // only the first half
    assert_eq!(arr, [10, 10, 5, 5]);
}

#[test]
fn names() {
    assert_eq!(
        split_name(String::from("Grace Hopper")),
        (String::from("Grace"), String::from("Hopper"))
    );
    assert_eq!(
        split_name(String::from("Cher")),
        (String::from("Cher"), String::new())
    );
}

#[test]
fn concat() {
    let parts = vec![
        String::from("own"),
        String::from("borrow"),
        String::from("move"),
    ];
    assert_eq!(concat_all(&parts), "own borrow move");
    assert_eq!(concat_all(&[]), "");
    assert_eq!(parts.len(), 3); // still ours
}

#[test]
fn long_words_clones() {
    let words: Vec<String> = ["a", "rusty", "crab", "ferris"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(long_words(&words, 4), vec!["rusty", "ferris"]);
    assert_eq!(words.len(), 4); // original untouched
}

#[test]
fn reverse_moves() {
    let v = vec![String::from("a"), String::from("b"), String::from("c")];
    assert_eq!(reverse_owned(v), vec!["c", "b", "a"]);
    // `v` is gone now. Try uncommenting the next line and read the error:
    // println!("{:?}", v);
}
