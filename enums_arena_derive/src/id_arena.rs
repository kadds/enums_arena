use convert_case::Casing;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated,
    token::{Gt, Lt},
    Data, DeriveInput, GenericParam, TypeParam,
};

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
    let generics = &ast.generics;
    let mut generics2 = generics.clone();
    generics2.params.clear();

    let mut generic_res = Vec::new();

    for p in &generics.params {
        match p {
            GenericParam::Lifetime(_) => continue,
            GenericParam::Type(t) => {
                generics2.params.push(p.clone());
                let i = t.ident.clone();
                generic_res.push(quote! {
                    #i: Clone
                })
            }
            GenericParam::Const(_c) => {}
        }
    }
    let user_generics = if generics2.params.is_empty() {
        quote! {}
    } else {
        quote! {
            ::#generics2
        }
    };
    let user_bound = quote! {
        #(#generic_res),*
    };

    let mut extends = Vec::new();
    let mut impl_extends = Vec::new();
    let mut vec_body = Vec::new();
    let mut match_body = Vec::new();
    let mut clear_extends = Vec::new();
    let enum_name_ident = format_ident!("{}ExtendEnum", name);

    let n = variants.len() as u64;
    let repr = if n <= u8::MAX as u64 {
        quote! {
            #[repr(u8)]
        }
    } else if n <= u16::MAX as u64 {
        quote! {
            #[repr(u16)]
        }
    } else if n <= u32::MAX as u64 {
        quote! {
            #[repr(u32)]
        }
    } else {
        quote! {
            #[repr(u64)]
        }
    };

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
                    "This macro doesn't support multi-fields in enums",
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
            clear_extends.push(quote! {
                self.#var_ident.clear()
            });
            impl_extends.push(quote! {
                #[allow(unused)]
                pub fn #alloc_ident(&mut self, val: #ret_ty) -> #id_ident<HIDE_I, HIDE_G> {
                    let index = HIDE_I::from_usize(self.enums_vec_id_offset_of.len());
                    let real_index = HIDE_I::from_usize(self.#var_ident.len());
                    self.#var_ident.push(val);
                    self.enums_vec_id_offset_of.push(real_index);
                    (#enum_name_ident::#ident, index, self.g)
                }
            });
            match_body.push(quote! {
                #enum_name_ident::#ident => {
                    Some(#name::#ident #user_generics(
                        self.#var_ident.get(ty_index.to_usize()).cloned()?
                    ))
                },
            });
        } else {
            impl_extends.push(quote! {
                #[allow(unused)]
                pub fn #alloc_ident(&mut self) -> #id_ident<HIDE_I, HIDE_G> {
                    let index = HIDE_I::from_usize(self.enums_vec_id_offset_of.len());
                    self.enums_vec_id_offset_of.push(HIDE_I::from_usize(0));
                    (#enum_name_ident::#ident, index, self.g)
                }
            });
            match_body.push(quote! {
                #enum_name_ident::#ident => {
                    match ty_index.to_usize() {
                        0 => Some(#name::#ident #user_generics),
                        _ => None,
                    }
                },
            });
        }
    }

    let mut new_generics = generics.clone();

    if new_generics.lt_token.is_some() {
    } else {
        new_generics.lt_token = Some(Lt::default());
        new_generics.gt_token = Some(Gt::default())
    }

    new_generics.params.push(syn::GenericParam::Type(TypeParam {
        ident: format_ident!("HIDE_I"),
        attrs: Vec::new(),
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    }));
    new_generics.params.push(syn::GenericParam::Type(TypeParam {
        ident: format_ident!("HIDE_G"),
        attrs: Vec::new(),
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    }));

    let arena_name_ident = format_ident!("{}IdArena", name);
    let impl_part = quote! {
        #[allow(unused)]
        pub fn len(&self) -> usize {
            self.enums_vec_id_offset_of.len()
        }
        #[allow(unused)]
        pub fn clear(&mut self) {
            self.g.add();
            self.enums_vec_id_offset_of.clear();
            #(#clear_extends);*
        }

        #(#impl_extends)*
    };

    let res = quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, ::std::hash::Hash)]
        #repr
        enum #enum_name_ident {
            #(#extends),*
        }

        type #id_ident<I, G> = (#enum_name_ident, I, G);

        #[derive(Default)]
        struct #arena_name_ident #new_generics {
            g: HIDE_G,

            enums_vec_id_offset_of: Vec<HIDE_I>,
            #(#vec_body),*
        }
        impl #new_generics #arena_name_ident #new_generics
        where HIDE_I: ::enums_arena_defines::Index,
            HIDE_G: ::enums_arena_defines::Generation,
        {
            #impl_part
        }
        impl #new_generics #arena_name_ident #new_generics
        where HIDE_I: ::enums_arena_defines::Index,
            HIDE_G: ::enums_arena_defines::Generation,
            #user_bound
        {
            #[allow(unused)]
            pub fn get_cloned(&self, id: #id_ident<HIDE_I, HIDE_G>) -> Option<#name #user_generics> {
                let (ty, index, g) = id;
                if g != self.g {
                    return None;
                }
                let ty_index = self.enums_vec_id_offset_of.get(index.to_usize())?.clone();
                match ty {
                    #(#match_body)*
                }
            }
        }

    };

    Ok(res.into())
}
