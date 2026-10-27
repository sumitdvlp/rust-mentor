# 🔒 P4 — `kvd`: a multithreaded key-value server

> **When:** after Module 10 · **Locked** — ask your mentor to unlock it (you'll get
> a skeleton with milestones and tests, like P1).

A tiny Redis-like server over TCP using only `std`.

## Features

- Text protocol: `SET key value`, `GET key`, `DEL key`, `KEYS`
- Talk to it with `nc localhost 7878`
- A hand-written thread pool
- Graceful shutdown, optional TTLs and snapshot-to-disk

## Concepts practiced

- `TcpListener` / `TcpStream`, `BufReader`
- `Arc<RwLock<HashMap<..>>>` shared state
- Channels to feed jobs to worker threads
- Load testing and measuring contention
