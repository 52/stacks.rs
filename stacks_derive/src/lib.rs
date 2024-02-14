// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod from_tuple;

#[proc_macro_derive(FromTuple, attributes(key))]
pub fn derive_from_tuple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    from_tuple::inner(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
