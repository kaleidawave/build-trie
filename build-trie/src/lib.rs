use proc_macro::{TokenStream, Span};
use quote::quote;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    braced, parse_macro_input, parse_quote, punctuated::Punctuated, Arm, Expr, Ident, LitChar,
    LitStr, Token, 
};

struct BuildTrie {
    state_enum_name: Ident,
    result_enum_name: Ident,
    function_name: Ident,
    result_name: Ident,
    mappings: Punctuated<(LitStr, Expr), Token![,]>,
}

impl Parse for BuildTrie {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut function_name: Option<Ident> = None;
        let mut state_enum_name: Option<Ident> = None;
        let mut result_enum_name: Option<Ident> = None;
        let mut result_name: Option<Ident> = None;
        let mut mappings: Option<Punctuated<(LitStr, Expr), Token![,]>> = None;
        while !(state_enum_name.is_some()
            && result_enum_name.is_some()
            && function_name.is_some()
            && mappings.is_some()
            && result_name.is_some())
            && !input.is_empty()
        {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            match key.to_string().as_str() {
                "function" => {
                    if function_name.is_some() {
                        panic!("Function name already defined ")
                    }
                    input.parse::<Token![fn]>()?;
                    let name: Ident = input.parse()?;
                    input.parse::<Token![;]>()?;
                    function_name = Some(name);
                }
                "state_enum" => {
                    if state_enum_name.is_some() {
                        panic!("State enum name already defined ")
                    }
                    input.parse::<Token![enum]>()?;
                    let name: Ident = input.parse()?;
                    input.parse::<Token![;]>()?;
                    state_enum_name = Some(name);
                }
                "result_enum" => {
                    if result_enum_name.is_some() {
                        panic!("Result enum name already defined ")
                    }
                    input.parse::<Token![enum]>()?;
                    let name: Ident = input.parse()?;
                    input.parse::<Token![;]>()?;
                    result_enum_name = Some(name);
                }
                "result" => {
                    if result_name.is_some() {
                        panic!("Result reference already defined ")
                    }
                    let name: Ident = input.parse()?;
                    input.parse::<Token![;]>()?;
                    result_name = Some(name);
                }
                "mappings" => {
                    if mappings.is_some() {
                        panic!("Mappings already defined ")
                    }
                    let content;
                    braced!(content in input);
                    let mappings_result =
                        content.parse_terminated::<(LitStr, Expr), Token![,]>(|input| {
                            let string: LitStr = input.parse()?;
                            input.parse::<Token![=>]>()?;
                            let expr: Expr = input.parse()?;
                            Ok((string, expr))
                        })?;
                    mappings = Some(mappings_result);
                }
                _ => panic!("invalid key"),
            }
        }

        Ok(BuildTrie {
            state_enum_name: state_enum_name.expect("No state enum name"),
            result_enum_name: result_enum_name.expect("No result enum name"),
            function_name: function_name.expect("No function name"),
            mappings: mappings.expect("No mappings"),
            result_name: result_name.expect("No result name"),
        })
    }
}

struct Trie<K, V>(HashMap<K, Trie<K, V>>, Option<V>);

impl<K, V> Trie<K, V> {
    fn is_leaf(&self) -> bool {
        self.0.is_empty()
    }
}

const NO_STATE_NAME: &str = "None";

#[proc_macro]
pub fn build_trie(input: TokenStream) -> TokenStream {
    let BuildTrie {
        state_enum_name,
        result_enum_name,
        function_name,
        result_name,
        mappings,
    } = parse_macro_input!(input as BuildTrie);

    let mut trie: Trie<char, Expr> = Trie(HashMap::new(), None);

    for (string, value) in mappings {
        let mut node = &mut trie;
        for chr in string.value().chars() {
            if node.0.get(&chr).is_none() {
                node.0.insert(chr, Trie(HashMap::new(), None));
            }
            node = node.0.get_mut(&chr).unwrap();
        }
        node.1 = Some(value);
    }

    let mut states: Vec<Ident> = Vec::new();
    let mut arms: Vec<Arm> = Vec::new();

    fn expand_trie(
        trie: &Trie<char, Expr>,
        state_enum_name_ident: &Ident,
        result_enum_name_ident: &Ident,
        arms: &mut Vec<Arm>,
        states: &mut Vec<Ident>,
        prev_state: &Ident
    ) {
        let mut count: u8 = 0;
        for (key, value) in trie.0.iter() {
            let chr = LitChar::new(*key, Span::call_site().into());
            if value.is_leaf() {
                if let Some(value) = &value.1 {
                    let arm: Arm = parse_quote! {
                        (#state_enum_name_ident::#prev_state, #chr) => #result_enum_name_ident::Result(#value, true),
                    };
                    arms.push(arm);
                }
            } else {
                let new_state = {
                    let as_string = prev_state.to_string();
                    count += 1;
                    if as_string.is_empty() || as_string == NO_STATE_NAME {
                        let mut string = String::new();
                        string.push((count + 96) as char);
                        Ident::new(&string, Span::call_site().into())
                    } else {
                        let mut string = as_string.clone();
                        string.push((count + 96) as char);
                        Ident::new(&string, Span::call_site().into())
                    }
                };
                states.push(new_state.clone());
                let arm: Arm = parse_quote! {
                    (#state_enum_name_ident::#prev_state, #chr) => #result_enum_name_ident::NewState(#state_enum_name_ident::#new_state),
                };
                arms.push(arm);
                expand_trie(
                    value,
                    state_enum_name_ident,
                    result_enum_name_ident,
                    arms,
                    states,
                    &new_state
                );
                if let Some(value) = &value.1 {
                    let arm: Arm = parse_quote! {
                        (#state_enum_name_ident::#new_state, _) => #result_enum_name_ident::Result(#value, false),
                    };
                    arms.push(arm);
                }
            }
        }
    }

    let no_state_ident = Ident::new(NO_STATE_NAME, Span::call_site().into());

    expand_trie(
        &trie,
        &state_enum_name,
        &result_enum_name,
        &mut arms,
        &mut states,
        &no_state_ident
    );

    let expanded = quote! {
        pub enum #result_enum_name {
            Result(#result_name, bool),
            NewState(#state_enum_name)
        }

        #[derive(PartialEq)]
        pub enum #state_enum_name {
            None,
            #( #states ),*
        }

        pub fn #function_name(state: &#state_enum_name, chr: &char) -> #result_enum_name {
            match (state, chr) {
                #( #arms )*
                (#state_enum_name::#no_state_ident, _) => #result_enum_name::NewState(#state_enum_name::#no_state_ident),
                (state, chr) => panic!("Invalid {}", chr)
            }
        }
    };

    TokenStream::from(expanded)
}