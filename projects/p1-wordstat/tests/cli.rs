//! Integration tests: these run the real compiled binary, like a user would.
//! Files in `tests/` are compiled as separate crates that can only use the
//! *public* API — or, as here, the executable itself.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

/// Cargo tells integration tests where it built our binary.
fn wordstat() -> Command {
    Command::new(env!("CARGO_BIN_EXE_wordstat"))
}

fn sample() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample.txt")
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn m3_default_output_for_file() {
    let out = wordstat().arg(sample()).output().unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        stdout(&out),
        "lines: 4\nwords: 24\nchars: 148\nbytes: 148\n"
    );
}

#[test]
fn m3_reads_stdin() {
    let mut child = wordstat()
        .args(["-w", "-l"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"hi there hi\n")
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert_eq!(stdout(&out), "lines: 1\nwords: 3\n");
}

#[test]
fn m3_bad_flag_exits_2() {
    let out = wordstat().arg("-z").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("wordstat: unknown flag: -z"),
        "stderr was: {err}"
    );
    assert!(err.contains("usage:"), "stderr was: {err}");
}

#[test]
fn m3_missing_file_exits_1() {
    let out = wordstat().arg("definitely-not-here.txt").output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("wordstat: "));
}

#[test]
fn m4_top_words_on_sample() {
    let out = wordstat()
        .args(["-w", "--top", "3"])
        .arg(sample())
        .output()
        .unwrap();
    assert_eq!(
        stdout(&out),
        "words: 24\ntop 3 words:\n  is: 3\n  rust: 2\n  them: 2\n"
    );
}
