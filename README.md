# Rust Base Template

Rust Programming Base Template for new Rust Application code. 

### Macros

`std::env`

```rust
let path: &'static str = env!("PATH");
println!("the $PATH variable at the time of compiling was: {path}");
```

`std::stringify`

```rust
let one_plus_one = stringify!(1 + 1);
assert_eq!(one_plus_one, "1 + 1");
```


### Crates

reqwest
tokio
axum
uuid
colored
clap
color_eyre::eyre::Result
tracing::info
rayon
aws-sdk-rust
clap
sqlx
chrono
egui

### Commands
```sh
cargo clippy --fix -- \
    -A clippy::restriction \
    -W clippy::correctness \
    -W clippy::suspicious \
    -W clippy::complexity \
    -W clippy::perf \
    -W clippy::style \
    -W clippy::pedantic \
    -W clippy::nursery \
    -W clippy::cargo \
    -W clippy::unwrap_used \
    -W clippy::expect_used
```



### Tools

- [bacon](https://dystroy.org/bacon/)
- cargo-generate