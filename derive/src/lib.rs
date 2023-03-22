// SPDX-License-Identifier: MIT OR Apache-2.0 WITH LLVM-exception

//! Derive macro for `pretty_error_debug`

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote_spanned;
use syn::{parse_macro_input, DeriveInput};

/// Derive `std::fmt::Debug` using `pretty_error_debug`
///
/// For `struct MyError` this code would be derived:
///
/// ```rust
/// #   struct MyError;
/// #   impl std::error::Error for MyError {}
/// #   impl std::fmt::Display for MyError {
/// #       fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/// #           Ok(())
/// #       }
/// #   }
/// impl std::fmt::Debug for MyError {
///     #[inline]
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/// #       /*
///         pretty_error_debug::pretty_error_debug(self, f)
/// #       */ Ok(())
///     }
/// }
/// ```
#[proc_macro_derive(PrettyDebug)]
pub fn pretty_error_debug_derive_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let span = ident.span();

    let import = if let Ok(FoundCrate::Name(name)) = crate_name("pretty-error-debug") {
        let crate_name = syn::Ident::new(&name, span);
        quote_spanned!(
            span =>
            extern crate #crate_name as pretty_error_debug;
        )
    } else {
        quote_spanned!(span =>)
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    TokenStream::from(quote_spanned! {
        span =>
        const _: () = {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #import
                    pretty_error_debug::pretty_error_debug(self, f)
                }
            }
        };
    })
}
