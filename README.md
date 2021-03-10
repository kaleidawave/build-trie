# build-trie

[![Crates](https://img.shields.io/crates/v/build-trie.svg)](https://crates.io/crates/build-trie)

A procedural macro for building a state machine / trie. Main use is for lexing multiple character wide tokens see [`example/src/main.rs`](example/src/main.rs).

Run example:
```
cargo run -p example
```

View example macro expansion (requires [cargo-expand](https://github.com/dtolnay/cargo-expand)):
```
cargo expand -p example
```

Probably over engineered ©

Design is WIP and subject to change.

### Example

Given the following example:

- Define the name of a function (in this case `get_symbol_from_state_and_char`) which we will use. 
- The function takes two arguments. A reference to the previous state (if no state yet use *state_enum*::None) and a character.
- It will return a *result_enum*. Which has two forms, either *result_enum*`::Result(result, character_consumed)` with a result that matched and whether the character was used in constructing the result (if not rerun lex loop on character). or *result_enum*`::NewState` indicating a new state (which should be assigned somewhere).
- `result: ...` indicates the return type of this trie

```rust
use build_trie::build_trie;

#[derive(Debug)]
pub enum Tokens {
    OpenBrace, CloseBrace, ArrowFunction, Equal, StrictEqual, Assign, Literal(String)
}

build_trie! {
    function: fn get_symbol_from_state_and_char;
    result: Tokens;
    state_enum: enum SymbolState;
    result_enum: enum SymbolStateResult;
    mappings: {
        "{" => Tokens::OpenBrace,
        "}" => Tokens::CloseBrace,
        "=>" => Tokens::ArrowFunction,
        "==" => Tokens::Equal,
        "===" => Tokens::StrictEqual,
        "=" => Tokens::Assign
    }
}
```