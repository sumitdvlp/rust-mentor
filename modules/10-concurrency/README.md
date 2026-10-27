# 🔒 Module 10 — Fearless Concurrency

> Data races become compile errors.
>
> **Locked** — finish the previous module, then ask your mentor:
> *"I finished Module XX, unlock the next one."* You'll get the lesson,
> exercises and tests in the same format as Modules 01–05.

## What you'll learn

- `std::thread::spawn`, `move` closures, `JoinHandle`, scoped threads
- Message passing with `mpsc` channels (Go: channels!)
- Shared state: `Arc<Mutex<T>>`, `RwLock`, atomics
- `Send` and `Sync` — the two marker traits that make it all safe
- Data parallelism with `rayon` (`.par_iter()`)
- Comparison: goroutines vs Java threads/virtual threads vs Python GIL vs Rust

🛠️ **Followed by project:** P4 — multithreaded key-value server (see `projects/`)
