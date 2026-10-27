# 🔒 Module 11 — Async Rust with Tokio

> Concurrency for I/O-bound work at scale.
>
> **Locked** — finish the previous module, then ask your mentor:
> *"I finished Module XX, unlock the next one."* You'll get the lesson,
> exercises and tests in the same format as Modules 01–05.

## What you'll learn

- Futures, `async`/`await`, and why Rust futures are lazy
- Runtimes: why Rust has no built-in one; `tokio` basics
- Tasks (`tokio::spawn`) vs goroutines; `JoinSet`
- `tokio::select!`, timeouts, cancellation
- Async channels (`mpsc`, `broadcast`, `oneshot`) and `tokio::sync::Mutex`
- Pitfalls: blocking in async, `Send` bounds across `.await`

🛠️ **Followed by project:** P5 — async chat server (see `projects/`)
