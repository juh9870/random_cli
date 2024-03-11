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
                        fn type_name() -> &'static str;
                    }

                    pub struct DatabaseItemId<T: DatabaseItem>(pub i32, std::marker::PhantomData<T>);

                    impl<T: DatabaseItem> DatabaseItemId<T> {
                        pub fn new(id: i32) -> Self {
                            Self(id, Default::default())
                        }
                    }

                    impl<T: DatabaseItem> From<i32> for DatabaseItemId<T> {
                        fn from(x: i32) -> Self {
                            Self::new(x)
                        }
                    }

                    impl<T: DatabaseItem> From<DatabaseItemId<T>> for i32 {
                        fn from(x: DatabaseItemId<T>) -> Self {
                            x.0
                        }
                    }

                    impl<T: DatabaseItem> serde::Serialize for DatabaseItemId<T> {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: serde::Serializer,
                        {
                            self.0.serialize(serializer)
                        }
                    }

                    impl<T: DatabaseItem> PartialEq for DatabaseItemId<T> {
                        fn eq(&self, other: &Self) -> bool {
                            self.0 == other.0
                        }
                    }

                    impl<T: DatabaseItem> Eq for DatabaseItemId<T> {}

                    impl<T: DatabaseItem> Clone for DatabaseItemId<T> {
                        fn clone(&self) -> Self {
                            Self(self.0, Default::default())
                        }
                    }

                    impl<T: DatabaseItem> Copy for DatabaseItemId<T> {}

                    impl<T: DatabaseItem> std::fmt::Debug for DatabaseItemId<T> {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.debug_tuple(&format!("DatabaseItemId::<{}>", T::type_name()))
                                .field(&self.0)
                                .field(&format_args!("_"))
                                .finish()
                        }
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

                        let code = self
                            .codegen_struct(ident.clone(), members, data.switch)
                            .context("Failed to generate object data")?;

                        let id_name = format_ident!("{}Id", ident);
                        quote! {
                            type #id_name = DatabaseItemId::<#ident>;

                            #code
                        }
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
