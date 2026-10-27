// ===========================================================================
// Tests — these are your specification. Read them! Don't edit them.
//
// `lib.rs` declares this file with `#[cfg(test)] mod tests;`, so it is only
// compiled by `cargo test`, and `use super::*` sees everything in `lib.rs`.
// ===========================================================================

use super::*;

#[test]
fn ages() {
    assert_eq!(parse_age("42"), Ok(42));
    assert_eq!(parse_age("  7 "), Ok(7));
    assert_eq!(parse_age("150"), Ok(150));
    assert_eq!(parse_age("abc"), Err("not a number: abc".to_string()));
    assert_eq!(parse_age("-1"), Err("not a number: -1".to_string()));
    assert_eq!(parse_age("200"), Err("unrealistic age: 200".to_string()));
}

#[test]
fn division() {
    assert_eq!(divide(10, 3), Ok(3));
    assert_eq!(divide(-9, 3), Ok(-3));
    assert_eq!(divide(1, 0), Err(MathError::DivisionByZero));
    assert_eq!(divide(i32::MIN, -1), Err(MathError::Overflow));
}

#[test]
fn summing() {
    assert_eq!(sum_strings(&["1", "2", "3"]), Ok(6));
    assert_eq!(sum_strings(&[]), Ok(0));
    assert!(sum_strings(&["1", "two", "3"]).is_err());
}

#[test]
fn config_line() {
    assert_eq!(
        parse_config_line("port=8080"),
        Ok(("port".to_string(), 8080))
    );
    assert_eq!(
        parse_config_line(" timeout = 30 "),
        Ok(("timeout".to_string(), 30))
    );
    assert_eq!(parse_config_line("nope"), Err(ConfigError::MissingEquals));
    assert_eq!(parse_config_line(" =1"), Err(ConfigError::EmptyKey));
    assert!(matches!(
        parse_config_line("port=99999"),
        Err(ConfigError::BadNumber(_))
    ));
}

#[test]
fn config_messages() {
    assert_eq!(ConfigError::MissingEquals.to_string(), "expected key=value");
    assert_eq!(ConfigError::EmptyKey.to_string(), "key must not be empty");
    let err = parse_config_line("x=abc").unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid number: invalid digit found in string"
    );
}

#[test]
fn option_combinators() {
    assert_eq!(first_even_doubled(&[1, 3, 4, 6]), Some(8));
    assert_eq!(first_even_doubled(&[1, 3]), None);
    assert_eq!(display_name(Some("  ferris ")), "ferris");
    assert_eq!(display_name(Some("   ")), "anonymous");
    assert_eq!(display_name(None), "anonymous");
}

#[test]
fn boxed_errors() {
    let good = "# server config\nport=8080\n\ntimeout=20\n";
    assert_eq!(total_of_config(good).unwrap(), 8100);
    let bad = "port=8080\nthis line is broken\n";
    let err = total_of_config(bad).unwrap_err();
    assert_eq!(err.to_string(), "expected key=value");
}

#[test]
fn wallet() {
    let mut a = Wallet {
        balance: 100,
        frozen: false,
    };
    let mut b = Wallet::default();
    assert_eq!(a.withdraw(30), Ok(70));
    assert_eq!(
        a.withdraw(500),
        Err(BankError::InsufficientFunds {
            needed: 500,
            available: 70
        })
    );
    assert_eq!(a.transfer(&mut b, 50), Ok(()));
    assert_eq!((a.balance, b.balance), (20, 50));
    assert!(a.transfer(&mut b, 999).is_err());
    assert_eq!((a.balance, b.balance), (20, 50)); // unchanged
    b.frozen = true;
    assert_eq!(a.transfer(&mut b, 10), Err(BankError::AccountFrozen));
    assert_eq!((a.balance, b.balance), (20, 50)); // unchanged
}
