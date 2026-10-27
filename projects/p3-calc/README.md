# 🔒 P3 — `calc`: an expression language interpreter

> **When:** after Modules 08–09 · **Locked** — ask your mentor to unlock it (you'll get
> a skeleton with milestones and tests, like P1).

A REPL that evaluates `let x = 3 * (2 + 4)` then `x / 2 ^ 2`.

## Features

- Tokenizer: `&str` → `Vec<Token>` (lifetimes if tokens borrow the input!)
- Recursive-descent parser with operator precedence → AST
- Tree-walking evaluator with variables and good error messages
- A REPL with history

## Concepts practiced

- `enum Expr { Num(f64), Binary(Box<Expr>, Op, Box<Expr>), Var(String), ... }`
- `Box` for recursive types, exhaustive `match` over the AST
- Lifetimes in a zero-copy lexer
- Property-style tests: `parse(print(ast)) == ast`
