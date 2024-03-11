use crate::codegen::{CodegenState, TokensResult};
use crate::m_try;
use crate::schema::SchemaEnumItem;
use itertools::Itertools;
use miette::bail;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::str::FromStr;

impl CodegenState {
    pub fn codegen_enum(&mut self, name: Ident, items: Vec<SchemaEnumItem>) -> TokensResult {
        let mut is_char = false;
        self.enums.insert(
            name.to_string(),
            items.iter().map(|i| i.name.clone()).collect(),
        );
        let variants: Vec<_> = items
            .iter()
            .map(|SchemaEnumItem { name, value:raw_value, .. }| {
                m_try(|| {
                    let ident = format_ident!("{}", name);
                    let value = match raw_value {
                        None => {
                            quote!()
                        }
                        Some(value) => match i32::from_str(value) {
                            Ok(num) => quote! { = #num },
                            Err(_) => {
                                if !value.starts_with('\'') || value.len() != 3 {
                                    bail!("Enum value must be an integer or a character in 'c' form, but got `{}`", value)
                                }

                                is_char = true;

                                let char_code = value
                                    .chars().nth(1)
                                    .expect("Length should be 3 here")
                                    as u32;

                                quote! {
                                = #char_code
                            }
                            }
                        },
                    };
                    let doc_comment = if let Some(value) = raw_value {
                        quote!(#[doc = #value])
                    } else {
                        quote!()
                    };
                    Ok(quote! {
                    #doc_comment
                    #ident #value,
                })
                })
            })
            .try_collect()?;

        let impls = if is_char {
            quote! {
                impl serde::Serialize for #name {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        self.to_string().serialize(serializer)
                    }
                }

                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let code = *self as u32;
                        if code == 0 {
                            write!(f, "")
                        } else {
                            write!(f, "{}", char::from_u32(code).unwrap())
                        }
                    }
                }
            }
        } else {
            quote! {
                impl serde::Serialize for #name {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        (*self as i32).serialize(serializer)
                    }
                }
            }
        };

        let repr = if is_char { quote!(u32) } else { quote!(i32) };

        let name_str = name.to_string();

        Ok(quote! {
            #[repr(#repr)]
            #[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
            pub enum #name {
                #[default]
                #(#variants)*
            }

            impl DatabaseItem for #name {
                fn validate(&mut self) {}

                fn type_name() -> &'static str {
                    #name_str
                }
            }

            #impls
        })
    }
}
