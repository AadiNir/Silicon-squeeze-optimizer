# Learning Log: Silicon Squeeze Optimizer

## Scope
This file captures the main learning from Phase 1 through the current Phase 2 work: design choices, Rust concepts, tradeoffs, and the questions we used to reason through the implementation.

## Project Roadmap
- Phase 1: Local in-memory key-value store
- Phase 2: Persistence and error handling
- Phase 3: Networked service
- Phase 4: Distributed cluster

## Phase 1: In-Memory Store

### What We Built
- A `SqueezeStore` struct wrapping `HashMap<String, String>`
- A `Command` enum with `Set`, `Get`, and `Exit`
- A `parse_command` function that converts raw user input into typed commands
- A CLI loop that reads commands, mutates the store, and prints results

### Key Rust Learning
- `HashMap<String, String>` owns both keys and values
- `&mut self` is required for mutation in `set`
- `&self` is enough for read-only access in `get`
- `Option<&str>` is an efficient return type for reads because it borrows from the stored `String` instead of cloning

### Why Use an Enum Instead of Parsing Directly in `main`
- Raw text input and program intent are different layers
- `Command` creates a typed boundary between parsing and execution
- `main` stays cleaner because it handles intent, not string syntax
- Rust can enforce exhaustive handling with `match`
- Adding future commands becomes safer and easier

### Core Tradeoff
- Without `Command`, parsing logic leaks into `main`
- With `Command`, the compiler helps model valid states explicitly

## Phase 2: Persistence and Error Handling

### Goal
- Persist the store to disk so data survives program restarts

### First-Principles Design Questions We Worked Through
- What is the actual state that must survive restart?
- When should data be written to disk?
- Should persistence happen inside `set`, or should it be controlled by the caller?
- What type should represent file paths?
- What kind of failures are normal and what kind are exceptional?

## Persistence Design Decisions

### Where Persistence Logic Belongs
- `load_from_file` and `save_to_file` belong on `SqueezeStore`
- They are not general utilities; they are behavior attached to the store type

### Why `set` Should Not Write to Disk By Itself
- `set` has one core responsibility: mutate in-memory state
- Persistence policy is a higher-level decision
- Keeping file I/O outside `set` keeps `SqueezeStore` more reusable
- This separation makes testing and future reuse easier

### Why `load_from_file` Does Not Need `self`
- It constructs a new store from disk contents
- That makes it an associated function returning `Self`

### Why `save_to_file` Uses `&self`
- It persists the current in-memory state
- It needs to borrow the current store data, not create a new one

### Chosen Method Shapes
```rust
fn load_from_file(path: &Path) -> io::Result<Self>
fn save_to_file(&self, path: &Path) -> io::Result<()>
```

## Rust Concepts Learned in Phase 2

### `Serialize` and `Deserialize`
- These are traits provided by `serde`
- Deriving them teaches Rust how to convert a type to and from stored data
- Persistence starts by teaching the type system how to represent state externally

### `Result<T, E>`
- Rust uses `Result` to model operations that can fail
- `Ok(T)` means success
- `Err(E)` means failure
- File I/O and deserialization cannot assume success, so `Result` is the right abstraction

### Panic Style vs Result Style
- `expect(...)` panics on failure
- That is useful for quick debugging
- It is not ideal for production-style error handling
- A `match` on `Result` lets the program communicate failure honestly and continue or stop cleanly

### `&Path`
- `&Path` is an idiomatic borrowed path type for filesystem APIs
- The path is not owned application data; it is a borrowed description of a location
- Borrowing is appropriate because the function does not need to own the path

### RAII and Why We Do Not Need `close_file`
- Rust closes files automatically when file handles go out of scope
- `fs::read_to_string` and `fs::write` do not require a manual close step
- A separate `close_file` function is unnecessary for this phase

## Important Tradeoffs We Discussed

### Save on Exit vs Save on Every `set`
- Save on exit:
  - fewer disk writes
  - better short-term performance
  - unsafe if the process crashes before exit
- Save on every `set`:
  - more durable
  - simpler mental model
  - slightly more write overhead

### Current Learning Choice
- Save after every successful `set`
- This is the simplest correct durability model for Phase 2
- Optimization can come later with batching, async flushes, append-only logs, or WAL-style designs

### Missing File vs Corrupted File
- Missing file:
  - normal first-run case
  - should return an empty store
- Corrupted file:
  - persistence data exists but is invalid
  - should surface as an actual error

## Semicolon Learning

### Why Removing `;` Mattered in `get`
- Rust returns the final expression of a block if it does not end with `;`
- Adding `;` turns the expression into a statement and discards the value
- `get` must return `Option<&str>`, so the final expression must not be discarded

### Why Removing `;` Mattered in `save_to_file`
- `fs::write(...)` already returns `io::Result<()>`
- If you add `;`, you discard that result and the function body becomes `()`
- The function signature requires `io::Result<()>`, so the final expression should be the write result itself

## Serialization Boundary Learning

### Inner `HashMap` vs Whole `SqueezeStore`
- We explored both options
- Serializing only the map is possible because it contains the real key-value data
- Serializing the whole `SqueezeStore` is often more future-friendly because the domain type may grow later
- The current code moved toward serializing and deserializing the full `SqueezeStore`

### Key Lesson
- The saved representation and loaded representation must match
- If you serialize `SqueezeStore`, you must deserialize `SqueezeStore`

## Parsing Improvements

### Original Limitation
- `split_whitespace()` treated every space as a separator
- That made `set key multi word value` difficult to support cleanly

### New Direction
- Use `splitn(3, ' ')` so `set` can preserve everything after the key as the value
- This is the start of better command parsing for multi-word values

### Remaining Caveat
- Terminal input from `read_line` includes a trailing newline
- Parser input should be normalized with trimming before matching commands and keys

## Current Code State Summary
- In-memory key-value store exists
- Load from disk on startup exists
- Save to disk after `set` exists
- Save errors are handled with `match`
- Parser now aims to support multi-word `set` values
- Some polishing still remains around formatting and non-panicking startup/input handling

## Questions That Guided the Build
- What is the smallest unit of state that must survive restart?
- Should parsing return raw strings or typed intent?
- Should `set` know about files?
- Should missing files be errors or first-run behavior?
- If the process crashes before `exit`, what happens to unsaved writes?
- What does a semicolon do to a Rust expression?
- If a function promises `io::Result<Self>`, what exact value must it return on success?
- What should be serialized: the raw map or the whole domain type?
- What happens to command parsing when values contain spaces?

## What Phase 2 Has Taught So Far
- Persistence is a boundary between in-memory ownership and external representation
- Error handling is part of the design, not an afterthought
- Rust’s type system helps force honest modeling of success and failure
- Small API choices like `&Path`, `Result`, and enums make the system easier to evolve
- Durability always comes with tradeoffs against simplicity and performance

## Next Logical Steps After This Point
- Trim input before parsing
- Decide how polished `main` should be with respect to `expect(...)`
- Format and clean the code
- Add tests for persistence and parsing
- Improve quoted-string or multi-word value handling further if needed
