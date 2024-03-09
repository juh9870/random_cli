use crate::schema::{SchemaDataType, SchemaItem, SchemaStructMember, SchemaStructMemberType};
use miette::{miette, Diagnostic, IntoDiagnostic, LabeledSpan, Report, SourceCode};
use miette::{Context, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::iter::once;
use thiserror::Error;

mod enums;
mod structs;
mod switch;

type TokensResult = Result<TokenStream>;

#[derive(Debug, Default)]
pub struct CodegenState {
    pub enums: HashMap<String, Vec<String>>,
}

impl CodegenState {
    pub fn codegen(&mut self, item: SchemaItem) -> Result<Option<String>> {
        let tokens = match item {
            SchemaItem::Schema { .. } => {
                quote! {
                    pub trait DatabaseItem: serde::Serialize {
                        fn validate(&mut self);
                    }
                }
            }
            SchemaItem::Data(data) => {
                let ident = format_ident!("{}", data.name);

                match data.ty {
                    SchemaDataType::Struct | SchemaDataType::Settings => self
                        .codegen_struct(
                            ident,
                            data.member
                                .ok_or_else(|| miette!("Got struct or settings without members"))?,
                            data.switch,
                        )
                        .context("Failed to generate struct data")?,
                    SchemaDataType::Object => {
                        let mut members = data
                            .member
                            .ok_or_else(|| miette!("Got struct or settings without members"))?;

                        members.insert(
                            0,
                            SchemaStructMember {
                                name: "Id".to_string(),
                                ty: SchemaStructMemberType::Object,
                                minvalue: None,
                                maxvalue: None,
                                typeid: Some(data.name.clone()),
                                options: Some("notnull".to_string()),
                                case: None,
                                alias: None,
                                default: None,
                                arguments: None,
                                description: None,
                            },
                        );

                        self.codegen_struct(ident, members, data.switch)
                            .context("Failed to generate object data")?
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
