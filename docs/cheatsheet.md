# Go / Java / Python → Rust cheat sheet

## Basics

| Concept | Go | Java | Python | Rust |
|---|---|---|---|---|
| Variable | `x := 1` | `var x = 1;` | `x = 1` | `let x = 1;` (immutable) |
| Mutable variable | default | default | default | `let mut x = 1;` |
| Constant | `const N = 10` | `static final int N = 10;` | `N = 10` | `const N: i32 = 10;` |
| Function | `func add(a, b int) int` | `int add(int a, int b)` | `def add(a, b):` | `fn add(a: i32, b: i32) -> i32` |
| Return | `return a + b` | `return a + b;` | `return a + b` | `a + b` (tail expression) |
| Ternary | — | `c ? a : b` | `a if c else b` | `if c { a } else { b }` |
| String formatting | `fmt.Sprintf("%d", x)` | `String.format` | `f"{x}"` | `format!("{x}")` |
| Print | `fmt.Println` | `System.out.println` | `print` | `println!` |
| Debug print | `%+v` | `toString()` | `repr()` | `{:?}` / `{:#?}` with `#[derive(Debug)]` |

## Types & data

| Concept | Go | Java | Python | Rust |
|---|---|---|---|---|
| Null / nil | `nil` | `null` | `None` | **none** — use `Option<T>` |
| Struct / class | `type P struct{}` | `class P {}` / `record` | `class P:` / `@dataclass` | `struct P {}` + `impl P {}` |
| Constructor | `NewP()` | `new P()` | `__init__` | `P::new()` (convention) |
| Enum | `iota` consts | `enum` | `Enum` | `enum` — variants can carry data |
| Tagged union | interface + type switch | sealed interface + records | `Union` + `match` | `enum` + `match` |
| Interface | implicit `interface` | explicit `interface` | duck typing / `Protocol` | explicit `trait` + `impl Trait for T` |
| Generics | `[T any]` | `<T>` (erased) | `TypeVar` (hints only) | `<T: Bound>` (monomorphized) |
| Inheritance | embedding | `extends` | `class A(B)` | **none** — traits + composition |
| Tuple | — (multiple returns) | — | `(a, b)` | `(a, b)` |

## Errors

| Concept | Go | Java | Python | Rust |
|---|---|---|---|---|
| Fallible fn | `(T, error)` | `throws E` | raises | `Result<T, E>` |
| Propagate | `if err != nil { return err }` | (implicit) | (implicit) | `?` |
| Crash | `panic()` | `throw new Error` | `raise` | `panic!()` |
| Any error | `error` | `Exception` | `Exception` | `Box<dyn Error>` / `anyhow::Error` |
| Cleanup | `defer` | `try-with-resources` / `finally` | `with` | `Drop` (automatic, RAII) |

## Collections & iteration

| Concept | Go | Java | Python | Rust |
|---|---|---|---|---|
| Growable list | `[]T` | `ArrayList<T>` | `list` | `Vec<T>` |
| Map | `map[K]V` | `HashMap<K,V>` | `dict` | `HashMap<K, V>` |
| Set | `map[T]struct{}` | `HashSet<T>` | `set` | `HashSet<T>` |
| Loop with index | `for i, x := range xs` | `for (int i...)` | `for i, x in enumerate(xs)` | `for (i, x) in xs.iter().enumerate()` |
| Map/filter | loops | Streams | comprehensions | iterator adapters `.map().filter()` |
| Lambda | `func(x int) int {...}` | `x -> x * 2` | `lambda x: x * 2` | `\|x\| x * 2` |

## Memory & concurrency

| Concept | Go | Java | Python | Rust |
|---|---|---|---|---|
| Memory management | GC | GC | refcount + GC | ownership, freed at scope end |
| Pointer / reference | `*T` | object refs | object refs | `&T`, `&mut T`, `Box<T>` |
| Shared ownership | (GC) | (GC) | (refcount) | `Rc<T>` / `Arc<T>` |
| Lightweight concurrency | goroutine | virtual thread | `asyncio` task | `tokio::spawn` (async) |
| OS thread | — | `Thread` | `threading.Thread` | `std::thread::spawn` |
| Channel | `chan T` | `BlockingQueue` | `queue.Queue` | `mpsc::channel()` |
| Mutex | `sync.Mutex` (separate from data) | `synchronized` | `threading.Lock` | `Mutex<T>` (**wraps** the data) |
| Data race | runtime race detector | possible | GIL-limited | **compile error** |

## Tooling

| Task | Go | Java | Python | Rust |
|---|---|---|---|---|
| Build tool | `go` | Maven/Gradle | pip/poetry/uv | `cargo` |
| Package | module | artifact | package | crate |
| Registry | proxy.golang.org | Maven Central | PyPI | crates.io |
| Manifest | `go.mod` | `pom.xml` | `pyproject.toml` | `Cargo.toml` |
| Test | `go test` | JUnit | pytest | `cargo test` (built in) |
| Format | `gofmt` | — | black/ruff | `cargo fmt` |
| Lint | `go vet` | SpotBugs | ruff | `cargo clippy` |
