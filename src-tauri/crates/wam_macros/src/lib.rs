use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Punct, Spacing};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, Ident, ImplItem, ItemImpl, Pat,
    PatIdent,
};

#[derive(Default)]
struct FunctionParameters {
    state: Option<String>,
    typed_params: Vec<(PatIdent, String)>,
}

#[derive(Default, FromMeta)]
struct WithTauriCommandArgs {
    #[darling(default)]
    _replace_self: Option<bool>,
    _state: Option<String>,
}

impl ToTokens for FunctionParameters {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(state) = &self.state {
            tokens.append(Ident::from_string(state).expect("invalid state name"));
            tokens.append(Punct::new(':', Spacing::Alone));
            tokens.append(
                Ident::from_string("State").expect("Invalid state type. This should not happen"),
            );
            tokens.append(Punct::new('<', Spacing::Alone));
            tokens.append(
                Ident::from_string("AppState").expect("Could not parse provided state type."),
            );
            tokens.append(Punct::new('>', Spacing::Alone));
        }
        for i in 0..self.typed_params.len() {
            if self.state.is_some() || i != 0 {
                tokens.append(Punct::new(',', Spacing::Alone));
            }
            let (ident, path) = &self.typed_params[i];
            tokens.append(ident.ident.clone());
            tokens.append(Punct::new(':', Spacing::Alone));
            tokens.append(Ident::from_string(&path).expect("Invalid parameter path."));
        }
    }
}

impl FunctionParameters {
    fn add_new(&mut self, punctuated: &Punctuated<FnArg, Comma>) {
        for arg in punctuated {
            match arg {
                FnArg::Receiver(_) => {
                    self.state = Some("state".to_owned());
                }
                FnArg::Typed(pat_type) => match &*pat_type.pat {
                    Pat::Ident(ident) => match &*pat_type.ty {
                        syn::Type::Path(path) => {
                            let mut path_string = String::new();
                            for (i, segment) in path.path.segments.clone().iter().enumerate() {
                                path_string.push_str(&segment.ident.to_string());
                                if i > 0 && i < path.path.segments.len() - 1 {
                                    path_string.push_str("::");
                                }
                            }
                            self.typed_params.push((ident.clone(), path_string));
                        }
                        _ => {}
                    },
                    _ => {}
                },
            }
        }
    }
}

#[proc_macro_attribute]
pub fn contains_tauri_commands(_: TokenStream, input: TokenStream) -> TokenStream {
    let temp_input = input.clone();
    let parsed_input = parse_macro_input!(temp_input as ItemImpl);

    let mut function_names = Vec::new();
    let mut function_params = Vec::new();

    for item in parsed_input.items {
        match item {
            ImplItem::Fn(function) => {
                for attribute in function.attrs {
                    if let Some(last_segment) = attribute.path().segments.last() {
                        if last_segment.ident == "with_tauri_command" {
                            function_names.push(function.sig.ident.clone());
                            let mut params = FunctionParameters::default();
                            params.add_new(&function.sig.inputs);
                            function_params.push(params);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    if !function_names.is_empty() {
        return TokenStream::from_iter([
            input,
            quote! {
                pub fn #(#function_names)*(#(#function_params)*) {}
            }
            .into(),
        ]);
    }
    quote! {
        pub fn shit() {}
    }
    .into()
}

/// Supplementary macro for [contains_tauri_commands] to use.
#[proc_macro_attribute]
pub fn with_tauri_command(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}
