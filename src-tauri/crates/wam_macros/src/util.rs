use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Punct, Spacing};
use quote::TokenStreamExt;

macro_rules! return_error {
    ($content:expr) => {
        syn::Error::new(Span::call_site().into(), $content)
            .into_compile_error()
            .into()
    };
}

macro_rules! match_error {
    ($call:expr) => {
        match $call {
            Ok(ok) => ok,
            Err(error) => return return_error!(error),
        }
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

pub fn to_arg_struct<T: FromMeta>(
    tokens: impl Into<proc_macro2::TokenStream>,
) -> Result<T, TokenStream> {
    match NestedMeta::parse_meta_list(tokens.into()) {
        Ok(attribute_arguments) => match T::from_list(&attribute_arguments) {
            Ok(args) => Ok(args),
            Err(error) => return Err(TokenStream::from(error.write_errors())),
        },
        Err(error) => return Err(TokenStream::from(Error::from(error).write_errors())),
    }
}
