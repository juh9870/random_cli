use crate::codegen::structs::StructData;
use crate::codegen::switch::Variant;
use crate::schema::{SchemaDataType, SchemaItem};
use miette::{miette, Diagnostic, IntoDiagnostic, LabeledSpan, Report, SourceCode};
use miette::{Context, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::iter::once;
use thiserror::Error;

mod enums;
mod objects;
mod structs;
mod switch;

type TokensResult = Result<TokenStream>;

#[derive(Debug, Default)]
pub struct CodegenState {
    pub enums: HashMap<String, Vec<String>>,
    pub objects: HashMap<String, StructData>,
}

impl CodegenState {
    pub fn codegen(&mut self, item: SchemaItem) -> Result<Option<TokenStream>> {
        let tokens = match item {
            SchemaItem::Schema { .. } => {
                quote! {
                    pub use crate::helpers::*;
                }
            }
            SchemaItem::Data(data) => {
                let ident = format_ident!("{}", data.name);

                match data.ty {
                    SchemaDataType::Struct | SchemaDataType::Settings => {
                        let obj = self
                            .codegen_struct(
                                ident,
                                data.member.ok_or_else(|| {
                                    miette!("Got struct or settings without members")
                                })?,
                                data.switch,
                            )
                            .context("Failed to generate struct data")?;
                        let code = obj.code.clone();
                        if let Some(id) = &data.typeid {
                            self.objects.insert(id.clone(), obj);
                        }
                        code
                    }
                    SchemaDataType::Object => {
                        let obj = self
                            .codegen_object(
                                ident,
                                data.member
                                    .ok_or_else(|| miette!("Got object without members"))?,
                                data.switch,
                            )
                            .context("Failed to generate object data")?;
                        let code = obj.code.clone();
                        if let Some(id) = &data.typeid {
                            self.objects.insert(id.clone(), obj);
                        }
                        code
                    }
                    SchemaDataType::Enum => self
                        .codegen_enum(
                            ident,
                            data.item.ok_or_else(|| miette!("Got enum without items"))?,
                        )
                        .context("Failed to generate enum data")?,
                    SchemaDataType::Expression => return Ok(None),
                }
            }
        };

        Ok(Some(tokens))
    }

    pub fn codegen_core_db_item(&mut self) -> TokensResult {
        let data = self
            .enums
            .get("ItemType")
            .ok_or_else(|| miette!("`ItemType` enum was not present in schema"))?;

        let mut variants = vec![];
        for variant in data {
            if variant == "Undefined" {
                continue;
            }
            let data = self.objects.remove(variant).ok_or_else(|| {
                miette!(
                    "Object or Setting with typeid `{}` was not present in schema",
                    variant
                )
            })?;
            variants.push(Variant {
                ident: format_ident!("{variant}"),
                data,
            })
        }

        let id_fetchers = variants.iter().map(|Variant { ident, data }| {
            if let Some(id_access) = &data.id_access {
                quote! {
                    Self::#ident(x) => Some((#id_access).0),
                }
            } else {
                quote! {
                    Self::#ident(_) => None,
                }
            }
        });

        let ident = format_ident!("Item");
        let code = self.codegen_custom_switch(
            ident.clone(),
            format_ident!("ItemType"),
            variants.as_slice(),
            false,
            [],
            "ItemType",
            false,
        )?;

        Ok(quote! {
            #code

            impl #ident {
                /// Fetches untyped ID of the inner item, or None if content is a setting
                pub fn id(&self) -> Option<i32> {
                    match self {
                        #(#id_fetchers)*
                    }
                }
            }
        })
    }

    pub fn format_tokens(tokens: Option<TokenStream>) -> Result<Option<String>> {
        match tokens {
            None => Ok(None),
            Some(tokens) => {
                let source = tokens.to_string();

                let text = prettyplease::unparse(
                    &syn::parse_file(&source)
                        .into_diagnostic()
                        .map_err(|e| SourceParseError(source, e))
                        .context("Generated code is not a valid Rust")?,
                );
                Ok(Some(text))
            }
        }
    }
}

#[derive(Debug, Error)]
#[error("{}", .1)]
struct SourceParseError(String, Report);

impl Diagnostic for SourceParseError {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.0)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        Some(Box::new(once(LabeledSpan::new(None, 0, self.0.len()))))
    }
}
