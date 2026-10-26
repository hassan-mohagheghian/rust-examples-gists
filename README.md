# rust-examples-gists

A collection of small, self-contained Rust examples.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, so Rust 1.85 or newer)

Check your version with:

```sh
rustc --version
```

## Running an example

Each file in `examples/` is a standalone program with its own `main` function.
Run one by passing its file name (without `.rs`) to `cargo run --example`:

```sh
cargo run --example struct_update_partial_move
```

Expected output:

```
user1.active: true
user1.email: alice@example.com
user2.active: true
user2.username: Alice
user2.email: new@example.com
```

## Available examples

| Example | Description |
| --- | --- |
| `struct_update_partial_move` | Struct update syntax (`..user1`), `Copy` vs. move semantics, and partial moves |

## Listing examples

To see every example in the project:

```sh
cargo run --example
```

## Adding a new example

1. Create a new `.rs` file in `examples/` with a `fn main()`.
2. Run it with `cargo run --example <file-name-without-extension>`.
