use darling::FromMeta;
use proc_macro2::{Punct, Spacing, Ident};
use quote::TokenStreamExt;

macro_rules! return_error {
    ($content:expr) => {
        syn::Error::new(Span::call_site().into(), $content)
            .into_compile_error()
            .into()
    };
}

pub fn evaluate_body_state(body_state: &String) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    let parts = body_state.split('.');
    for (i, part) in parts.enumerate() {
        if i != 0 {
            tokens.append(Punct::new('.', Spacing::Alone));
        }
        tokens.append(Ident::from_string(part).expect("Invalid item within 'body_state'"));
    }

    tokens
}
