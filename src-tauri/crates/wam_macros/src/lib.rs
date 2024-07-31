use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Punct, Spacing};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, Ident, ImplItem, ItemImpl, Pat,
    PatIdent,
};

struct FunctionParameters {
    state: String,
    use_state: bool,
    typed_params: Vec<(PatIdent, String)>,
}

#[derive(Default, FromMeta)]
struct WithTauriCommandArgs {
    state: Option<String>,
}

impl ToTokens for FunctionParameters {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.use_state {
            tokens.append(Ident::from_string(&self.state).expect("invalid state name"));
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
            if self.use_state || i != 0 {
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
    fn new(state: String) -> Self {
        Self {
            state,
            use_state: false,
            typed_params: Vec::new(),
        }
    }

    fn add_new(&mut self, punctuated: &Punctuated<FnArg, Comma>) {
        for arg in punctuated {
            match arg {
                FnArg::Receiver(_) => {
                    self.use_state = true;
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
pub fn contains_tauri_commands(args: TokenStream, input: TokenStream) -> TokenStream {
    let temp_input = input.clone();
    let parsed_input = parse_macro_input!(temp_input as ItemImpl);
    let parsed_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(attribute_arguments) => match WithTauriCommandArgs::from_list(&attribute_arguments) {
            Ok(args) => args,
            Err(error) => return TokenStream::from(error.write_errors()),
        },
        Err(error) => return TokenStream::from(Error::from(error).write_errors()),
    };

    let mut function_names = Vec::new();
    let mut function_params = Vec::new();

    for item in parsed_input.items {
        match item {
            ImplItem::Fn(function) => {
                for attribute in function.attrs {
                    if let Some(last_segment) = attribute.path().segments.last() {
                        if last_segment.ident == "with_tauri_command" {
                            function_names.push(function.sig.ident.clone());
                            let mut params = FunctionParameters::new(
                                parsed_args.state.clone().unwrap_or(String::from("state")),
                            );
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
    quote! {}
    .into()
}

/// Supplementary macro for [contains_tauri_commands] to use.
#[proc_macro_attribute]
pub fn with_tauri_command(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}
