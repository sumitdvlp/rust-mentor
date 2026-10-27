// ===========================================================================
// Tests — these are your specification. Read them! Don't edit them.
//
// `lib.rs` declares this file with `#[cfg(test)] mod tests;`, so it is only
// compiled by `cargo test`, and `use super::*` sees everything in `lib.rs`.
// ===========================================================================

use super::*;

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn rectangle() {
    let mut r = Rectangle::new(3, 4);
    assert_eq!(r.area(), 12);
    assert!(r.can_hold(&Rectangle::new(2, 3)));
    assert!(!r.can_hold(&Rectangle::new(3, 1)));
    r.scale(2);
    assert_eq!(
        r,
        Rectangle {
            width: 6,
            height: 8
        }
    );
    assert_eq!(Rectangle::square(5).area(), 25);
}

#[test]
fn shapes() {
    let c = Shape::Circle { radius: 1.0 };
    assert!(approx(c.area(), std::f64::consts::PI));
    assert!(approx(c.perimeter(), 2.0 * std::f64::consts::PI));
    let r = Shape::Rect { w: 2.0, h: 5.0 };
    assert!(approx(r.area(), 10.0));
    assert!(approx(r.perimeter(), 14.0));
    let t = Shape::Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    assert!(approx(t.area(), 6.0));
    assert!(approx(t.perimeter(), 12.0));
}

#[test]
fn traffic() {
    let light = TrafficLight::Red;
    let cycle: Vec<TrafficLight> = std::iter::successors(Some(light), |l| Some(l.next()))
        .take(4)
        .collect();
    assert_eq!(
        cycle,
        vec![
            TrafficLight::Red,
            TrafficLight::Green,
            TrafficLight::Yellow,
            TrafficLight::Red
        ]
    );
    let total: u32 = cycle[..3].iter().map(|l| l.duration_secs()).sum();
    assert_eq!(total, 60);
}

#[test]
fn money() {
    assert_eq!(Cents(1234).to_dollars(), "$12.34");
    assert_eq!(Cents(5).to_dollars(), "$0.05");
    assert_eq!(Cents(100).to_dollars(), "$1.00");
    assert_eq!(Cents(150).plus(Cents(75)), Cents(225));
}

#[test]
fn commands() {
    assert_eq!(parse_command("quit"), Some(Command::Quit));
    assert_eq!(
        parse_command("say hello there"),
        Some(Command::Say("hello there".to_string()))
    );
    assert_eq!(
        parse_command("move 3 -4"),
        Some(Command::Move { x: 3, y: -4 })
    );
    assert_eq!(parse_command("move 3"), None);
    assert_eq!(parse_command("move a b"), None);
    assert_eq!(parse_command("dance"), None);
    assert_eq!(parse_command(""), None);
}

#[test]
fn temps() {
    assert_eq!(describe_temp(None), "no reading");
    assert_eq!(describe_temp(Some(-3)), "freezing");
    assert_eq!(describe_temp(Some(0)), "cold");
    assert_eq!(describe_temp(Some(20)), "pleasant");
    assert_eq!(describe_temp(Some(30)), "hot");
}

#[test]
fn say_length() {
    assert_eq!(say_len(&Command::Say("hey".into())), 3);
    assert_eq!(say_len(&Command::Quit), 0);
}

#[test]
fn account() {
    let mut acc = Account::open("Ada");
    assert_eq!(acc.owner(), "Ada");
    acc.deposit(Cents(1000));
    assert!(acc.withdraw(Cents(300)));
    assert!(!acc.withdraw(Cents(5000))); // insufficient
    assert_eq!(acc.balance(), Cents(700));
    assert_eq!(
        acc.history(),
        &[
            Transaction::Deposit(Cents(1000)),
            Transaction::Withdrawal(Cents(300))
        ]
    );
}
