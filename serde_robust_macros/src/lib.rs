extern crate proc_macro;

mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parse::Input;

use std::iter;

#[proc_macro_derive(Deserialize_robust, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    println!("{:?}", input);
    let ident = input.ident;
    let variants = input.variants.iter().map(|variant| &variant.ident);

    let declare_variants = input.variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            #variant,
        }
    });

    let match_variants = input.variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            Discriminant::#variant => #ident::#variant,
        }
    });

    let error_format = match input.variants.len() {
        0 => "invalid value: expected".to_owned(),
        1 => "invalid value: expected {:?}".to_owned(),
        2 => "invalid value: expected {:?} or {:?}".to_owned(),
        n => {
            "invalid value: expected one of: {:?}".to_owned()
                + &iter::repeat(", {:?}").take(n - 1).collect::<String>()
        }
    };

    let other_arm = quote! {
        serde_robust::Error::Invalid(
            //format_args!(#error_format #(, Discriminant::#variants)*)
            format!("Could not parse {}", stringify!(#ident)) // TODO
        )
    };

    let result_name = proc_macro2::Ident::new(
        &format!("Result{}", ident)[..],
        ident.span(), // TODO: which span?
    );
    println!("{:?}", result_name);

    let declare_result_type = quote! {
        #[derive(Debug)] // TODO: take from other
        struct #result_name(core::result::Result<#ident, serde_robust::Error>);
    };

    TokenStream::from(quote! {
     impl<'de> serde::Deserialize<'de> for Value<#ident> {
         fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                /*
                impl discriminant {
                    #(#declare_discriminants)*
                }*/

                core::result::Result::Ok(Value(<#ident as serde::Deserialize>::deserialize(deserializer).map_err(|_| #other_arm)))
            }
        }
    }) /**/
}
