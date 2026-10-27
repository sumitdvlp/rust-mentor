//! Progress dashboard: runs each module's tests and shows a bar.
//!
//!   cargo run -q -p progress
//!
//! (It's also a tiny real-world Rust program — read it after Module 05!)
//! When a new module is unlocked, add it to `TRACKS`.

use std::process::Command;

const TRACKS: &[(&str, &str)] = &[
    ("m01-basics", "01 Basics"),
    ("m02-ownership", "02 Ownership & Borrowing"),
    ("m03-structs-enums", "03 Structs & Enums"),
    ("m04-error-handling", "04 Error Handling"),
    ("m05-collections-iterators", "05 Collections & Iterators"),
    ("wordstat", "P1 wordstat"),
];

#[derive(Default)]
struct Tally {
    passed: u32,
    failed: u32,
}

/// Sum "N passed; M failed" across every `test result:` line cargo prints.
fn parse(stdout: &str) -> Option<Tally> {
    let mut tally = Tally::default();
    let mut seen = false;
    for line in stdout.lines().filter(|l| l.contains("test result:")) {
        seen = true;
        let words: Vec<&str> = line.split_whitespace().collect();
        for pair in words.windows(2) {
            let Ok(n) = pair[0].parse::<u32>() else {
                continue;
            };
            if pair[1].starts_with("passed") {
                tally.passed += n;
            } else if pair[1].starts_with("failed") {
                tally.failed += n;
            }
        }
    }
    seen.then_some(tally)
}

fn bar(done: u32, total: u32, width: u32) -> String {
    let filled = if total == 0 { 0 } else { done * width / total };
    format!(
        "{}{}",
        "█".repeat(filled as usize),
        "░".repeat((width - filled) as usize)
    )
}

fn main() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    println!("\n  🦀 Rust Mentor — progress\n");
    let (mut all_pass, mut all_total) = (0, 0);

    for (pkg, label) in TRACKS {
        let output = Command::new(&cargo)
            .args(["test", "-q", "--no-fail-fast", "-p", pkg])
            .output()
            .expect("failed to run cargo");
        let stdout = String::from_utf8_lossy(&output.stdout);

        match parse(&stdout) {
            Some(t) => {
                let total = t.passed + t.failed;
                all_pass += t.passed;
                all_total += total;
                let mark = if t.failed == 0 && total > 0 {
                    "✅"
                } else {
                    "  "
                };
                println!(
                    "  {mark} {label:<28} {} {:>3}/{:<3}",
                    bar(t.passed, total, 20),
                    t.passed,
                    total
                );
            }
            None => println!("  ❌ {label:<28} does not compile — run `cargo test -p {pkg}`"),
        }
    }
    println!("\n  Total: {all_pass}/{all_total} tests passing\n");
}
