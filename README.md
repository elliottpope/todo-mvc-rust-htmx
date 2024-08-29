# TodoMVC in HTMX with Rust

> An implementation of the [TodoMVC specification](https://todomvc.com/) using HTMX and Rust

## Tooling

- [HTMX](https://htmx.org/) for frontend interactivity
- [Rust](https://www.rust-lang.org/) for backend implementation
- [Askama](https://djc.github.io/askama/askama.html) for Rust HTML templating
- [Axum](https://github.com/tokio-rs/axum) for the Rust web server
- [Tokio](https://github.com/tokio-rs/tokio) for the Rust async runtime (required by Axum)

## Running

```
cargo run
```

or if you use [`cargo watch`](https://github.com/watchexec/cargo-watch)

```
cargo watch -w src/ -w templates/ -w Cargo.toml -x run
```
