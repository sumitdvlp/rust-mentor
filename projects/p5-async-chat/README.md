# 🔒 P5 — `chatd`: an async chat server

> **When:** after Module 11 · **Locked** — ask your mentor to unlock it (you'll get
> a skeleton with milestones and tests, like P1).

A multi-room chat server with tokio; thousands of clients on a few threads.

## Features

- `/join room`, `/nick name`, `/list`, broadcast messages
- Per-client tasks, a broadcast channel per room
- Timeouts and clean disconnect handling

## Concepts practiced

- `tokio::net`, `tokio::spawn`, `select!`
- `broadcast` and `mpsc` channels
- Framing with `tokio_util::codec::LinesCodec`
- Compare: rewrite P4 in async — what changes?
