use build_trie::build_trie;

#[derive(Debug)]
pub enum Tokens {
    OpenBrace, CloseBrace, ArrowFunction, Equal, StrictEqual, Assign
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

fn main() {
    let x = "{} {} == = === => =={".to_owned();
    let mut state: SymbolState = SymbolState::NoState;
    let mut tokens = Vec::new();
    for chr in x.chars() {
        match get_symbol_from_state_and_char(&state, &chr) {
            SymbolStateResult::Result(tok, used) => {
                tokens.push(tok);
                state = SymbolState::NoState;
            }
            SymbolStateResult::NewState(new_state) => {
                state = new_state;
            }
        }
    }
    println!("{:#?}", tokens);
}