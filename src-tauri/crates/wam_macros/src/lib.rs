use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Punct, Spacing};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, Ident, ImplItem, ItemImpl, Pat,
    PatIdent,
};

#[derive(Clone)]
struct FunctionParameters {
    state: State,
    typed_params: Vec<(PatIdent, String)>,
}

#[derive(Clone)]
struct State {
    name: String,
    use_state: bool,
}

#[derive(Default, FromMeta)]
struct WithTauriCommandArgs {
    state: Option<String>,
    body_state: Option<String>,
}

impl ToTokens for FunctionParameters {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.state.use_state {
            tokens.append(Ident::from_string(&self.state.name).expect("invalid state name"));
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
            if self.state.use_state || i != 0 {
                tokens.append(Punct::new(',', Spacing::Alone));
            }
            let (ident, path) = &self.typed_params[i];
            tokens.append(ident.ident.clone());
            tokens.append(Punct::new(':', Spacing::Alone));
            tokens.append(Ident::from_string(&path).expect("Invalid parameter path."));
        }
    }
}

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.use_state {
            tokens.append(Ident::from_string(&self.name).expect("Invalid state name"));
            tokens.append(Punct::new('.', Spacing::Alone));
        }
    }
}

impl FunctionParameters {
    fn new(state: String) -> Self {
        Self {
            state: State {
                name: state,
                use_state: false,
            },
            typed_params: Vec::new(),
        }
    }

    fn add_new(&mut self, punctuated: &Punctuated<FnArg, Comma>) {
        for arg in punctuated {
            match arg {
                FnArg::Receiver(_) => {
                    self.state.use_state = true;
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
                        _ => {} // TODO: Implement behavior.
                    },
                    _ => {} // TODO: Implement behavior.
                },
            }
        }
    }

    fn get_params(&self) -> Vec<PatIdent> {
        let mut vec = Vec::with_capacity(self.typed_params.len());
        for (pat_ident, _) in &self.typed_params {
            vec.push(pat_ident.clone());
        }
        vec
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
                            function_names.push(function.sig.clone().ident);
                            let mut params = FunctionParameters::new(
                                parsed_args.state.clone().unwrap_or(String::from("state")),
                            );
                            params.add_new(&function.sig.inputs);
                            function_params.push(params.clone());
                        }
                    }
                }
            }
            _ => (),
        }
    }

    if !function_names.is_empty() {
        let mut functions = Vec::new();

        functions.push(input);
        for (i, name) in function_names.iter().enumerate() {
            let parameters = function_params.get(i).unwrap(); // If we got to this point, we can assume the items exist.
            let call_params = parameters.get_params();
            let body_state = evaluate_body_state(
                &parsed_args
                    .body_state
                    .clone()
                    .unwrap_or(parameters.state.name.clone()),
            );

            functions.push(
                quote! {
                    pub fn #name(#parameters) {
                        #body_state.#name(#(#call_params)*);
                    }
                }
                .into(),
            )
        }

        return TokenStream::from_iter(functions);
    }
    quote! {}.into()
}

/// Supplementary macro for [contains_tauri_commands] to use.
#[proc_macro_attribute]
pub fn with_tauri_command(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

fn evaluate_body_state(body_state: &String) -> proc_macro2::TokenStream {
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
