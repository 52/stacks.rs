// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use syn::DeriveInput;
use syn::Result;

pub(crate) fn inner(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    Err(syn::Error::new_spanned(
        input,
        "This is a dummy implementation of the derive macro",
    ))
}
