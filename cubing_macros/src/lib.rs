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
        Ok(_alg) => quote! {
            {
                static PARSED_ALG: std::sync::LazyLock<cubing::alg::Alg> = std::sync::LazyLock::new(|| (#alg_string).parse::<cubing::alg::Alg>().unwrap());
                &*PARSED_ALG
            }
        }
        .into(), // TODO: construct alg data structure instead of parsing at runtime?
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
            quote! {
                {
                    static PARSED_MOVE: std::sync::LazyLock<cubing::alg::Move> = std::sync::LazyLock::new(|| {
                        cubing::alg::Move {
                            quantum: std::sync::Arc::new(cubing::alg::QuantumMove {
                                family: String::from(#move_family),
                                prefix: None,
                            }),
                            amount: #move_amount,
                        }
                    });
                    &*PARSED_MOVE
                }
             }
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
