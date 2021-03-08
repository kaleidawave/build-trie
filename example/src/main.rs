use build_trie::build_trie;

enum Tokens {
    SingleBracket, DoubleBracket
}

build_trie! {
    function: fn get_symbol_from_state_and_char;
    result: Tokens;
    state_enum: enum SymbolState;
    result_enum: enum SymbolStateResult;
    mappings: {
        "{" => Tokens::SingleBracket,
        "{{" => Tokens::DoubleBracket,
    }
}