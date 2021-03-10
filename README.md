# build-trie

[![Crates](https://img.shields.io/crates/v/build-trie.svg)](https://crates.io/crates/build-trie)

A procedural macro for building a state machine / trie. Main use is for lexing multiple wide character wide tokens see [`example/src/main.rs`](example/src/main.rs).

Run example:
```
cargo +nightly run -p example
```

View example macro expansion (requires [cargo-expand](https://github.com/dtolnay/cargo-expand)):
```
cargo expand -p example
```

Design is WIP and subject to change