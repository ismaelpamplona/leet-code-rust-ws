# leet-code-rust-ws

Leet Code Rust Workspace

## How do add a new lib/question?

1. Add the project name to the `Cargo.toml`of the project root path. The file should be like this:

```Rust
[workspace]

members = [
    "lib_a",
    "lib_b",
    "lib_c",
]

```

2. Run `cargo new` command:  

```console
$ cargo new <lib_name> --lib 
```

## How do test your code?

```console
$ cargo test -p <lib_name> -- --nocapture
```

- `--` is used as a separator to distinguish between the arguments 
- `--nocapture` indicates that the output of the test cases should not be captured, and thus, should be displayed regardless of whether the test passes or fails.

## How to how Clippy linting tool?

```console
$ cargo clippy -p <lib_name>
```


