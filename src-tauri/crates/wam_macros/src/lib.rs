use std::{ops::Deref, str::FromStr};

use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::{Span, TokenStream};
use proc_macro2::{Punct, Spacing};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, GenericArgument, Ident, ImplItem, ItemImpl, Pat, PatIdent, ReturnType, Type
};

macro_rules! return_error {
    ($content:expr) => {
        syn::Error::new(Span::call_site().into(), $content).into_compile_error().into()
    };
}

#[derive(Clone)]
struct FunctionData {
    state: State,
    typed_params: Vec<(PatIdent, String)>,
    return_type: ReturnType,
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
    #[darling(default)]
    mutex_behavior: MutexBehavior,
}

#[derive(Default)]
enum MutexBehavior {
    #[default]
    None,
    Lock,
    MatchToOption,
}

impl ToTokens for FunctionData {
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

impl FunctionData {
    fn new(state: String, return_type: ReturnType) -> Self {
        Self {
            state: State {
                name: state,
                use_state: false,
            },
            return_type,
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

impl FromMeta for MutexBehavior {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.to_ascii_lowercase().as_str() {
            "none" => darling::Result::Ok(Self::None),
            "lock" => darling::Result::Ok(Self::Lock),
            "match_to_option" => darling::Result::Ok(Self::MatchToOption),
            _ => panic!("An invalid option was supplied for 'mutex_behavior'"),
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
    let mut function_data_vec = Vec::new();

    for item in parsed_input.items {
        match item {
            ImplItem::Fn(function) => {
                for attribute in function.attrs {
                    if let Some(last_segment) = attribute.path().segments.last() {
                        if last_segment.ident == "with_tauri_command" {
                            function_names.push(function.sig.clone().ident);
                            let mut params = FunctionData::new(
                                parsed_args.state.clone().unwrap_or(String::from("state")),
                                function.sig.output.clone(),
                            );
                            params.add_new(&function.sig.inputs);
                            function_data_vec.push(params.clone());
                        }
                    }
                }
            }
            _ => (),
        }
    }

    if !function_names.is_empty() {
        let mut output_stream = Vec::new();

        output_stream.push(input);
        for (i, name) in function_names.iter().enumerate() {
            output_stream.push(
                TokenStream::from_str(&"#[tauri::command]")
                    .expect("Cannot add tauri command attribute."),
            );

            let function_data = function_data_vec.get(i).unwrap(); // If we got to this point, we can assume the items exist.
            let call_params = function_data.get_params();
            let body_state = evaluate_body_state(
                &parsed_args
                    .body_state
                    .clone()
                    .unwrap_or(function_data.state.name.clone()),
            );
            let mut option_type = None;
            let return_type = match &function_data.return_type {
                // ReturnType::Default => Type::Infer(syn::TypeInfer { underscore_token:  }),
                ReturnType::Type(_, typed) => match typed.deref() {
                    Type::Reference(reference) => {
                        match &*reference.elem {
                            Type::Path(path) => {
                                // Special Case: Option: Don't double up on options when using the MatchToOption mutex behavior
                                // We know the first segment exists because we can't get to this point if it doesn't.
                                let first_segment = path.path.segments.first().unwrap();
                                if first_segment.ident.to_string().ends_with("Option") {
                                    match &first_segment.arguments {
                                        syn::PathArguments::AngleBracketed(arguments) => {
                                            if let Some(first) = arguments.args.first() {
                                                match first {
                                                    GenericArgument::Type(typed) => {
                                                        match typed {
                                                            Type::Path(path) => {
                                                                option_type = Some(path.path.to_token_stream());
                                                            },
                                                            _ => return return_error!("Currently, the option can only contain one layer of plain types.")
                                                        }
                                                    },
                                                    _ => return return_error!("Currently, the option can only contain types.")
                                                }
                                            } else {
                                                return return_error!("The provided option is invalid, or you are not using 'std::option::Option'")
                                            }
                                        }
                                        _ => return return_error!("The provided option is invalid")
                                    }
                                }
                            },
                            _ => return return_error!("This macro does not currently support anything but one type nesting.")
                        }

                        reference.elem.to_token_stream()
                    },
                    Type::Path(path) => path.path.to_token_stream().into(),
                    _ => return_error!("ugh"),
                },
                ReturnType::Default => TokenStream::from_str("()").unwrap().into(),
            };

            match parsed_args.mutex_behavior {
                MutexBehavior::None => output_stream.push(
                    quote! {
                        pub fn #name(#function_data) -> #return_type {
                            #body_state.#name(#(#call_params)*);
                        }
                    }
                    .into(),
                ),
                MutexBehavior::Lock => output_stream.push(
                    if function_data.return_type != ReturnType::Default {
                        return_error!("The 'lock' mutex behavior does not work when the original function has a return type.")
                    } else {
                        quote! {
                            pub fn #name(#function_data) {
                                match #body_state.lock() {
                                    Ok(guard) => guard.#name(#(#call_params)*),
                                    Err(_) => (),
                                }
                            }
                        }
                        .into()
                    }
                ),
                MutexBehavior::MatchToOption => output_stream.push(
                    if let Some(option_type) = option_type {
                        quote! {
                            pub fn #name(#function_data) -> Option<#option_type> {
                                match #body_state.lock().ok() {
                                    Some(guard) => match guard.#name(#(#call_params)*) {
                                        Some(some) => Some(some.clone()),
                                        None => None,
                                    },
                                    None => None,
                                }
                            }
                        }.into()
                    } else {
                        quote! {
                            pub fn #name(#function_data) -> Option<#return_type> {
                                match #body_state.lock().ok() {
                                    // We must clone here or else this could be a reference that's owned by the generated function
                                    Some(guard) => Some(guard.#name(#(#call_params)*).clone()),
                                    None => None,
                                }
                            }
                        }.into()
                    }
                )
            }
        }

        return TokenStream::from_iter(output_stream);
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
