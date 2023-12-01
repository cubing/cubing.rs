use cubing_core::alg::{Alg, Move};
use litrs::StringLit;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use std::convert::TryFrom;

#[proc_macro]
pub fn parse_alg(input: TokenStream) -> TokenStream {
    // Adapted from https://stackoverflow.com/a/67678127
    let input: Vec<TokenTree> = input.into_iter().collect();
    if input.len() != 1 {
        return quote! { compile_error!("The cubing::alg!(…) macro only accepts a single string.") }.into();
    }

    let string_lit = match StringLit::try_from(&input[0]) {
        // Error if the token is not a string literal
        Err(e) => return e.to_compile_error(),
        Ok(lit) => lit,
    };

    let alg_string = string_lit.value();
    match alg_string.parse::<Alg>() {
        Ok(_alg) => quote! { (#alg_string).parse::<cubing::alg::Alg>().unwrap() }.into(), // TODO: construct alg data structure?
        Err(e) => {
            let message = format!(
                "Invalid alg passed to cubing::alg!(…) macro. Parse error: {}",
                e
            );
            quote! { compile_error!(#message) }.into()
        }
    }
}

#[proc_macro]
pub fn parse_move(input: TokenStream) -> TokenStream {
    // Adapted from https://stackoverflow.com/a/67678127
    let input: Vec<TokenTree> = input.into_iter().collect();
    if input.len() != 1 {
        return quote! { compile_error!("The cubing::alg!(…) macro only accepts a single string.") }.into();
    }

    let string_lit = match StringLit::try_from(&input[0]) {
        // Error if the token is not a string literal
        Err(e) => return e.to_compile_error(),
        Ok(lit) => lit,
    };

    let move_string = string_lit.value();
    match move_string.parse::<Move>() {
        Ok(_alg) => quote! { (#move_string).parse::<cubing::alg::Move>().unwrap() }.into(), // TODO: construct alg data structure?
        Err(e) => {
            let message = format!(
                "Invalid move passed to cubing::alg!(…) macro. Parse error: {}",
                e
            );
            quote! { compile_error!(#message) }.into()
        }
    }
}
