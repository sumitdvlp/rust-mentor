# 🔒 Module 07 — Lifetimes

> Making the borrow checker's reasoning explicit.
>
> **Locked** — finish the previous module, then ask your mentor:
> *"I finished Module XX, unlock the next one."* You'll get the lesson,
> exercises and tests in the same format as Modules 01–05.

## What you'll learn

- What a lifetime *is* (a region of code where a borrow is valid) and what it isn't
- The three elision rules — why you rarely write `'a`
- Functions returning references: `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str`
- Structs that hold references (`struct Parser<'a> { input: &'a str }`)
- `'static`: string literals vs `T: 'static` bounds
- Reading lifetime errors like a pro; when to just use owned data instead
