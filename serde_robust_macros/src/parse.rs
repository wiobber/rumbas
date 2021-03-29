use proc_macro2::Span;
use syn::parse::{Error, Parse, ParseStream, Parser, Result};
use syn::{parenthesized, Data, DeriveInput, Fields, Ident, Meta, NestedMeta};

#[derive(Debug)]
pub struct Input {
    pub ident: Ident,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub ident: Ident,
    pub attrs: VariantAttrs,
}

#[derive(Debug, Clone)]
pub struct VariantAttrs {
    pub allow_override: Option<bool>, // TODO: allow_override induces read_safe
    pub read_safe: Option<bool>,      // TODO: rename, no rust unsafeness happening
}

fn parse_meta(attrs: &mut VariantAttrs, meta: &Meta) {
    if let Meta::List(value) = meta {
        for meta in &value.nested {
            if let NestedMeta::Meta(Meta::Path(path)) = meta {
                if path.is_ident("override") {
                    attrs.allow_override = Some(true);
                } else if path.is_ident("no_override") {
                    attrs.allow_override = Some(false);
                } else if path.is_ident("read_safe") {
                    attrs.read_safe = Some(true);
                } else if path.is_ident("read_unsafe") {
                    attrs.read_safe = Some(false);
                }
            }
        }
    }
}

fn parse_attrs(variant: &syn::Variant) -> Result<VariantAttrs> {
    let mut attrs = VariantAttrs {
        allow_override: None,
        read_safe: None,
    };
    for attr in &variant.attrs {
        if attr.path.is_ident("serde") {
            parse_meta(&mut attrs, &attr.parse_meta()?);
        }
    }
    Ok(attrs)
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let call_site = Span::call_site();
        let derive_input = DeriveInput::parse(input)?;

        /*
        let data = match derive_input.data {
            Data::Enum(data) => data,
            _ => {
                return Err(Error::new(call_site, "input must be an enum")); // TODO: structs
            }
        };

        let variants = data
            .variants
            .into_iter()
            .map(|variant| match variant.fields {
                Fields::Unit => {
                    let attrs = parse_attrs(&variant)?;
                    Ok(Variant {
                        ident: variant.ident,
                        attrs,
                    })
                }
                Fields::Named(_) | Fields::Unnamed(_) => {
                    Err(Error::new(variant.ident.span(), "must be a unit variant"))
                    // TODO: allow others
                }
            })
            .collect::<Result<Vec<Variant>>>()?;

        if variants.is_empty() {
            return Err(Error::new(call_site, "there must be at least one variant"));
        }
        */

        let generics = derive_input.generics;
        if !generics.params.is_empty() || generics.where_clause.is_some() {
            return Err(Error::new(call_site, "generic enum is not supported")); // TODO: check
        }

        let mut global_allow_override: Option<bool> = None;
        let mut global_read_safe: Option<bool> = None;
        let mut repr = None; // TODO: global settings
        for attr in derive_input.attrs {
            if attr.path.is_ident("repr") {
                fn repr_arg(input: ParseStream) -> Result<Ident> {
                    let content;
                    parenthesized!(content in input);
                    content.parse()
                }
                let ty = repr_arg.parse2(attr.tokens)?;
                repr = Some(ty);
                break;
            }
        }
        //let repr = repr.ok_or_else(|| Error::new(call_site, "missing #[repr(...)] attribute"))?;

        /* TODO: check variants with not allowed fields
            TODO: check if contradiction read_safe and allow_override
        let mut default_variants = variants.iter().filter(|x| x.attrs.is_default);
        let default_variant = default_variants.next().cloned();
        if default_variants.next().is_some() {
            return Err(Error::new(
                call_site,
                "only one variant can be #[serde(other)]",
            ));
        }*/

        Ok(Input {
            ident: derive_input.ident,
            variants: Vec::new(),
        })
    }
}
