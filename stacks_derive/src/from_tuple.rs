// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use darling::ast::Data;
use darling::util::Ignored;
use darling::FromDeriveInput;
use darling::FromField;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Error;
use syn::Ident;
use syn::Result;

use crate::parse_opts;

#[derive(FromDeriveInput)]
#[darling(supports(struct_any, enum_any))]
pub(crate) struct InputRecv {
    /// The name of the struct.
    ident: syn::Ident,
    /// The generics present.
    generics: syn::Generics,
    /// The data of the struct.
    data: Data<Ignored, FieldRecv>,
}

#[derive(Debug, FromField)]
#[darling(attributes(stacks))]
pub(crate) struct FieldRecv {
    /// Type of the field.
    ty: syn::Type,
    /// Gets the ident of the field.
    ///
    /// This can be `None`, in enums & tuple structs.
    ident: Option<syn::Ident>,
    /// Key used to map the field to a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(FromTuple)]
    /// struct Data {
    ///     #[stacks(key = "some-key")]
    ///     a: i128,
    /// }
    /// ```
    key: Option<String>,
    /// Indicate whether the field is a response.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(FromTuple)]
    /// struct Data {
    ///     #[stacks(key = "some-key", response)]
    ///     a: i128,
    /// }
    /// ```
    response: Option<bool>,
}

/// The internal error type for the `FromTuple` implementation.
pub(crate) enum __Error<Token>
where
    Token: quote::ToTokens,
{
    /// Cast error. (key, ty, ident)
    Cast(Token, Token, Token),
    /// Extraction error. (key, ident)
    Extract(Token, Token),
    /// Match error. (key, ident)
    Match(Token, Token),
}

impl<Token> ToTokens for __Error<Token>
where
    Token: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut stream = TokenStream::new();

        stream.extend(quote!(::stacks_rs::derive::Error::));

        match self {
            Self::Cast(key, ty, ident) => {
                stream.extend(quote!(Cast(#key.to_string(), stringify!(#ty).to_string(), stringify!(#ident).to_string())))
            }
            Self::Extract(key, ident) => {
                stream.extend(quote!(Extract(#key.to_string(), stringify!(#ident).to_string())))
            }
            Self::Match(key, ident) => {
                stream.extend(quote!(Match(#key.to_string(), stringify!(#ident).to_string())))
            }
        }

        tokens.extend(stream);
    }
}

pub(crate) fn __impl(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let recv = parse_opts::<InputRecv>(input);

    let InputRecv { ident, data, .. } = recv;
    let ty_name = ident.clone();

    let fields = data
        .as_ref()
        .take_struct()
        .expect("Enums are not yet supported.")
        .fields;

    if fields.is_empty() {
        return Err(Error::new_spanned(
            ident,
            "Unit structs are not yet supported.",
        ));
    }

    for field in &fields {
        if field.key.is_none() || field.ident.is_none() {
            return Err(Error::new_spanned(
                field.ident.to_token_stream(),
                "All fields must be named & have an explicit `key` attribute",
            ));
        }
    }

    let mut tokens = vec![];
    for field in &fields {
        let FieldRecv { ident, key, .. } = field;

        let mut stream = TokenStream::new();

        let key = key.to_token_stream();
        let ty_name = ty_name.to_token_stream();

        match &field.ty {
            syn::Type::Path(tp) if tp.path.is_ident("i128") => 'stream: {
                // handle i128 as response
                if field.response == Some(true) {
                    __internal_extract_and_cast(&mut stream, &key, &ty_name);
                    __internal_err_unwrap(&mut stream);
                    __internal_type_cast(&mut stream, &quote!(ResponseOk), &key, &ty_name);
                    __internal_err_unwrap(&mut stream);
                    stream.extend(quote!(.into_value()));
                    __internal_type_cast(&mut stream, &quote!(Int), &key, &ty_name);
                    __internal_err_unwrap(&mut stream);
                    stream.extend(quote!(.into_value()));
                } else {
                    __internal_extract_and_cast(&mut stream, &key, &ty_name);
                    __internal_if_chain_cast(&mut stream, &[quote!(Int)], &key, &ty_name);
                }
                break 'stream;
            }
            syn::Type::Path(tp) if tp.path.is_ident("u128") => 'stream: {
                // handle u128 as response
                if field.response == Some(true) {
                    __internal_extract_and_cast(&mut stream, &key, &ty_name);
                    __internal_err_unwrap(&mut stream);
                    __internal_type_cast(&mut stream, &quote!(ResponseOk), &key, &ty_name);
                    __internal_err_unwrap(&mut stream);
                    stream.extend(quote!(.into_value()));
                    __internal_type_cast(&mut stream, &quote!(UInt), &key, &ty_name);
                    __internal_err_unwrap(&mut stream);
                    stream.extend(quote!(.into_value()));
                } else {
                    __internal_extract_and_cast(&mut stream, &key, &ty_name);
                    __internal_if_chain_cast(&mut stream, &[quote!(UInt)], &key, &ty_name);
                }
                break 'stream;
            }
            syn::Type::Path(tp) if tp.path.is_ident("bool") => 'stream: {
                __internal_extract_and_cast(&mut stream, &key, &ty_name);
                __internal_if_chain_cast(&mut stream, &[quote!(True), quote!(False)], &key, &ty_name);
                break 'stream;
            }
            syn::Type::Path(tp) if tp.path.is_ident("String") => 'stream: {
                __internal_extract_and_cast(&mut stream, &key, &ty_name);
                __internal_err_unwrap(&mut stream);
                stream.extend(quote!(.to_string()));
                break 'stream;
            }
            syn::Type::Path(tp) => 'stream: {
                let field = __internal_parse_type_path(tp);
                if let Some((ident, Some(ref args))) = field {

                    // handle Vec<u8>
                    if ident.eq("Vec") && args.to_string().eq("u8") {
                        __internal_extract_and_cast(&mut stream, &key, &ty_name);
                        __internal_if_chain_cast(&mut stream, &[quote!(Buffer)], &key, &ty_name);
                        break 'stream;
                    }

                    // handle Option<i128>
                    if ident.eq("Option") && args.to_string().eq("i128") {
                        __internal_extract_and_cast(&mut stream, &key, &ty_name);
                        __internal_derive_option(&mut stream, &quote!(Int));
                        break 'stream;
                    }

                    // handle Option<u128>
                    if ident.eq("Option") && args.to_string().eq("u128") {
                        __internal_extract_and_cast(&mut stream, &key, &ty_name);
                        __internal_derive_option(&mut stream, &quote!(UInt));
                        break 'stream;
                    }
                }

                // handle standalone T: TryFrom<Tuple>
                if let Some((ident, None)) = field {
                __internal_extract_and_cast(&mut stream, &key, &ty_name);
                __internal_err_unwrap(&mut stream);
                __internal_type_cast(&mut stream, &quote!(Tuple), &key, &ident.into_token_stream());
                stream.extend(quote!(.and_then(#ident::try_from)?));
                break 'stream;
                }

                return Err(Error::new_spanned(
                    tp.to_token_stream(),
                    "Unsupported type, did you mean ::std::vec::Vec<u8>, Option<T> or `T: TryFrom<Tuple>`?",
                ))
            }
            _ => return Err(Error::new_spanned(
                field.ty.to_token_stream(),
                "Unsupported type, expected one of: i128, u128, ::std::vec::Vec<u8>, Option<T>, bool, String, or `T: TryFrom<Tuple>`",
            )),
        };

        tokens.push(quote!(
            #ident: #stream
        ))
    }

    let InputRecv { generics, .. } = recv;
    let (imp, ty, wher) = generics.split_for_impl();

    Ok(quote!(
        impl #imp ::std::convert::TryFrom<::stacks_rs::clarity::Tuple> for #ident #ty #wher {
            type Error = ::stacks_rs::derive::Error;
            fn try_from(tuple: ::stacks_rs::clarity::Tuple) -> Result<Self, Self::Error> {
                use ::stacks_rs::clarity::Cast;
                Ok(Self { #(#tokens),* })
            }
        }
    ))
}

fn __internal_extract_and_cast<Token>(stream: &mut TokenStream, key: &Token, ident: &Token)
where
    Token: quote::ToTokens,
{
    let err = __Error::Extract(key, ident);

    stream.extend(quote! {
        tuple.get(#key).ok_or_else(||#err)
    });
}

fn __internal_if_chain_cast<Token>(
    stream: &mut TokenStream,
    types: &[Token],
    key: &Token,
    ident: &Token,
) where
    Token: quote::ToTokens,
{
    let err = __Error::Match(key, ident);

    let mut inner = TokenStream::new();
    for (i, ty) in types.iter().enumerate() {
        let cast_err = __Error::Cast(key, ty, ident);

        if i == usize::MIN {
            inner.extend(quote! { if })
        } else {
            inner.extend(quote! { else if })
        }

        inner.extend(quote! {
            value.as_any().is::<::stacks_rs::clarity::#ty>() {
                value
                .cast::<::stacks_rs::clarity::#ty>()
                .map(::stacks_rs::clarity::#ty::into_value)
                .map_err(|_| #cast_err)
            }
        })
    }

    stream.extend(quote! {
            .and_then(|value| {
                #inner else { Err(#err) }
            })?
    })
}

fn __internal_type_cast<Token>(stream: &mut TokenStream, ty: &Token, key: &Token, ident: &Token)
where
    Token: quote::ToTokens,
{
    let err_cast = __Error::Cast(key, ty, ident);

    stream.extend(quote! {
        .cast::<::stacks_rs::clarity::#ty>()
        .map_err(|_| #err_cast)
    })
}

fn __internal_derive_option<Token>(stream: &mut TokenStream, ty: &Token)
where
    Token: quote::ToTokens,
{
    stream.extend(quote! {
        .map(|value| {
            value
            .cast::<::stacks_rs::clarity::OptionalSome>()
            .and_then(|value| value.cast::<::stacks_rs::clarity::#ty>())
            .map(::stacks_rs::clarity::#ty::into_value)
            .ok()
        })?
    })
}

fn __internal_err_unwrap(stream: &mut TokenStream) {
    stream.extend(quote! {?})
}

fn __internal_parse_type_path(ty: &syn::TypePath) -> Option<(&Ident, Option<TokenStream>)> {
    ty.path
        .segments
        .last()
        .and_then(|seg| match &seg.arguments {
            syn::PathArguments::AngleBracketed(it) => {
                let args = &it.args;
                Some((&seg.ident, Some(quote!(#args))))
            }
            syn::PathArguments::None => Some((&seg.ident, None)),
            _ => None,
        })
}
