extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use std::mem;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input, parse_quote,
    spanned::Spanned,
    Data, Fields, FnArg, GenericParam, Generics, LitInt, Token,
};

#[proc_macro_attribute]
pub fn exhaustive_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let output = match syn::Item::parse.parse(input.clone()) {
        Ok(syn::Item::Fn(mut orig_fn)) => {
            let max_choices: usize = parse_macro_input!(args as Option<LitInt>)
                .map(|v| v.base10_parse().unwrap())
                .unwrap_or(usize::MAX);

            let name = &orig_fn.sig.ident;
            let attrs = mem::replace(&mut orig_fn.attrs, Vec::new());

            let mut args = Vec::new();
            let bindings = orig_fn
                .sig
                .inputs
                .iter()
                .enumerate()
                .map(|(i, input)| match input {
                    FnArg::Typed(t) => {
                        let ty = &t.ty;
                        let ident = format_ident!("_v{i}");
                        args.push(ident.clone());
                        quote! {
                            let #ident = match <#ty>::arbitrary(&mut run) {
                                Ok(v) => v,
                                Err(_) => continue,
                            };
                        }
                    }
                    _ => panic!("Unsupported kind of function argument"),
                }).collect::<Vec<_>>();
            let debug_prints = orig_fn.sig.inputs.iter().zip(args.iter()).map(|(arg, arg_name)| {
                let aname = match arg {
                    FnArg::Typed(t) => t.pat.to_token_stream().to_string(),
                    _ => panic!("Unsupported kind of function argument"),
                };
                quote!{
                println!(">>>>> {} =====\n{:?}\n\n", #aname, #arg_name);
            }
            });

            quote! {
                #[test]
                #(#attrs)*
                fn #name() {
                    use ::exhaustive::Exhaustive;
                    use ::exhaustive::DataSource;

                    #orig_fn
                    let mut source = DataSource::new(#max_choices);
                    let mut count = 0;
                    while let Some(mut run) = source.next_run() {
                        #(#bindings)*

                        if let Err(e) = std::panic::catch_unwind(|| {
                            #name(#(#args),*);
                        }) {
                            println!("================================================================================");
                            println!("This panic happened while trying to execute the test with parameters:\n");
                            run.reset(#max_choices);
                            #(#bindings)*
                            #(#debug_prints)*
                            std::panic::resume_unwind(e);
                        }

                        count += 1;
                    }
                    assert!(count > 0, "Zero test cases were executed, this is probably a mistake. Try increasing the choice limit?");
                }
            }
        }
        _ => {
            let span = proc_macro2::TokenStream::from(input).span();
            let msg = "#[quickcheck] is only supported on statics and functions";

            syn::parse::Error::new(span, msg).to_compile_error()
        }
    };

    output.into()
}

#[proc_macro_derive(Exhaustive)]
pub fn derive_exhaustive(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics.clone());
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();

    let map_fields = |fields: &Fields| match fields {
        Fields::Named(fields) => {
            let tokens = fields
                .named
                .iter()
                .map(|field| {
                    let fname = &field.ident;
                    quote! {
                        #fname: ::exhaustive::Exhaustive::arbitrary(u)?
                    }
                })
                .collect::<Punctuated<_, Token![,]>>();
            quote! {
                { #tokens }
            }
        }
        Fields::Unnamed(fields) => {
            let tokens = fields
                .unnamed
                .iter()
                .map(|_field| {
                    quote! {
                        ::exhaustive::Exhaustive::arbitrary(u)?
                    }
                })
                .collect::<Punctuated<_, Token![,]>>();
            quote! {
                ( #tokens )
            }
        }
        Fields::Unit => quote!(),
    };
    let fn_impl = match input.data {
        Data::Struct(s) => {
            let fields = map_fields(&s.fields);
            quote! {
                Ok(#name #fields)
            }
        }
        Data::Enum(e) => {
            let variants = e
                .variants
                .iter()
                .enumerate()
                .map(|(i, variant)| {
                    let vname = &variant.ident;
                    let fields = map_fields(&variant.fields);
                    quote! {
                        #i => #name::#vname #fields
                    }
                })
                .collect::<Punctuated<_, Token![,]>>();

            let variant_count = e.variants.len();
            quote! {
                Ok(match u.choice(#variant_count)? {
                    #variants,
                    _ => unreachable!(),
                })
            }
        }
        Data::Union(_) => panic!("Can't implement Exhaustive for unions"),
    };

    quote! {
        impl<#generics> ::exhaustive::Exhaustive for #name #ty_generics #where_clause {
            fn arbitrary(u: &mut ::exhaustive::DataSourceTaker) -> Result<Self, ::exhaustive::ChoiceError> {
                #fn_impl
            }
        }
    }.into()
}

// Add a bound `T: Arbitrary` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(parse_quote!(::exhaustive::Exhaustive));
        }
    }
    generics
}
