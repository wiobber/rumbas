#[macro_use]
extern crate darling;
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use darling::ast;
use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, FromField)]
struct InputFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,

    /// This magic field name pulls the type from the input.
    ty: syn::Type,
}

#[derive(FromDeriveInput)]
#[darling(attributes(input))]
#[darling(supports(struct_any), forward_attrs(doc, derive))]
struct InputReceiver {
    /// The struct ident.
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<(), InputFieldReceiver>,
    attrs: Vec<syn::Attribute>,

    #[darling(rename = "name")]
    input_name: String,
}

#[proc_macro_derive(Input, attributes(input))]
pub fn derive_input(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let machine = match InputReceiver::from_derive_input(&derive_input) {
        Ok(sm) => sm,
        Err(e) => panic!("error in derive(Input): {}", e),
    };

    quote!(#machine).into()
}
impl ToTokens for InputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let InputReceiver {
            ref ident,
            ref generics,
            ref data,
            ref attrs,
            ref input_name,
        } = *self;

        /*let derive_attrs = attrs
            .iter()
            .filter(|a| a.path.is_ident("derive"))
            .collect::<Vec<_>>();
        eprintln!("{:?}", derive_attrs);*/

        let (imp, ty, wher) = generics.split_for_impl();
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        // Generate the format string which shows each field and its name
        let fmt_string = fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                // We have to preformat the ident in this case so we can fall back
                // to the field index for unnamed fields. It's not easy to read,
                // unfortunately.
                format!(
                    "{} = {{}}",
                    f.ident
                        .as_ref()
                        .map(|v| format!("{}", v))
                        .unwrap_or_else(|| format!("{}", i))
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        // Generate the actual values to fill the format string.
        let field_names = fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                // This works with named or indexed fields, so we'll fall back to the index so we can
                // write the output as a key-value pair.
                let field_ident = f.ident.as_ref().map(|v| quote!(#v)).unwrap_or_else(|| {
                    let i = syn::Index::from(i);
                    quote!(#i)
                });
                field_ident
            })
            .collect::<Vec<_>>();

        let input_type_tys = fields
            .into_iter()
            .enumerate()
            .map(|(_i, f)| {
                // This works with named or indexed fields, so we'll fall back to the index so we can
                // write the output as a key-value pair.
                match &f.ty {
                    syn::Type::Path(p) => {
                        let ident_opt = p.path.get_ident();
                        if let Some(ident) = ident_opt {
                            ident.to_owned()
                        } else {
                            panic!("{:?} is not a valid type for an Input struct.", p)
                        }
                    }
                    _ => panic!("{:?} is not a valid type for an Input struct.", f.ty),
                }
                //f.ty.clone()
            })
            .collect::<Vec<_>>();

        let input_ident = syn::Ident::new(&input_name, ident.span());
        tokens.extend(quote! {
            #[derive(Clone)]
            pub struct #input_ident #ty #wher {
                #(pub #field_names: <#input_type_tys as InputInverse>::Input),*
            }
        });
        tokens.extend(quote! {
            #[automatically_derived]
            impl #imp InputInverse for #ident #ty #wher {
                type Input = #input_ident #ty;
            }
            #[automatically_derived]
            impl #imp Input for #input_ident #ty #wher {
                type Normal = #ident #ty;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    Self::Normal {
                        #(#field_names: self.#field_names.to_normal()),*
                    }
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    Self {
                        #(#field_names: Input::from_normal(normal.#field_names)),*
                    }
                }
                fn find_missing(&self) -> InputCheckResult {
                    InputCheckResult::empty() // TODO
                }
            }
        });
    }
}
