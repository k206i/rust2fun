use proc_macro::{TokenStream, TokenTree};

use quote::{format_ident, quote};

#[proc_macro]
pub fn curry_arity(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("curry{}", arity);
    let fn_args = (0..arity)
        .map(|i| format_ident!("x{}", i))
        .collect::<Vec<_>>();
    let msg = format!("Curry a function of {arity} arguments.");

    let expanded = quote! {
        #[doc = #msg]
        #[macro_export]
        macro_rules! #fn_name {
            ($f:expr) => {
                #( move | #fn_args | )* $f( #( #fn_args ),* )
            };
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn constant_arity(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("constant{}", arity);
    let fn_args = (0..arity).map(|_| quote!(_));
    let msg_args = vec!["_"; arity as usize].join(", ");
    let msg = format!(
        "The constant function with {arity} arguments *constant{arity}(x) = ({msg_args}) -> x*."
    );

    let expanded = quote! {
        #[doc = #msg]
        #[macro_export]
        macro_rules! #fn_name {
            ($x:expr) => {
                | #( #fn_args ),* | $x
            };
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn tuple_arity(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("tuple{}", arity);
    let types = ('A'..='Z').take(arity as usize);
    let type_args = types.clone().map(|t| format_ident!("{}", t));
    let args = types.map(|x| format_ident!("{}", x.to_lowercase().next().unwrap()));
    let fn_args = args
        .clone()
        .zip(type_args.clone())
        .map(|(a, t)| quote!(#a: #t));
    let return_type = type_args.clone();
    let msg = format!("Create a tuple of {arity} elements.");

    let expanded = quote! {
        #[doc = #msg]
        #[inline]
        pub const fn #fn_name< #( #type_args ),* >( #( #fn_args ),* ) -> ( #( #return_type ),* ) {
            ( #( #args ),* )
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn noop_arity(input: TokenStream) -> TokenStream {
    let arity = parse_arity(input);
    let fn_name = format_ident!("noop{}", arity);
    let type_args = ('A'..='Z')
        .take(arity as usize)
        .map(|t| format_ident!("{}", t));
    let fn_args = type_args.clone().map(|t| quote!(_: #t));
    let msg = format!("The no operation function of {arity} arguments.");

    let expanded = quote! {
        #[doc = #msg]
        #[inline]
        pub fn #fn_name< #( #type_args ),* >( #( #fn_args ),*) {}
    };

    TokenStream::from(expanded)
}

fn parse_arity(input: TokenStream) -> u32 {
    match input.into_iter().next().expect("arity is required") {
        TokenTree::Literal(x) => x
            .to_string()
            .parse::<u32>()
            .expect("arity must be a number"),
        _ => panic!("arity must be a literal"),
    }
}
