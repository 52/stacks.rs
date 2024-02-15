// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

mod from_tuple;

#[proc_macro_derive(FromTuple, attributes(stacks))]
pub fn derive_from_tuple(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_tuple::__impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Parses the tokens from a `proc_macro::TokenStream` into a struct.
///
/// `T` is required to derive `darling::FromDeriveInput`.
pub(crate) fn parse_opts<T: darling::FromDeriveInput>(input: proc_macro::TokenStream) -> T {
    let ast = syn::parse(input).expect("failed to parse the input.");
    T::from_derive_input(&ast).expect("failed to parse derive input.")
}
