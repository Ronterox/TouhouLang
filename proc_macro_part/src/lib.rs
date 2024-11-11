use proc_macro::TokenStream;

#[proc_macro_derive(Evaluate)]
pub fn proc_macro_derive(token_stream: TokenStream) -> TokenStream {
    let code = token_stream.to_string();
    format!("evaluate_derive! {{ {code} }}").parse().unwrap()
}
