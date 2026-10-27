# 🔒 Module 06 — Traits & Generics

> Go's interfaces + Java's generics, but resolved at compile time.
>
> **Locked** — finish the previous module, then ask your mentor:
> *"I finished Module XX, unlock the next one."* You'll get the lesson,
> exercises and tests in the same format as Modules 01–05.

## What you'll learn

- Defining traits, default methods, implementing traits for your types
- Generics & trait bounds: `fn largest<T: PartialOrd>(v: &[T]) -> &T`, `where` clauses
- Static vs dynamic dispatch: `impl Trait` / generics (monomorphized) vs `dyn Trait` (vtable)
- The std trait zoo: `Display`, `Debug`, `Clone`/`Copy`, `PartialEq`/`Eq`/`Hash`, `PartialOrd`/`Ord`, `Default`, `From`/`Into`, `AsRef`, `IntoIterator`
- Operator overloading (`impl Add for Cents` — finally!)
- Associated types vs generic parameters; the orphan rule
- Comparison: Go implicit interfaces vs Java explicit interfaces vs Rust traits

🛠️ **Followed by project:** P2 — todo CLI (see `projects/`)
