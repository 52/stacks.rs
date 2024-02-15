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

        match self {
            Self::Cast(key, ty, ident) => {
                stream.extend(quote!("Failed to cast field '{}' as '{}' on struct '{}'", #key, stringify!(#ty), stringify!(#ident)))
            }
            Self::Extract(key, ident) => {
                stream.extend(quote!("Failed to extract value for field '{}' on '{}'", #key, stringify!(#ident)))
            }
            Self::Match(key, ident) => {
                stream.extend(quote!("Failed to match value for field '{}' on '{}'", #key, stringify!(#ident)))
            }
        }

        tokens.extend(quote!(::stacks_rs::Error::Derive(format!(#stream))));
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
                break 'stream __internal_derive_cast(&mut stream, &[quote!(Int)], &key, &ty_name)
            }
            syn::Type::Path(tp) if tp.path.is_ident("u128") => 'stream: {
                break 'stream __internal_derive_cast(&mut stream, &[quote!(UInt)], &key, &ty_name)
            }
            syn::Type::Path(tp) if tp.path.is_ident("bool") => 'stream: {
                break 'stream __internal_derive_cast(&mut stream, &[quote!(True), quote!(False)], &key, &ty_name);
            }
            syn::Type::Path(tp) if tp.path.is_ident("String") => 'stream: {
                break 'stream __internal_derive_string(&mut stream, &key, &ty_name);
            }
            syn::Type::Path(tp) => 'stream: {
                let field = tp.path.segments.last().and_then(|seg| {
                    match &seg.arguments {
                        syn::PathArguments::AngleBracketed(it) => {
                            let args = &it.args;
                            Some((&seg.ident, Some(quote!(#args))))
                        }
                        syn::PathArguments::None => Some((&seg.ident, None)),
                        _ => None,
                    }
                });

                if let Some((ident, Some(ref args))) = field {
                    if ident.eq("Vec") && args.to_string().eq("u8") {
                        break 'stream __internal_derive_cast(&mut stream, &[quote!(Buffer)], &key, &ty_name);
                    }
                }

                if let Some((ident, None)) = field {
                    break 'stream __internal_derive_type(&mut stream, &quote!(Tuple), &key, &ident.into_token_stream());
                }

                return Err(Error::new_spanned(
                    tp.to_token_stream(),
                    "Unsupported type, did you mean ::std::vec::Vec<u8> or `T: FromTuple`?",
                ))
            }
            _ => return Err(Error::new_spanned(
                field.ty.to_token_stream(),
                "Unsupported type, expected one of: i128, u128, ::std::vec::Vec<u8>, bool, String, or `T: FromTuple`",
            )),
        };

        tokens.push(quote!(
            #ident: #stream
        ))
    }

    let InputRecv { generics, .. } = recv;
    let (imp, ty, wher) = generics.split_for_impl();

    Ok(quote!(
        impl #imp #ident #ty #wher {
            pub fn from_tuple(tuple: ::stacks_rs::clarity::Tuple) -> Result<Self, ::stacks_rs::Error> {
                use ::stacks_rs::clarity::Cast;
                Ok(Self { #(#tokens),* })
            }
        }
    ))
}

fn __internal_derive_cast<Token>(
    stream: &mut TokenStream,
    types: &[Token],
    key: &Token,
    ident: &Token,
) where
    Token: quote::ToTokens,
{
    let extract_err = __Error::Extract(key, ident);
    let match_err = __Error::Match(key, ident);

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
        tuple.get(#key)
            .ok_or_else(|| #extract_err)
            .and_then(|value| {
                #inner else { Err(#match_err) }
            })?
    })
}

fn __internal_derive_type<Token>(stream: &mut TokenStream, ty: &Token, key: &Token, ident: &Token)
where
    Token: quote::ToTokens,
{
    let err_extract = __Error::Extract(key, ident);
    let err_cast = __Error::Cast(key, ty, ident);

    stream.extend(quote! {
        tuple.get(#key).ok_or_else(||#err_extract)?
        .cast::<::stacks_rs::clarity::#ty>()
        .map_err(|_| #err_cast)
        .and_then(#ident::from_tuple)?
    })
}

fn __internal_derive_string<Token>(stream: &mut TokenStream, key: &Token, ident: &Token)
where
    Token: quote::ToTokens,
{
    let err = __Error::Extract(key, ident);

    stream.extend(quote! {
        tuple.get(#key).ok_or_else(||#err)?.to_string()
    })
}
