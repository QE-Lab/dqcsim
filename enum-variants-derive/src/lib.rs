//! This crate adds a derive macro for the `EnumVariants` trait from the
//! [`enum_variants`](../enum_variants/index.html) crate.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_quote::quote;

/// Derive macro for `enum_variants::EnumVariants`.
///
/// This implements `enum_variants::EnumVariants`, `std::fmt::Display`, and
/// `std::str::FromStr` for the enum it is applied to. The enum must implement
/// `std::Clone` as well. Note that the enum variants cannot contain values,
/// as the EnumVariants implementation must be able to construct them
/// statically.
#[proc_macro_derive(EnumVariants)]
pub fn enum_variants_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Get the name for use in type_name().
    let ident = input.ident;
    let ident_string = ident.to_string();
    let ident_string_lower = ident_string.to_lowercase();

    // Get the enum variants.
    let data;
    match input.data {
        syn::Data::Enum(d) => data = d,
        _ => panic!("derive(EnumVariants) is only possible for enums"),
    }
    let variant_idents: Vec<syn::Ident> = data.variants.iter().map(|x| x.ident.clone()).collect();
    let variant_ident_strings: Vec<String> = variant_idents
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    let variant_ident_strings_lower: Vec<String> = variant_ident_strings
        .iter()
        .map(|x| x.to_lowercase())
        .collect();

    // Build the output, possibly using quasi-quotation.
    let expanded = quote! {

        impl enum_variants::EnumVariants for #ident {
            #[allow(dead_code)]
            fn type_name() -> &'static str {
                #ident_string
            }

            #[allow(dead_code)]
            fn variant_map() -> Vec<(&'static str, &'static Self)> {
                vec![#( (#variant_ident_strings, &#ident::#variant_idents) ),*]
            }

            #[allow(dead_code)]
            fn variants() -> Vec<&'static str> {
                vec![#( #variant_ident_strings ),*]
            }

            #[allow(dead_code)]
            fn type_name_lower() -> &'static str {
                #ident_string_lower
            }

            #[allow(dead_code)]
            fn variant_map_lower() -> Vec<(&'static str, &'static Self)> {
                vec![#( (#variant_ident_strings_lower, &#ident::#variant_idents) ),*]
            }

            #[allow(dead_code)]
            fn variants_lower() -> Vec<&'static str> {
                vec![#( #variant_ident_strings_lower ),*]
            }

        }

        impl ::std::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    #( #ident::#variant_idents => write!(f, #variant_ident_strings) ),*
                }
            }
        }

        impl ::std::str::FromStr for #ident {
            type Err = enum_variants::EnumVariantError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #ident::variant_from_str_fuzzy(s)
            }
        }

    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(expanded)
}
