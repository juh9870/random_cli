use crate::codegen::{CodegenState, TokensResult};
use crate::schema::{SchemaStructMember, SchemaStructMemberType};
use miette::Context;
use proc_macro2::Ident;
use quote::{format_ident, quote};

impl CodegenState {
    pub fn codegen_object(
        &mut self,
        name: Ident,
        mut fields: Vec<SchemaStructMember>,
        switch: Option<String>,
    ) -> TokensResult {
        fields.insert(
            0,
            SchemaStructMember {
                name: "Id".to_string(),
                ty: SchemaStructMemberType::Object,
                minvalue: None,
                maxvalue: None,
                typeid: Some(name.to_string()),
                options: Some("notnull".to_string()),
                case: None,
                alias: None,
                default: None,
                arguments: None,
                description: None,
            },
        );

        let is_switch = switch.is_some();

        let code = self
            .codegen_struct(name.clone(), fields, switch)
            .context("Failed to generate object data")?;

        let id_name = format_ident!("{}Id", name);

        let id_field_getter = if is_switch {
            quote!(*self.id())
        } else {
            quote!(self.id)
        };

        Ok(quote! {
            pub type #id_name = DatabaseItemId::<#name>;
            #code

            impl DatabaseItemWithId for #name {
                fn id(&self) -> DatabaseItemId<Self> {
                    #id_field_getter
                }
            }
        })
    }
}
