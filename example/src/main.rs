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

#[derive(PartialEq)]
enum State {
    Symbol(SymbolState),
    Literal,
    None,
}

fn main() {
    let source = "{} {} == = === => =={as==}}a".to_owned();
    println!("Source:\n\"{}\"", source);

    let mut state: State = State::None;
    let mut tokens = Vec::new();
    let mut start: usize = 0;
    for (idx, chr) in source.char_indices() {
        match state {
            State::Literal => {
                if !matches!(chr, 'a'..='z' | 'A'..='Z') {
                    tokens.push(Tokens::Literal(source[start..idx].to_owned()));
                    start = 0;
                    state = State::None;
                }
            }
            State::Symbol(ref mut symbol_state) => {
                match get_symbol_from_state_and_char(&symbol_state, &chr) {
                    SymbolStateResult::Result(tok, used) => {
                        tokens.push(tok);
                        *symbol_state = SymbolState::None;
                        state = State::None;
                        if used {
                            continue;
                        }
                    }
                    SymbolStateResult::NewState(new_state) => {
                        *symbol_state = new_state;
                    }
                }
            }
            State::None => {}
        }
        if state == State::None {
            match chr {
                'a'..='z' | 'A'..='Z' => {
                    start = idx;
                    state = State::Literal;
                }
                chr if chr.is_whitespace() => {}
                chr => {
                    match get_symbol_from_state_and_char(&SymbolState::None, &chr) {
                        SymbolStateResult::Result(tok, _) => {
                            tokens.push(tok);
                        }
                        SymbolStateResult::NewState(new_state) => {
                            state = State::Symbol(new_state);
                        }
                    }
                }
            }
        }
    }
    // Trailing state
    match state {
        State::Literal => {
            tokens.push(Tokens::Literal(source[start..].to_owned()));
        }
        State::Symbol(symbol_state) => {
            match get_symbol_from_state_and_char(&symbol_state, &(0 as char)) {
                SymbolStateResult::Result(tok, _) => {
                    tokens.push(tok);
                }
                SymbolStateResult::NewState(_) => {
                    panic!();
                }
            }
        }
        State::None => {}
    }

    println!("Resulting tokens:\n{:#?}", tokens);
}