use crate::codegen::structs::Field;
use crate::codegen::{CodegenState, TokensResult};
use crate::schema::{SchemaStructMember, SchemaStructMemberType};
use itertools::Itertools;
use miette::{bail, miette};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use tracing::{debug, info, warn};

impl CodegenState {
    pub fn codegen_switch_struct(
        &mut self,
        ident: Ident,
        mut fields: Vec<SchemaStructMember>,
        switch: String,
    ) -> TokensResult {
        warn!(%ident, "Generating code for switch item");
        let switch_field_idx = fields
            .iter()
            .enumerate()
            .find(|f| f.1.name == switch)
            .map(|(i, _)| i)
            .ok_or_else(|| miette!("switch field points at a missing field"))?;

        let switch_field = fields.remove(switch_field_idx);

        if !matches!(switch_field.ty, SchemaStructMemberType::Enum) {
            bail!("switch field must be an enum");
        };
        let Some(ty) = &switch_field.typeid else {
            bail!("switch field is missing a typeid")
        };
        let Some(enum_items) = self.enums.get(ty) else {
            bail!("switch typeid points at the unknown enum `{}`", ty)
        };

        let mut variants: HashMap<String, Vec<SchemaStructMember>> = HashMap::default();

        for item in enum_items {
            variants.insert(item.clone(), vec![]);
        }

        let mut neutrals = vec![];

        for field in fields {
            match &field.case {
                None => {
                    for members in variants.values_mut() {
                        members.push(field.clone())
                    }
                    neutrals.push(field);
                }
                Some(cases) => {
                    let cases = cases.split(',').map(|e| e.trim());
                    for case in cases {
                        let Some(members) = variants.get_mut(case) else {
                            bail!("Field {} contains unknown case `{}`", &field.name, case)
                        };

                        members.push(field.clone());
                    }
                }
            }
        }

        let names: Vec<Ident> = enum_items
            .iter()
            .map(|variant| format_ident!("{}{}", ident, variant))
            .collect();

        let variant_names: Vec<Ident> = enum_items
            .iter()
            .map(|variant| format_ident!("{}", variant))
            .collect();

        let enum_variants = names
            .iter()
            .zip(variant_names.iter())
            .map(|(ident, variant_ident)| {
                quote! {
                    #variant_ident(#ident),
                }
            });

        let tag_name = switch_field.name;

        let default_item = &variants[&enum_items[0]];

        let has_default = !default_item.iter().any(|f| {
            f.options
                .as_ref()
                .is_some_and(|opt| opt.contains("notnull"))
        });

        let default_impl = has_default.then(|| {
            let first_variant = &variant_names[0];
            quote! {
                impl Default for #ident {
                    fn default() -> Self {
                        Self::#first_variant(Default::default())
                    }
                }
            }
        });

        let shared_enum = quote! {
            #[derive(Debug, Clone, serde::Serialize)]
            #[serde(tag = #tag_name)]
            pub enum #ident {
                #(#enum_variants)*
            }

            #default_impl
        };

        let mut blocks: Vec<TokenStream> = vec![shared_enum];

        let shared_fields: Vec<Field> = neutrals
            .into_iter()
            .map(|f| Field::new(f, &ident))
            .try_collect()?;

        for (struct_name, variant_name) in names.iter().zip(variant_names.iter()) {
            blocks.push(quote! {
                impl From<#struct_name> for #ident {
                    fn from(item: #struct_name) -> Self {
                        Self::#variant_name(item)
                    }
                }
            });
        }

        let matcher = |body: TokenStream| {
            let matches = variant_names.iter().map(|name| {
                quote! {
                    Self::#name(x) => {#body}
                }
            });

            quote! {
                match self {
                    #(#matches)*
                }
            }
        };

        for Field {
            ident: field_name,
            ty,
            ..
        } in shared_fields
        {
            let field_name_mut = format_ident!("{}_mut", field_name);
            let access = matcher(quote!(&x.#field_name));
            let access_mut = matcher(quote!(&mut x.#field_name));
            blocks.push(quote! {
                impl #ident {
                    pub fn #field_name(&self) -> &#ty {
                        #access
                    }

                    pub fn #field_name_mut(&mut self) -> &mut #ty {
                        #access_mut
                    }
                }
            });
        }

        for (name, fields) in variants.into_iter() {
            let variant_ident = format_ident!("{}{}", ident, name);
            let code = self.codegen_struct(variant_ident, fields, None)?;
            blocks.push(code);
        }

        Ok(quote! {
            #(#blocks)*
        })
    }
}
