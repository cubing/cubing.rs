// TODO: report errors better.
// Maybe use https://docs.rs/proc-macro-error/latest/proc_macro_error/ ?

use cubing_core::alg::{Alg, Move};

use proc_macro::TokenStream;
use quote::quote;

use syn::parse_macro_input;

#[proc_macro]
pub fn parse_alg(item: TokenStream) -> TokenStream {
    let alg_string = parse_macro_input!(item as syn::LitStr).value();
    match alg_string.parse::<Alg>() {
        Ok(_alg) => quote! { (#alg_string).parse::<cubing_core::alg::Alg>().unwrap() }.into(), // TODO: construct alg data structure?
        Err(e) => {
            let message = format!(
                "Invalid alg passed to cubing::parse_alg!(…) macro. Parse error: {}",
                e
            );
            quote! { compile_error!(#message) }.into()
        }
    }
}

#[proc_macro]
pub fn parse_move(item: TokenStream) -> TokenStream {
    let move_string = parse_macro_input!(item as syn::LitStr).value();
    match move_string.parse::<Move>() {
        Ok(r#move) => {
            let move_family = &r#move.quantum.family;
            let move_amount = r#move.amount;
            // TODO: can we avoid constructing the move from scratch every time?
            quote! { cubing_core::alg::Move {
                quantum: std::sync::Arc::new(cubing_core::alg::QuantumMove {
                    family: String::from(#move_family),
                    prefix: None,
                }),
                amount: #move_amount,
            } }
            .into()
        }
        Err(e) => {
            let message = format!(
                "Invalid move passed to cubing::parse_move!(…) macro. Parse error: {}",
                e
            );
            quote! { compile_error!(#message) }.into()
        }
    }
}
