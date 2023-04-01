use convert_case::Casing;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

pub fn enums_id_arena_to(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => {
            return Err(syn::Error::new(
                Span::call_site().into(),
                "This macro only supports enums.",
            ))
        }
    };
    let id_ident = format_ident!("{}Id", name);

    let mut extends = Vec::new();
    let mut impl_extends = Vec::new();
    let mut vec_body = Vec::new();
    let mut match_body = Vec::new();
    let enum_name_ident = format_ident!("{}ExtendEnum", name);

    for variant in variants {
        let ident = &variant.ident;
        let ident_case = format!("{}", ident).to_case(convert_case::Case::Snake);
        let fields = &variant.fields;

        let is_empty = fields.is_empty();
        let ret_ty = match fields.len() {
            0 => {
                quote! {
                    ()
                }
            }
            1 => {
                let field = fields.iter().next().unwrap();
                quote! {
                    #field
                }
            }
            _ => {
                return Err(syn::Error::new(
                    Span::call_site().into(),
                    "This macro doesn't support multi-fields enum",
                ))
            }
        };
        let alloc_ident = format_ident!("alloc_{}", ident_case);
        let var_ident = format_ident!("{}_vec", ident_case);

        extends.push(quote! {#ident});
        if !is_empty {
            vec_body.push(quote! {
                #var_ident: Vec<#ret_ty>
            });
            impl_extends.push(quote! {
                #[allow(unused)]
                pub fn #alloc_ident(&mut self, val: #ret_ty) -> #id_ident {
                    let index = self.enums_vec_id_offset_of.len() as u32;
                    let real_index = self.#var_ident.len() as u32;
                    self.#var_ident.push(val);
                    self.enums_vec_id_offset_of.push(real_index);
                    (#enum_name_ident::#ident, index)
                }
            });
            match_body.push(quote! {
                #enum_name_ident::#ident => {
                    Some(#name::#ident(
                        match self.#var_ident.get(ty_index as usize).cloned() {
                            Some(v) => v,
                            None => return None,
                        }
                    ))
                },
            });
        } else {
            impl_extends.push(quote! {
                #[allow(unused)]
                pub fn #alloc_ident(&mut self) -> #id_ident {
                    let index = self.enums_vec_id_offset_of.len() as u32;
                    self.enums_vec_id_offset_of.push(0);
                    (#enum_name_ident::#ident, index)
                }
            });
            match_body.push(quote! {
                #enum_name_ident::#ident => {
                    match ty_index {
                        0 => Some(#name::#ident),
                        _ => None,
                    }
                },
            });
        }
    }

    let vec_name_ident = format_ident!("{}IdArena", name);

    let res = quote! {
        type #id_ident = (#enum_name_ident, u32);
        enum #enum_name_ident {
            #(#extends),*
        }
        #[derive(Default)]
        struct #vec_name_ident {
            enums_vec_id_offset_of: Vec<u32>,
            #(#vec_body),*
        }
        impl #vec_name_ident {
            //! get value by id
            #[allow(unused)]
            pub fn get_cloned(&self, id: #id_ident) -> Option<#name> {
                let (ty, index) = id;
                let ty_index = self.enums_vec_id_offset_of.get(index as usize)?.clone();
                match ty {
                    #(#match_body)*
                }
            }

            #(#impl_extends)*
        }
    };

    Ok(res.into())
}
