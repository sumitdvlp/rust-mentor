# 🔒 P2 — `todo`: a persistent task manager CLI

> **When:** after Module 06 · **Locked** — ask your mentor to unlock it (you'll get
> a skeleton with milestones and tests, like P1).

A command-line todo app that saves to a JSON file.

## Features

- `todo add "buy milk" --due 2026-11-01 --tag home`
- `todo list [--done] [--tag home]`
- `todo done 3`, `todo rm 3`, `todo edit 3 "buy oat milk"`

## Concepts practiced

- Domain modeling with structs/enums (`Status`, `Priority`)
- A `Storage` **trait** with `JsonFileStorage` and `InMemoryStorage` (for tests!)
- `serde` + `serde_json` for persistence, `clap` (derive) for args
- Error handling with `thiserror` (lib) + `anyhow` (bin)
