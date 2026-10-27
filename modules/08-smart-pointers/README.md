# 🔒 Module 08 — Smart Pointers & Interior Mutability

> Heap allocation, shared ownership, and mutation behind `&`.
>
> **Locked** — finish the previous module, then ask your mentor:
> *"I finished Module XX, unlock the next one."* You'll get the lesson,
> exercises and tests in the same format as Modules 01–05.

## What you'll learn

- `Box<T>`: heap allocation and recursive types (`enum Expr { Num(f64), Add(Box<Expr>, Box<Expr>) }`)
- `Deref` and `Drop` — RAII, Rust's `defer`/`try-with-resources`
- `Rc<T>` + `Weak<T>`: shared ownership, reference cycles
- `Cell<T>` / `RefCell<T>`: borrow rules checked at runtime
- `Rc<RefCell<T>>` — and why it's a code smell when overused
- Build: a linked list and a tree with parent pointers
